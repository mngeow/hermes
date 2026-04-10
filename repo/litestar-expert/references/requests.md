# Requests Reference

## Request Body

### Basic Body Parsing

```python
from pydantic import BaseModel

class CreateUserRequest(BaseModel):
    name: str
    email: str
    age: int

@post("/users")
async def create_user(data: CreateUserRequest) -> User:
    # data is validated and parsed
    return await save_user(data)
```

### Dataclass Body

```python
from dataclasses import dataclass

@dataclass
class UpdateProfileRequest:
    bio: str | None = None
    avatar_url: str | None = None

@patch("/profile")
async def update_profile(data: UpdateProfileRequest) -> User: ...
```

### TypedDict Body

```python
from typing import TypedDict

class ItemData(TypedDict):
    name: str
    price: float
    quantity: int

@post("/items")
async def create_item(data: ItemData) -> Item: ...
```

### Msgspec Struct

```python
import msgspec

class Product(msgspec.Struct):
    name: str
    price: float
    tags: list[str] = []

@post("/products")
async def create_product(data: Product) -> Product: ...
```

## Request Body Validation

### Body Parameter Options

```python
from litestar.params import Body

@post("/items")
async def create_item(
    data: Item = Body(
        title="Item Data",
        description="The item to create",
        examples={"simple": {"name": "Widget", "price": 9.99}},
    )
) -> Item: ...
```

### Content Type Handling

```python
from litestar.params import Body
from litestar.enums import RequestEncodingType

# URL-encoded form data
@post("/form")
async def handle_form(
    data: dict = Body(media_type=RequestEncodingType.URL_ENCODED)
) -> dict: ...

# Multipart form data (file uploads)
@post("/upload-form")
async def handle_multipart(
    data: dict = Body(media_type=RequestEncodingType.MULTI_PART)
) -> dict: ...

# MessagePack
@post("/msgpack")
async def handle_msgpack(
    data: dict = Body(media_type=RequestEncodingType.MESSAGEPACK)
) -> dict: ...
```

## File Uploads

### Single File

```python
from litestar.datastructures import UploadFile

@post("/upload")
async def upload_file(data: UploadFile) -> dict:
    content = await data.read()
    
    return {
        "filename": data.filename,
        "content_type": data.content_type,
        "size": len(content),
    }
```

### Multiple Files

```python
@post("/upload-multiple")
async def upload_files(data: list[UploadFile]) -> dict:
    results = []
    for file in data:
        content = await file.read()
        results.append({
            "filename": file.filename,
            "size": len(content),
        })
    return {"files": results}
```

### Named Files (Pydantic Model)

```python
from pydantic import BaseModel

class FileUpload(BaseModel):
    document: UploadFile
    thumbnail: UploadFile | None = None

@post("/upload-structured")
async def upload_structured(data: FileUpload) -> dict:
    doc_content = await data.document.read()
    
    thumb_content = None
    if data.thumbnail:
        thumb_content = await data.thumbnail.read()
    
    return {
        "document": data.document.filename,
        "has_thumbnail": data.thumbnail is not None,
    }
```

### Files as Dictionary

```python
@post("/upload-dict")
async def upload_dict(data: dict[str, UploadFile]) -> dict:
    results = {}
    for name, file in data.items():
        content = await file.read()
        results[name] = {
            "filename": file.filename,
            "size": len(content),
        }
    return results
```

### Synchronous File Reading

```python
@post("/upload-sync")
def upload_sync(data: UploadFile) -> dict:
    # In sync handlers, use file.file directly
    content = data.file.read()
    
    return {
        "filename": data.filename,
        "size": len(content),
    }
```

## Query Parameters

### Basic Query Parameters

```python
@get("/search")
async def search(
    q: str,
    limit: int = 10,
    offset: int = 0,
) -> list[Item]:
    return await search_items(q, limit, offset)
```

### Optional Query Parameters

```python
@get("/filter")
async def filter_items(
    category: str | None = None,
    min_price: float | None = None,
    max_price: float | None = None,
    in_stock: bool = False,
) -> list[Item]:
    query = Item.query()
    
    if category:
        query = query.filter(Item.category == category)
    if min_price:
        query = query.filter(Item.price >= min_price)
    if max_price:
        query = query.filter(Item.price <= max_price)
    if in_stock:
        query = query.filter(Item.stock > 0)
    
    return await query.all()
```

### Query Parameter Validation

