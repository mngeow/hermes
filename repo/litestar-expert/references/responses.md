# Responses Reference

## Response Types

### Basic Response

```python
from litestar import Response

@get("/custom")
async def custom_response() -> Response[dict]:
    return Response(
        content={"message": "Hello"},
        status_code=201,
        headers={"X-Custom": "value"},
        cookies=[Cookie(key="session", value="abc123")],
        media_type=MediaType.JSON,
    )
```

### Redirect

```python
from litestar.response import Redirect
from litestar.status_codes import HTTP_302_FOUND

@get("/old-path")
async def redirect() -> Redirect:
    return Redirect(
        path="/new-path",
        status_code=HTTP_302_FOUND,
        # Or use permanent redirect
        # status_code=HTTP_301_MOVED_PERMANENTLY
    )

# External redirect
@get("/external")
async def external_redirect() -> Redirect:
    return Redirect(path="https://example.com")
```

### File Response

```python
from litestar.response import File
from pathlib import Path

@get("/download")
async def download() -> File:
    return File(
        path=Path("/path/to/file.pdf"),
        filename="report.pdf",  # Download name
        media_type="application/pdf",
        # Optional: force download
        content_disposition_type="attachment",
    )

# With custom file system (e.g., S3)
from litestar.file_system import BaseLocalFileSystem

@get("/s3-file")
async def s3_file() -> File:
    return File(
        path="s3://bucket/file.pdf",
        file_system=s3_file_system,
    )
```

### Streaming Response

```python
from litestar.response import Stream

@get("/stream")
async def stream_response() -> Stream:
    async def content_generator():
        for i in range(100):
            yield f"Chunk {i}\n"
            await asyncio.sleep(0.1)
    
    return Stream(
        content=content_generator(),
        media_type=MediaType.TEXT,
    )

# With custom iterator class
class Counter:
    def __init__(self, limit: int):
        self.limit = limit
        self.current = 0
    
    def __aiter__(self):
        return self
    
    async def __anext__(self):
        if self.current >= self.limit:
            raise StopAsyncIteration
        await asyncio.sleep(0.1)
        self.current += 1
        return f"Count: {self.current}\n"

@get("/counter")
async def counter_stream() -> Stream:
    return Stream(Counter(10))
```

### Template Response

```python
from litestar.response import Template

@get("/page")
async def page(request: Request) -> Template:
    return Template(
        template_name="page.html",
        context={
            "user": request.user,
            "title": "My Page",
        }
    )
```

### Server-Sent Events (SSE)

```python
from litestar.response import ServerSentEvent, ServerSentEventMessage

@get("/events")
async def events() -> ServerSentEvent:
    async def event_generator():
        # Simple data
        yield {"message": "Hello"}
        
        # With event type
        yield ServerSentEventMessage(
            event_type="update",
            data={"progress": 50},
            id="event-123",
            retry_duration=5000,  # Reconnect after 5s
        )
        
        # Comment (ping)
        yield ServerSentEventMessage(comment="ping")
    
    return ServerSentEvent(event_generator())
```

## Response Headers

### Static Headers

```python
from litestar.datastructures import ResponseHeader

@get(
    "/",
    response_headers=[
        ResponseHeader(name="X-Rate-Limit", value="100"),
        ResponseHeader(
            name="X-Custom",
            value="static",
            description="Custom header"
        ),
    ]
)
async def with_headers() -> dict: ...
```

### Dynamic Headers

```python
from litestar import Response

@get("/dynamic-headers")
async def dynamic_headers() -> Response[dict]:
    return Response(
        content={"status": "ok"},
        headers={
            "X-Request-ID": str(uuid4()),
            "X-Response-Time": f"{calculate_time()}ms",
        }
    )
```

### After Request Hook

```python
def add_headers(response: Response) -> Response:
    response.headers["X-Processed-By"] = "my-app"
    return response

@get("/", after_request=add_headers)
async def with_after_request() -> dict: ...
```

## Response Cookies

### Static Cookies

```python
from litestar.datastructures import Cookie

@get(
    "/",
    response_cookies=[
        Cookie(
            key="session",
            value="abc123",
            max_age=3600,
            httponly=True,
            secure=True,
            samesite="lax",
        ),
    ]
)
async def with_cookies() -> dict: ...
```

### Dynamic Cookies

```python
from litestar import Response
from litestar.datastructures import Cookie

@get("/login")
async def login() -> Response[dict]:
    session_id = create_session()
    
    return Response(
        content={"status": "logged in"},
        cookies=[
            Cookie(
                key="session",
                value=session_id,
                max_age=3600,
                httponly=True,
                secure=True,
            ),
        ]
    )
```

### Deleting Cookies

