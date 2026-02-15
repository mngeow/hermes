---
name: litestar-expert
description: Expert-level guidance for building web applications with Litestar framework. Use when working with Litestar for creating ASGI web applications, REST APIs, WebSocket endpoints, dependency injection, DTOs, middleware, routing, authentication/authorization, OpenAPI documentation, or any Litestar-specific development tasks. Covers route handlers, controllers, routers, request/response handling, layered architecture, and advanced features like channels, caching, and background tasks.
---

# Litestar Expert

## Overview

Litestar is a powerful, flexible, highly performant ASGI framework with built-in dependency injection, OpenAPI schema generation, data validation, and more. It supports both function-based and class-based (Controller) approaches.

## Core Architecture

### Layered Architecture

Litestar uses a 4-layer architecture where parameters defined closer to the handler take precedence:

1. **Application** (`Litestar`)
2. **Router** (`Router`)
3. **Controller** (`Controller`)
4. **Handler** (`@get`, `@post`, etc.)

Parameters supporting layering: `after_request`, `after_response`, `before_request`, `cache_control`, `dependencies`, `dto`, `etag`, `exception_handlers`, `guards`, `include_in_schema`, `middleware`, `opt`, `request_class`, `response_class`, `response_cookies`, `response_headers`, `return_dto`, `security`, `tags`, `type_decoders`, `type_encoders`, `websocket_class`

## Quick Start

### Basic Application

```python
from litestar import Litestar, get

@get("/")
async def index() -> str:
    return "Hello, world!"

@get("/books/{book_id:int}")
async def get_book(book_id: int) -> dict[str, int]:
    return {"book_id": book_id}

app = Litestar([index, get_book])
```

Run with: `litestar run` or `uvicorn app:app --reload`

### Installation Options

```bash
# Standard (includes CLI, uvicorn, jinja2)
pip install litestar[standard]

# With specific extras
pip install litestar[pydantic,jwt,sqlalchemy,redis]

# All extras (not recommended for production)
pip install litestar[full]
```

## Route Handlers

### HTTP Handlers

```python
from litestar import get, post, put, patch, delete, head
from pydantic import BaseModel

class Resource(BaseModel):
    id: int
    name: str

@get("/resources")
async def list_resources() -> list[Resource]: ...

@post("/resources")
async def create_resource(data: Resource) -> Resource: ...

@get("/resources/{pk:int}")
async def get_resource(pk: int) -> Resource: ...

@put("/resources/{pk:int}")
async def update_resource(data: Resource, pk: int) -> Resource: ...

@patch("/resources/{pk:int}")
async def partial_update_resource(data: Resource, pk: int) -> Resource: ...

@delete("/resources/{pk:int}")
async def delete_resource(pk: int) -> None: ...
```

### Reserved Keyword Arguments

Handlers can receive special injected parameters:

- `request`: `Request` instance
- `state`: Application state
- `headers`: Parsed headers dict
- `cookies`: Parsed cookies dict
- `query`: Query parameters dict
- `scope`: ASGI scope dict
- `body`: Raw request body

```python
from litestar import get, Request
from litestar.datastructures import State

@get("/")
async def handler(
    request: Request,
    state: State,
    headers: dict[str, str],
    query: dict[str, Any],
    cookies: dict[str, Any],
) -> None: ...
```

### WebSocket Handlers

```python
from litestar import websocket, WebSocket

@websocket("/ws")
async def ws_handler(socket: WebSocket) -> None:
    await socket.accept()
    await socket.send_json({"message": "hello"})
    await socket.close()
```

### WebSocket Listeners (High-level)

```python
from litestar import websocket_listener

@websocket_listener("/ws")
async def ws_handler(data: str) -> str:
    return f"Echo: {data}"
```

## Controllers

Controllers organize related endpoints using OOP:

```python
from litestar import Controller, get, post, patch, delete
from litestar.dto import DTOConfig, DTOData
from pydantic import BaseModel, UUID4

class User(BaseModel):
    id: UUID4
    name: str
    email: str

class PartialUserDTO(DataclassDTO[User]):
    config = DTOConfig(exclude={"id"}, partial=True)

class UserController(Controller):
    path = "/users"
    
    @post()
    async def create_user(self, data: User) -> User: ...
    
    @get()
    async def list_users(self) -> list[User]: ...
    
    @get("/{user_id:uuid}")
    async def get_user(self, user_id: UUID4) -> User: ...
    
    @patch("/{user_id:uuid}", dto=PartialUserDTO)
    async def update_user(
        self, user_id: UUID4, data: DTOData[User]
    ) -> User: ...
    
    @delete("/{user_id:uuid}")
    async def delete_user(self, user_id: UUID4) -> None: ...

app = Litestar([UserController])
```