```python
from litestar.params import Parameter

@get("/items")
async def list_items(
    page: int = Parameter(
        ge=1,  # Greater than or equal to 1
        default=1,
    ),
    page_size: int = Parameter(
        ge=1,
        le=100,  # Less than or equal to 100
        default=20,
    ),
    sort: str = Parameter(
        pattern="^(name|price|created_at)$",  # Regex pattern
        default="created_at",
    ),
) -> list[Item]: ...
```

### List Query Parameters

```python
@get("/items")
async def list_items(
    tags: list[str] = [],  # ?tags=python&tags=web
    ids: list[int] = [],   # ?ids=1&ids=2&ids=3
) -> list[Item]:
    query = Item.query()
    
    if tags:
        query = query.filter(Item.tags.overlap(tags))
    if ids:
        query = query.filter(Item.id.in_(ids))
    
    return await query.all()
```

## Headers

### Header Parameters

```python
from litestar.params import Parameter

@get("/user-agent")
async def user_agent(
    user_agent: str = Parameter(header="User-Agent")
) -> dict:
    return {"user_agent": user_agent}

@get("/content-negotiation")
async def content_negotiation(
    accept: str = Parameter(header="Accept", default="application/json")
) -> Response:
    if "application/json" in accept:
        return Response(content={"data": []}, media_type=MediaType.JSON)
    elif "text/html" in accept:
        return Response(content="<html>...</html>", media_type=MediaType.HTML)
    else:
        raise HTTPException(status_code=406)
```

### Custom Header Validation

```python
@post("/webhook")
async def webhook(
    signature: str = Parameter(
        header="X-Webhook-Signature",
        required=True,
    ),
    event_type: str = Parameter(
        header="X-Event-Type",
        default="unknown",
    ),
    data: dict,
) -> dict:
    if not verify_signature(data, signature):
        raise HTTPException(status_code=401, detail="Invalid signature")
    
    await process_webhook(event_type, data)
    return {"status": "processed"}
```

## Cookies

### Cookie Parameters

```python
from litestar.params import Parameter

@get("/session")
async def get_session(
    session_id: str | None = Parameter(cookie="session_id", default=None)
) -> dict:
    if not session_id:
        raise HTTPException(status_code=401)
    
    session = await get_session_data(session_id)
    return {"session": session}
```

## Request Object

### Accessing Full Request

```python
from litestar import Request

@get("/request-info")
async def request_info(request: Request) -> dict:
    return {
        "method": request.method,
        "url": str(request.url),
        "path_params": request.path_params,
        "query_params": dict(request.query_params),
        "headers": dict(request.headers),
        "client": {
            "host": request.client.host if request.client else None,
            "port": request.client.port if request.client else None,
        },
        "user": str(request.user) if request.user else None,
    }
```

### Reading Raw Body

```python
@post("/raw-body")
async def raw_body(request: Request) -> dict:
    body = await request.body()
    return {
        "size": len(body),
        "content": body.decode("utf-8"),
    }
```

### Streaming Body

```python
@post("/stream-body")
async def stream_body(request: Request) -> dict:
    total_size = 0
    async for chunk in request.stream():
        total_size += len(chunk)
        # Process chunk
    
    return {"total_size": total_size}
```

### Form Data

```python
@post("/form-data")
async def form_data(request: Request) -> dict:
    form = await request.form()
    return {
        "fields": {k: v for k, v in form.items() if isinstance(v, str)},
        "files": [k for k, v in form.items() if isinstance(v, UploadFile)],
    }
```

## Custom Request Class

```python
from litestar import Request

class CustomRequest(Request):
    @property
    def tenant_id(self) -> str | None:
        return self.headers.get("X-Tenant-ID")
    
    async def get_validated_user(self) -> User:
        token = self.headers.get("Authorization", "").replace("Bearer ", "")
        return await validate_token(token)

app = Litestar(
    route_handlers=[...],
    request_class=CustomRequest,
)

# Usage
@get("/tenant")
async def get_tenant(request: CustomRequest) -> dict:
    return {
        "tenant_id": request.tenant_id,
        "user": await request.get_validated_user(),
    }
```

## Request Limits

### Body Size Limit

```python
# Global limit
app = Litestar(
    route_handlers=[...],
    request_max_body_size=10 * 1024 * 1024,  # 10MB
)

# Per-handler limit
@post("/upload", request_max_body_size=100 * 1024 * 1024)  # 100MB
async def upload(data: UploadFile) -> dict: ...

# No limit (not recommended)
@post("/unlimited", request_max_body_size=None)
async def unlimited(data: bytes) -> dict: ...
```
