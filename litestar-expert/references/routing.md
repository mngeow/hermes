# Routing Reference

## Path Parameters

Litestar supports typed path parameters using the syntax `{name:type}`:

```python
from litestar import get
from uuid import UUID
from datetime import datetime, date

@get("/users/{user_id:int}")
async def get_user(user_id: int) -> dict: ...

@get("/orders/{order_id:uuid}")
async def get_order(order_id: UUID) -> dict: ...

@get("/events/{event_date:date}")
async def get_events(event_date: date) -> dict: ...

@get("/files/{path:path}")
async def get_file(path: Path) -> File: ...
```

### Supported Types

- `str` - String (default if no type specified)
- `int` - Integer
- `float` - Float
- `uuid` - UUID
- `date` - Date (YYYY-MM-DD)
- `datetime` - DateTime (ISO format)
- `time` - Time
- `timedelta` - Time delta
- `path` - Path segments (can include slashes)

### Optional Path Parameters

```python
@get(["/items", "/items/{item_id:int}"])
async def get_items(item_id: int = 1) -> dict: ...
```

## Alternative Names and Constraints

```python
from litestar.params import Parameter

@get("/items")
async def list_items(
    sort_by: str = Parameter(query="sort", default="id")
) -> list: ...

@get("/search")
async def search(
    q: str = Parameter(min_length=3, max_length=100)
) -> list: ...
```

## Route Naming and Reverse URLs

```python
from litestar import get, Request
from litestar.response import Redirect

@get("/users/{user_id:int}", name="get_user")
async def get_user(user_id: int) -> dict: ...

@get("/redirect/{user_id:int}")
async def redirect_to_user(request: Request, user_id: int) -> Redirect:
    path = request.app.route_reverse("get_user", user_id=user_id)
    return Redirect(path=path)

# Or get absolute URL
@get("/user-url/{user_id:int}")
async def get_user_url(request: Request, user_id: int) -> str:
    url = request.url_for("get_user", user_id=user_id)
    return str(url)
```

## Handler Index

```python
@get("/", name="index")
async def index() -> dict: ...

@get("/handler-info/{name:str}")
async def handler_info(request: Request, name: str) -> dict:
    index = request.app.get_handler_index_by_name(name)
    if index:
        return {
            "paths": index["paths"],
            "handler": str(index["handler"]),
            "qualname": index["qualname"]
        }
    return {"error": "Handler not found"}
```

## Mounting ASGI Apps

```python
from litestar import asgi
from litestar.types import Scope, Receive, Send

@asgi("/sub-path")
async def mounted_app(scope: Scope, receive: Receive, send: Send) -> None:
    # Handle all requests under /sub-path/*
    ...

# Mounting external ASGI apps (e.g., Starlette)
from starlette.applications import Starlette

starlette_app = Starlette()

@asgi("/starlette")
async def starlette_mount(scope: Scope, receive: Receive, send: Send) -> None:
    await starlette_app(scope, receive, send)
```

## Dynamic Route Registration

```python
from litestar import Litestar, get

app = Litestar([])

@get("/dynamic")
async def dynamic_handler() -> dict:
    @get("/new-route")
    async def new_handler() -> str:
        return "Dynamically registered!"
    
    app.register(new_handler)
    return {"status": "registered"}
```

## Multiple Paths per Handler

```python
@get(["/api/v1/users", "/api/v2/users"])
async def list_users_v1_v2() -> list: ...
```

## Router Options

```python
from litestar import Router

router = Router(
    path="/api",
    route_handlers=[...],
    dependencies={...},
    middleware=[...],
    guards=[...],
    exception_handlers={...},
    response_class=CustomResponse,
    response_headers=[...],
    response_cookies=[...],
    tags=["api"],
    security=[...],
    opt={"custom_key": "value"},
)
```