## Routers

Routers group controllers and handlers under a common path:

```python
from litestar import Router, Litestar

user_router = Router(
    path="/users",
    route_handlers=[UserController],
    dependencies={...},  # Router-level dependencies
    middleware=[...],    # Router-level middleware
)

api_router = Router(
    path="/api/v1",
    route_handlers=[user_router],
)

app = Litestar([api_router])
```

## Dependency Injection

### Basic Usage

```python
from litestar import get
from litestar.di import Provide

async def get_db() -> Database:
    return Database()

@get("/items", dependencies={"db": Provide(get_db)})
async def list_items(db: Database) -> list[Item]: ...
```

### Yield Dependencies (with cleanup)

```python
async def get_db_session() -> AsyncGenerator[Session, None]:
    session = Session()
    try:
        yield session
        await session.commit()
    except Exception:
        await session.rollback()
        raise
    finally:
        await session.close()
```

### Dependencies in Dependencies

```python
async def get_user(db: Database) -> User: ...

async def get_user_service(user: User) -> UserService: ...

app = Litestar(
    route_handlers=[...],
    dependencies={
        "db": Provide(get_db),
        "user": Provide(get_user),
        "service": Provide(get_user_service),
    }
)
```

## Data Transfer Objects (DTOs)

DTOs control data flow between client and handler:

```python
from litestar.dto import DTOConfig, DataclassDTO
from litestar.plugins.pydantic import PydanticDTO
from dataclasses import dataclass

@dataclass
class User:
    id: UUID
    name: str
    email: str
    password: str

# Exclude sensitive fields
class UserDTO(DataclassDTO[User]):
    config = DTOConfig(exclude={"password", "id"})

# Partial updates (PATCH)
class PartialUserDTO(DataclassDTO[User]):
    config = DTOConfig(exclude={"id"}, partial=True)

@get("/users", return_dto=UserDTO)
async def list_users() -> list[User]: ...

@patch("/{user_id:uuid}", dto=PartialUserDTO)
async def update_user(
    user_id: UUID, 
    data: DTOData[User]
) -> User: ...
```

## Request Handling

### Request Body

```python
from pydantic import BaseModel

class CreateUserRequest(BaseModel):
    name: str
    email: str

@post("/users")
async def create_user(data: CreateUserRequest) -> User: ...
```

### File Uploads

```python
from litestar.datastructures import UploadFile

@post("/upload")
async def upload_file(data: UploadFile) -> dict:
    content = await data.read()
    return {"filename": data.filename, "size": len(content)}

# Multiple files
@post("/upload-multiple")
async def upload_files(data: list[UploadFile]) -> dict: ...
```

### Form Data

```python
from litestar.params import Body
from litestar.enums import RequestEncodingType

@post("/form")
async def handle_form(
    data: dict = Body(media_type=RequestEncodingType.URL_ENCODED)
) -> dict: ...

# Multipart
@post("/multipart")
async def handle_multipart(
    data: dict = Body(media_type=RequestEncodingType.MULTI_PART)
) -> dict: ...
```

## Response Handling

### Basic Responses

```python
from litestar import Response, MediaType
from litestar.response import Redirect, File, Stream, Template

# JSON (default)
@get("/json")
async def json_response() -> dict: ...

# Text
@get("/text", media_type=MediaType.TEXT)
async def text_response() -> str: ...

# HTML
@get("/html", media_type=MediaType.HTML)
async def html_response() -> str: ...

# Custom Response
@get("/custom")
async def custom_response() -> Response[User]:
    return Response(
        content=User(...),
        status_code=201,
        headers={"X-Custom": "value"}
    )
```

### Response Types

```python
from litestar.response import Redirect, File, Stream, Template, ServerSentEvent
from pathlib import Path

# Redirect
@get("/redirect")
async def redirect() -> Redirect:
    return Redirect(path="/new-location")

# File
@get("/download")
async def download() -> File:
    return File(
        path=Path("report.pdf"),
        filename="report.pdf"
    )

# Stream
@get("/stream")
async def stream() -> Stream:
    async def content_generator():
        for i in range(10):
            yield f"chunk {i}\n"
    
    return Stream(content_generator())

# Server-Sent Events
@get("/events")
async def events() -> ServerSentEvent:
    async def event_generator():
        while True:
            yield {"event_type": "update", "data": {"time": time.time()}}
    
    return ServerSentEvent(event_generator())
```

## Middleware

### Built-in Middleware