```python
@get("/logout")
async def logout() -> Response[dict]:
    return Response(
        content={"status": "logged out"},
        cookies=[
            Cookie(
                key="session",
                value="",  # Empty value
                max_age=0,  # Expire immediately
            ),
        ]
    )
```

## Status Codes

```python
from litestar.status_codes import (
    HTTP_200_OK,
    HTTP_201_CREATED,
    HTTP_204_NO_CONTENT,
    HTTP_400_BAD_REQUEST,
    HTTP_401_UNAUTHORIZED,
    HTTP_403_FORBIDDEN,
    HTTP_404_NOT_FOUND,
    HTTP_422_UNPROCESSABLE_ENTITY,
    HTTP_500_INTERNAL_SERVER_ERROR,
)

@post("/resource", status_code=HTTP_201_CREATED)
async def create() -> Resource: ...

@delete("/resource", status_code=HTTP_204_NO_CONTENT)
async def delete() -> None: ...
```

## Content Negotiation

```python
from litestar import Request

@get("/resource")
async def negotiable(request: Request) -> Response:
    data = {"message": "Hello"}
    
    if request.accepts(MediaType.JSON):
        return Response(content=data, media_type=MediaType.JSON)
    elif request.accepts(MediaType.TEXT):
        return Response(content=str(data), media_type=MediaType.TEXT)
    elif request.accepts(MediaType.HTML):
        html = f"<html><body>{data['message']}</body></html>"
        return Response(content=html, media_type=MediaType.HTML)
    else:
        raise HTTPException(
            status_code=HTTP_406_NOT_ACCEPTABLE,
            detail="Unsupported media type"
        )
```

## Custom Response Classes

```python
from litestar import Response
from typing import TypeVar, Generic

T = TypeVar("T")

class ApiResponse(Response[T]):
    """Standardized API response format"""
    
    def __init__(
        self,
        data: T,
        message: str = "success",
        meta: dict | None = None,
        **kwargs
    ):
        content = {
            "data": data,
            "message": message,
            "meta": meta or {},
        }
        super().__init__(content=content, **kwargs)

@get("/users")
async def list_users() -> ApiResponse[list[User]]:
    users = await get_users()
    return ApiResponse(
        data=users,
        meta={"count": len(users)}
    )
```

## Background Tasks

```python
from litestar.background_tasks import BackgroundTask, BackgroundTasks

async def send_email(email: str, subject: str) -> None:
    # Send email asynchronously
    ...

@post("/notify")
async def notify(email: str) -> tuple[dict, BackgroundTask]:
    task = BackgroundTask(
        send_email,
        email=email,
        subject="Notification"
    )
    return {"status": "queued"}, task

# Multiple tasks
@post("/notify-all")
async def notify_all(emails: list[str]) -> tuple[dict, BackgroundTasks]:
    tasks = BackgroundTasks([
        BackgroundTask(send_email, email=e, subject="Hello")
        for e in emails
    ])
    return {"status": "queued"}, tasks
```

## Pagination

```python
from litestar.pagination import (
    ClassicPagination,
    OffsetPagination,
    CursorPagination,
    AbstractSyncClassicPaginator,
)

# Classic pagination (page-based)
class UserPaginator(AbstractSyncClassicPaginator[User]):
    def __init__(self, query: Query):
        self.query = query
    
    async def get_items(self, page_size: int, current_page: int) -> list[User]:
        offset = (current_page - 1) * page_size
        return await self.query.limit(page_size).offset(offset).all()
    
    async def get_total(self) -> int:
        return await self.query.count()

@get("/users")
async def list_users(
    page: int = 1,
    page_size: int = 20,
) -> ClassicPagination[User]:
    paginator = UserPaginator(User.query())
    return await paginator(page_size=page_size, current_page=page)

# Offset pagination
@get("/items")
async def list_items(
    limit: int = 20,
    offset: int = 0,
) -> OffsetPagination[Item]:
    items = await Item.query().limit(limit).offset(offset).all()
    total = await Item.query().count()
    
    return OffsetPagination(
        items=items,
        total=total,
        limit=limit,
        offset=offset,
    )

# Cursor pagination
@get("/feed")
async def get_feed(
    cursor: str | None = None,
    results_per_page: int = 20,
) -> CursorPagination[Post]:
    query = Post.query().order_by(Post.created_at.desc())
    
    if cursor:
        query = query.filter(Post.id < cursor)
    
    items = await query.limit(results_per_page + 1).all()
    
    next_cursor = None
    if len(items) > results_per_page:
        next_cursor = str(items[-1].id)
        items = items[:-1]
    
    return CursorPagination(
        items=items,
        results_per_page=results_per_page,
        cursor=next_cursor,
    )
```
