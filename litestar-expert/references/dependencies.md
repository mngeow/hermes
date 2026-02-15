# Dependency Injection Reference

## Basic Dependencies

### Simple Dependency

```python
from litestar import get
from litestar.di import Provide

async def get_database() -> Database:
    return Database()

@get("/items", dependencies={"db": Provide(get_database)})
async def list_items(db: Database) -> list[Item]:
    return await db.query(Item).all()
```

### Sync Dependencies

```python
def get_config() -> Config:
    return Config.load()

@get("/config", dependencies={"config": Provide(get_config)})
async def get_settings(config: Config) -> dict:
    return config.to_dict()
```

## Dependency Scopes

Dependencies are scoped to where they're defined:

```python
from litestar import Controller, Router, Litestar, get
from litestar.di import Provide

# App-level: Available to all handlers
app = Litestar(
    route_handlers=[...],
    dependencies={"app_dep": Provide(app_dependency)},
)

# Router-level: Available to handlers in this router
router = Router(
    path="/api",
    route_handlers=[...],
    dependencies={"router_dep": Provide(router_dependency)},
)

# Controller-level: Available to handlers in this controller
class MyController(Controller):
    dependencies = {"controller_dep": Provide(controller_dependency)}
    
    @get("/")
    async def handler(self, controller_dep: DepType) -> dict: ...

# Handler-level: Only this handler
@get("/special", dependencies={"local_dep": Provide(local_dependency)})
async def special_handler(local_dep: DepType) -> dict: ...
```

## Dependencies with Cleanup (Yield)

```python
from typing import AsyncGenerator

async def get_db_session() -> AsyncGenerator[Session, None]:
    """Dependency with automatic cleanup"""
    session = Session()
    try:
        yield session
        await session.commit()
    except Exception:
        await session.rollback()
        raise
    finally:
        await session.close()

@post("/items", dependencies={"session": Provide(get_db_session)})
async def create_item(data: ItemCreate, session: Session) -> Item:
    item = Item(**data.dict())
    session.add(item)
    # Automatically committed on success
    return item
```

### Exception Handling in Yield Dependencies

```python
async def get_resource() -> AsyncGenerator[Resource, None]:
    resource = await acquire_resource()
    try:
        yield resource
    except Exception as e:
        # Handle exception in dependency
        await log_error(resource, e)
        raise
    finally:
        await release_resource(resource)
```

## Dependencies within Dependencies

```python
async def get_database() -> Database:
    return Database()

async def get_repository(
    db: Database,  # Injected from outer scope
) -> Repository:
    return Repository(db)

async def get_service(
    repo: Repository,  # Injected from outer scope
) -> Service:
    return Service(repo)

app = Litestar(
    route_handlers=[...],
    dependencies={
        "db": Provide(get_database),
        "repo": Provide(get_repository),
        "service": Provide(get_service),
    }
)

@get("/items")
async def list_items(service: Service) -> list[Item]:
    return await service.get_all()
```

## Dependency Overrides

Lower-scoped dependencies override higher-scoped ones:

```python
class UserController(Controller):
    path = "/users"
    # Controller-level dependency
    dependencies = {"db": Provide(read_replica_db)}
    
    @get("/")
    async def list(self, db: Database) -> list[User]:
        # Uses read_replica_db
        return await db.query(User).all()
    
    @post("/", dependencies={"db": Provide(primary_db)})
    async def create(self, data: UserCreate, db: Database) -> User:
        # Uses primary_db (handler-level override)
        return await db.insert(User, data)
```

## Dependency Validation

### Skipping Validation

```python
from litestar.params import Dependency

@get("/items")
async def list_items(
    db: Database = Dependency(skip_validation=True)
) -> list[Item]:
    # Validation skipped for this dependency
    return await db.query(Item).all()
```

### Dependency Marker

```python
from litestar.params import Dependency

@get("/items")
async def list_items(
    db: Database = Dependency(),
    optional_param: str | None = Dependency(default=None),
) -> list[Item]:
    # Explicitly marks parameters as dependencies
    # Excludes them from OpenAPI docs
    return await db.query(Item).all()
```

## Reserved Keyword Arguments

Dependencies can receive special injected parameters:

```python
from litestar import Request, State

async def get_current_user(
    request: Request,
    state: State,
) -> User:
    """Access request and state in dependency"""
    token = request.headers.get("Authorization", "").replace("Bearer ", "")
    user = await verify_token(token)
    
    # Cache in state
    state.setdefault("user_cache", {})[token] = user
    return user

@get("/profile", dependencies={"user": Provide(get_current_user)})
async def profile(user: User) -> dict:
    return {"name": user.name, "email": user.email}
```

## Dependency Caching

```python
from litestar.di import Provide

async def expensive_initialization() -> Service:
    """This will be called once and cached"""
    return await Service.create()

app = Litestar(
    route_handlers=[...],
    dependencies={
        "service": Provide(expensive_initialization, use_cache=True)
    }
)
```

**Note**: `use_cache` caches the result per-request, not globally.

## Class-Based Dependencies

```python
from litestar.di import Provide

class DatabaseConnection:
    def __init__(self, connection_string: str):
        self.connection_string = connection_string
    
    async def connect(self):
        self.db = await create_connection(self.connection_string)
        return self.db
    
    async def disconnect(self):
        await self.db.close()

# Factory function
async def get_db() -> AsyncGenerator[Database, None]:
    conn = DatabaseConnection("postgresql://...")
    try:
        yield await conn.connect()
    finally:
        await conn.disconnect()

app = Litestar(
    route_handlers=[...],
    dependencies={"db": Provide(get_db)}
)
```

## Conditional Dependencies

```python
async def get_database(request: Request) -> Database:
    """Choose database based on request"""
    tenant_id = request.headers.get("X-Tenant-ID")
    
    if tenant_id:
        return await get_tenant_database(tenant_id)
    
    return await get_default_database()

@get("/items", dependencies={"db": Provide(get_database)})
async def list_items(db: Database) -> list[Item]:
    return await db.query(Item).all()
```

## Testing with Dependencies

### Dependency Overrides in Tests

```python
from litestar.testing import TestClient

async def mock_database() -> Database:
    return MockDatabase()

app = Litestar(
    route_handlers=[...],
    dependencies={"db": Provide(mock_database)}
)

def test_with_mock():
    with TestClient(app=app) as client:
        response = client.get("/items")
        assert response.status_code == 200
```

### Dependency Factories

```python
def create_mock_dependency(mock_value):
    async def mock_dependency():
        return mock_value
    return mock_dependency

# In test
app.dependencies["service"] = Provide(
    create_mock_dependency(MockService())
)
```