```python
from litestar.middleware import (
    LoggingMiddleware,
    CompressionMiddleware,
    RateLimitMiddleware,
    CSRFMiddleware,
    SessionMiddleware,
    JWTAuthMiddleware,
)

app = Litestar(
    route_handlers=[...],
    middleware=[
        LoggingMiddleware(),
        CompressionMiddleware(),
        RateLimitMiddleware(rate_limit=("minute", 100)),
    ]
)
```

### Custom Middleware

```python
from litestar.middleware import AbstractMiddleware
from litestar.types import ASGIApp, Scope, Receive, Send

class CustomMiddleware(AbstractMiddleware):
    async def __call__(self, scope: Scope, receive: Receive, send: Send) -> None:
        # Pre-processing
        await self.app(scope, receive, send)
        # Post-processing
```

## Security

### Guards (Authorization)

```python
from litestar import get
from litestar.connection import ASGIConnection
from litestar.handlers.base import BaseRouteHandler
from litestar.exceptions import NotAuthorizedException

async def admin_guard(
    connection: ASGIConnection,
    handler: BaseRouteHandler
) -> None:
    if not connection.user.is_admin:
        raise NotAuthorizedException()

@get("/admin", guards=[admin_guard])
async def admin_endpoint() -> dict: ...
```

### JWT Authentication

```python
from litestar.security.jwt import JWTAuth, Token

jwt_auth = JWTAuth(
    token_secret="your-secret",
    retrieve_user_handler=get_user_by_id,
    token_cls=Token,
)

app = Litestar(
    route_handlers=[...],
    on_app_init=[jwt_auth.on_app_init],
)

# Protected endpoint
@get("/protected", middleware=[jwt_auth.middleware])
async def protected(request: Request) -> dict: ...
```

## Application State

```python
from litestar.datastructures import State

# Initialize state
app = Litestar(
    route_handlers=[...],
    state=State({"app_name": "My App", "version": "1.0.0"})
)

# Access in handler
@get("/info")
async def info(state: State) -> dict:
    return {"app": state.app_name, "version": state.version}
```

## Lifecycle Hooks

```python
# Startup/Shutdown
async def connect_db():
    db.connect()

async def disconnect_db():
    db.disconnect()

app = Litestar(
    route_handlers=[...],
    on_startup=[connect_db],
    on_shutdown=[disconnect_db],
)

# Lifespan context manager
from contextlib import asynccontextmanager

@asynccontextmanager
async def lifespan(app: Litestar):
    await connect_db()
    yield
    await disconnect_db()

app = Litestar(route_handlers=[...], lifespan=[lifespan])
```

## OpenAPI

Litestar auto-generates OpenAPI 3.1.0 documentation:

```python
from litestar.openapi import OpenAPIConfig

app = Litestar(
    route_handlers=[...],
    openapi_config=OpenAPIConfig(
        title="My API",
        version="1.0.0",
        description="API documentation",
    ),
)
```

Access at:
- `/schema` - ReDoc
- `/schema/swagger` - Swagger UI
- `/schema/elements` - Stoplight Elements
- `/schema/rapidoc` - RapiDoc

## Background Tasks

```python
from litestar.background_tasks import BackgroundTask, BackgroundTasks

async def send_email(email: str, message: str) -> None: ...

@post("/notify")
async def notify(email: str) -> dict:
    task = BackgroundTask(send_email, email, message="Hello!")
    return {"status": "queued"}, task

# Multiple tasks
@post("/notify-all")
async def notify_all(emails: list[str]) -> dict:
    tasks = BackgroundTasks([
        BackgroundTask(send_email, email, "Hello!")
        for email in emails
    ])
    return {"status": "queued"}, tasks
```

## Testing

```python
from litestar.testing import TestClient

def test_endpoint():
    with TestClient(app=app) as client:
        response = client.get("/")
        assert response.status_code == 200
        assert response.json() == {"message": "hello"}
```

## References

For detailed information on specific topics, see:

- [references/routing.md](references/routing.md) - Routing patterns and path parameters
- [references/dto.md](references/dto.md) - DTO configuration and advanced usage
- [references/middleware.md](references/middleware.md) - Middleware patterns and built-in options
- [references/security.md](references/security.md) - Authentication and authorization patterns
- [references/websockets.md](references/websockets.md) - WebSocket handling patterns
- [references/responses.md](references/responses.md) - Response types and customization
- [references/requests.md](references/requests.md) - Request handling patterns
- [references/dependencies.md](references/dependencies.md) - Dependency injection patterns
- [references/openapi.md](references/openapi.md) - OpenAPI customization
