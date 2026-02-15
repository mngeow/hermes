# OpenAPI Reference

## Basic Configuration

```python
from litestar import Litestar
from litestar.openapi import OpenAPIConfig
from litestar.openapi.spec import Server

app = Litestar(
    route_handlers=[...],
    openapi_config=OpenAPIConfig(
        title="My API",
        version="1.0.0",
        description="A sample API built with Litestar",
        contact={
            "name": "API Support",
            "email": "api@example.com",
            "url": "https://example.com/support",
        },
        license={
            "name": "MIT",
            "url": "https://opensource.org/licenses/MIT",
        },
        terms_of_service="https://example.com/terms",
        servers=[
            Server(url="https://api.example.com", description="Production"),
            Server(url="https://staging-api.example.com", description="Staging"),
            Server(url="http://localhost:8000", description="Local"),
        ],
    ),
)
```

## Route Documentation

### Operation Metadata

```python
from litestar import get

@get(
    "/items",
    summary="List all items",
    description="Returns a paginated list of items",
    operation_id="listItems",
    tags=["items"],
    deprecated=False,
)
async def list_items() -> list[Item]: ...
```

### Long Descriptions

```python
@get(
    "/items/{item_id}",
    summary="Get item by ID",
    description="""
    Retrieve a specific item by its unique identifier.
    
    ## Errors
    
    - `404`: Item not found
    - `403`: Insufficient permissions
    """,
)
async def get_item(item_id: int) -> Item: ...
```

## Schema Customization

### Field Documentation

```python
from pydantic import BaseModel, Field

class Item(BaseModel):
    id: int = Field(
        description="Unique identifier",
        examples=[1, 2, 3],
    )
    name: str = Field(
        description="Item name",
        min_length=1,
        max_length=100,
    )
    price: float = Field(
        description="Price in USD",
        gt=0,
        examples=[9.99, 19.99],
    )
    tags: list[str] = Field(
        default=[],
        description="Item tags",
    )
```

### Response Schema

```python
from litestar.openapi.datastructures import ResponseSpec

@get(
    "/items",
    responses={
        200: ResponseSpec(
            data_container=list[Item],
            description="List of items",
        ),
        401: ResponseSpec(
            data_container=ErrorResponse,
            description="Unauthorized",
        ),
        500: ResponseSpec(
            data_container=ErrorResponse,
            description="Server error",
        ),
    },
)
async def list_items() -> list[Item]: ...
```

### Excluding from Schema

```python
@get("/internal/health", include_in_schema=False)
async def health_check() -> dict:
    return {"status": "ok"}

# Or at controller/router level
class InternalController(Controller):
    include_in_schema = False
    ...
```

## Authentication Documentation

### Security Schemes

```python
from litestar.openapi.spec import Components, SecurityScheme

app = Litestar(
    route_handlers=[...],
    openapi_config=OpenAPIConfig(
        title="My API",
        version="1.0.0",
        components=Components(
            security_schemes={
                "bearer": SecurityScheme(
                    type="http",
                    scheme="bearer",
                    bearer_format="JWT",
                    description="JWT token authentication",
                ),
                "apiKey": SecurityScheme(
                    type="apiKey",
                    in_="header",
                    name="X-API-Key",
                    description="API key authentication",
                ),
                "oauth2": SecurityScheme(
                    type="oauth2",
                    flows={
                        "password": {
                            "tokenUrl": "/token",
                            "scopes": {
                                "read": "Read access",
                                "write": "Write access",
                            },
                        }
                    },
                ),
            }
        ),
    ),
)
```

### Route Security

```python
@get(
    "/protected",
    security=[{"bearer": []}],
)
async def protected() -> dict: ...

@get(
    "/admin",
    security=[{"bearer": ["admin"]}],
)
async def admin_only() -> dict: ...
```

## Custom Type Schemas

### Type Examples

```python
from pydantic import BaseModel, Field
from datetime import datetime
from uuid import UUID

class Event(BaseModel):
    id: UUID = Field(examples=["550e8400-e29b-41d4-a716-446655440000"])
    timestamp: datetime = Field(examples=["2024-01-15T10:30:00Z"])
    type: str = Field(examples=["user.created", "order.placed"])
```

### Custom Schema for Existing Types

```python
from litestar.openapi.spec import Schema
from litestar.types import MessageType

class Money:
    """Custom money type"""
    def __init__(self, amount: Decimal, currency: str):
        self.amount = amount
        self.currency = currency

# Register custom schema
def money_schema(
    field: FieldInfo,
) -> Schema:
    return Schema(
        type="object",
        properties={
            "amount": {"type": "number", "format": "decimal"},
            "currency": {"type": "string", "enum": ["USD", "EUR", "GBP"]},
        },
        required=["amount", "currency"],
        examples=[{"amount": 99.99, "currency": "USD"}],
    )

# Use in model
class Product(BaseModel):
    name: str
    price: Money  # Will use custom schema
```

## UI Plugins

Litestar supports multiple OpenAPI UI plugins:

### Swagger UI (Default)

```python
from litestar.openapi.plugins import SwaggerRenderPlugin

app = Litestar(
    route_handlers=[...],
    openapi_config=OpenAPIConfig(
        title="My API",
        version="1.0.0",
        render_plugins=[SwaggerRenderPlugin()],
    ),
)

# Access at: /schema/swagger
```

### ReDoc

```python
from litestar.openapi.plugins import ReDocRenderPlugin

app = Litestar(
    route_handlers=[...],
    openapi_config=OpenAPIConfig(
        title="My API",
        version="1.0.0",
        render_plugins=[ReDocRenderPlugin()],
    ),
)

# Access at: /schema
```

### Stoplight Elements

```python
from litestar.openapi.plugins import StoplightRenderPlugin

app = Litestar(
    route_handlers=[...],
    openapi_config=OpenAPIConfig(
        title="My API",
        version="1.0.0",
        render_plugins=[StoplightRenderPlugin()],
    ),
)

# Access at: /schema/elements
```

### RapiDoc

```python
from litestar.openapi.plugins import RapiDocRenderPlugin

app = Litestar(
    route_handlers=[...],
    openapi_config=OpenAPIConfig(
        title="My API",
        version="1.0.0",
        render_plugins=[RapiDocRenderPlugin()],
    ),
)

# Access at: /schema/rapidoc
```

### Multiple UIs

```python
from litestar.openapi.plugins import (
    SwaggerRenderPlugin,
    ReDocRenderPlugin,
    StoplightRenderPlugin,
)

app = Litestar(
    route_handlers=[...],
    openapi_config=OpenAPIConfig(
        title="My API",
        version="1.0.0",
        render_plugins=[
            SwaggerRenderPlugin(path="/swagger"),
            ReDocRenderPlugin(path="/docs"),
            StoplightRenderPlugin(path="/elements"),
        ],
    ),
)
```

## Schema Generation Hooks

### on_app_init Hook

```python
from litestar.config.app import AppConfig

def customize_openapi(app_config: AppConfig) -> AppConfig:
    if app_config.openapi_config:
        app_config.openapi_config.title = "Custom Title"
    return app_config

app = Litestar(
    route_handlers=[...],
    on_app_init=[customize_openapi],
)
```

## YAML/JSON Export

```python
# The OpenAPI schema is available at:
# /schema/openapi.json - JSON format
# /schema/openapi.yaml - YAML format (requires: pip install litestar[yaml])
```

## Examples Generation

### Polyfactory Integration

```python
# Install: pip install litestar[polyfactory]

from pydantic import BaseModel
from polyfactory.factories.pydantic_factory import ModelFactory

class User(BaseModel):
    id: int
    name: str
    email: str

class UserFactory(ModelFactory[User]):
    __model__ = User

# Examples are automatically generated for OpenAPI
```

### Manual Examples

```python
from litestar.openapi.spec import Example

@get(
    "/users",
    responses={
        200: ResponseSpec(
            data_container=list[User],
            examples={
                "single": Example(
                    summary="Single user",
                    value=[{"id": 1, "name": "John", "email": "john@example.com"}],
                ),
                "multiple": Example(
                    summary="Multiple users",
                    value=[
                        {"id": 1, "name": "John", "email": "john@example.com"},
                        {"id": 2, "name": "Jane", "email": "jane@example.com"},
                    ],
                ),
            },
        ),
    },
)
async def list_users() -> list[User]: ...
```
