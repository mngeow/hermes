# Middleware Reference

## Using Middleware

Middleware can be applied at any layer:

```python
from litestar import Litestar, Router, Controller, get
from litestar.middleware import LoggingMiddleware

# App level
app = Litestar(
    route_handlers=[...],
    middleware=[LoggingMiddleware()]
)

# Router level
router = Router(
    path="/api",
    route_handlers=[...],
    middleware=[LoggingMiddleware()]
)

# Controller level
class MyController(Controller):
    middleware = [LoggingMiddleware()]

# Handler level
@get("/", middleware=[LoggingMiddleware()])
async def handler() -> dict: ...
```

## Built-in Middleware

### Logging Middleware

```python
from litestar.middleware.logging import LoggingMiddleware

logging_middleware = LoggingMiddleware(
    request_log_fields=["method", "path", "query_params"],
    response_log_fields=["status_code", "duration"],
    logger="litestar.middleware",
)
```

### Compression Middleware

```python
from litestar.middleware.compression import CompressionMiddleware

compression_middleware = CompressionMiddleware(
    algorithm="gzip",  # or "brotli"
    minimum_size=500,  # Only compress responses > 500 bytes
    compress_level=6,
)

# Requires: pip install litestar[brotli] for brotli support
```

### Rate Limiting Middleware

```python
from litestar.middleware.rate_limit import RateLimitMiddleware, RateLimitConfig

rate_limit_middleware = RateLimitMiddleware(
    config=RateLimitConfig(
        rate_limit=("minute", 100),  # 100 requests per minute
        exclude=["/health", "/metrics"],  # Exclude paths
        # Key function to identify clients
        key_function=lambda request: request.client.host,
    )
)
```

### CSRF Middleware

```python
from litestar.middleware.csrf import CSRFMiddleware

csrf_middleware = CSRFMiddleware(
    secret="your-secret-key",
    cookie_name="csrftoken",
    header_name="x-csrftoken",
    safe_methods={"GET", "HEAD", "OPTIONS", "TRACE"},
)
```

### Session Middleware

```python
from litestar.middleware.session import SessionMiddleware
from litestar.middleware.session.server_side import ServerSideSessionBackend

session_middleware = SessionMiddleware(
    backend=ServerSideSessionBackend(),
    session_config={
        "secret_key": "your-secret",
        "max_age": 3600,
    }
)

# Client-side sessions (encrypted cookies)
from litestar.middleware.session.client_side import ClientSideSessionBackend

client_session = SessionMiddleware(
    backend=ClientSideSessionBackend(),
    session_config={
        "secret_key": "your-secret",
        "max_age": 3600,
    }
)
# Requires: pip install litestar[cryptography]
```

### CORS Middleware

```python
from litestar.middleware.cors import CORSMiddleware

cors_middleware = CORSMiddleware(
    allow_origins=["https://example.com", "https://app.example.com"],
    allow_methods=["GET", "POST", "PUT", "DELETE"],
    allow_headers=["Content-Type", "Authorization"],
    allow_credentials=True,
    max_age=600,
)
```

### Trusted Hosts Middleware

```python
from litestar.middleware.trusted_hosts import TrustedHostsMiddleware

trusted_hosts = TrustedHostsMiddleware(
    allowed_hosts=["example.com", "*.example.com"],
    www_redirect=True,  # Redirect www to non-www
)
```

### GZip Middleware

```python
from litestar.middleware.compression import CompressionMiddleware

gzip_middleware = CompressionMiddleware(
    algorithm="gzip",
    minimum_size=500,
    compress_level=6,
)
```

## JWT Authentication Middleware

```python
from litestar.security.jwt import JWTAuth

jwt_auth = JWTAuth(
    token_secret="your-secret-key",
    retrieve_user_handler=get_user_from_token,
    token_cls=Token,
    exclude=["/login", "/register", "/health"],
)

app = Litestar(
    route_handlers=[...],
    on_app_init=[jwt_auth.on_app_init],
)
```

## Custom Middleware

### Basic Custom Middleware

```python
from litestar.middleware import AbstractMiddleware
from litestar.types import ASGIApp, Scope, Receive, Send

class TimingMiddleware(AbstractMiddleware):
    """Add timing header to responses"""
    
    async def __call__(self, scope: Scope, receive: Receive, send: Send) -> None:
        if scope["type"] != "http":
            await self.app(scope, receive, send)
            return
        
        start_time = time.time()
        
        async def wrapped_send(message):
            if message["type"] == "http.response.start":
                duration = time.time() - start_time
                headers = list(message.get("headers", []))
                headers.append(
                    (b"X-Response-Time", f"{duration:.3f}s".encode())
                )
                message["headers"] = headers
            await send(message)
        
        await self.app(scope, receive, wrapped_send)
```

### Middleware with Configuration

```python
from dataclasses import dataclass
from litestar.middleware import AbstractMiddleware

@dataclass
class RequestIDConfig:
    header_name: str = "X-Request-ID"
    generator: callable = uuid4

class RequestIDMiddleware(AbstractMiddleware):
    def __init__(self, app: ASGIApp, config: RequestIDConfig = None):
        super().__init__(app)
        self.config = config or RequestIDConfig()
    
    async def __call__(self, scope: Scope, receive: Receive, send: Send) -> None:
        if scope["type"] != "http":
            await self.app(scope, receive, send)
            return
        
        request_id = str(self.config.generator())
        scope["request_id"] = request_id
        
        async def wrapped_send(message):
            if message["type"] == "http.response.start":
                headers = list(message.get("headers", []))
                headers.append(
                    (self.config.header_name.encode(), request_id.encode())
                )
                message["headers"] = headers
            await send(message)
        
        await self.app(scope, receive, wrapped_send)

# Usage
app = Litestar(
    route_handlers=[...],
    middleware=[
        RequestIDMiddleware(config=RequestIDConfig(header_name="X-Trace-ID"))
    ]
)
```

### Middleware with Request/Response Access

```python
from litestar import Request
from litestar.middleware import AbstractMiddleware

class AuditMiddleware(AbstractMiddleware):
    async def __call__(self, scope: Scope, receive: Receive, send: Send) -> None:
        if scope["type"] != "http":
            await self.app(scope, receive, send)
            return
        
        request = Request(scope, receive)
        
        # Log request
        await self.log_request(request)
        
        # Capture response
        response_body = []
        
        async def wrapped_send(message):
            if message["type"] == "http.response.body":
                response_body.append(message.get("body", b""))
            await send(message)
        
        await self.app(scope, receive, wrapped_send)
        
        # Log response
        await self.log_response(request, b"".join(response_body))
    
    async def log_request(self, request: Request):
        logger.info(f"{request.method} {request.url}")
    
    async def log_response(self, request: Request, body: bytes):
        logger.info(f"Response for {request.url}: {len(body)} bytes")
```

## Middleware Execution Order

Middleware executes in the order defined, wrapping each subsequent layer:

```python
app = Litestar(
    route_handlers=[...],
    middleware=[
        MiddlewareA(),  # Outermost - first to receive request, last to send response
        MiddlewareB(),  # Middle layer
        MiddlewareC(),  # Innermost - closest to handler
    ]
)

# Execution flow:
# Request: A → B → C → Handler
# Response: Handler → C → B → A
```

## Excluding Middleware

Some middleware supports exclusion patterns:

```python
from litestar.middleware.rate_limit import RateLimitMiddleware

middleware = RateLimitMiddleware(
    config=RateLimitConfig(
        rate_limit=("minute", 100),
        exclude=[
            "/health",           # Exact path
            "/static/*",         # Wildcard
            re.compile(r"^/api/v\d+/public/"),  # Regex
        ]
    )
)
```

## Conditional Middleware

```python
from litestar.middleware import AbstractMiddleware

class EnvironmentMiddleware(AbstractMiddleware):
    """Only applies in specific environments"""
    
    def __init__(self, app: ASGIApp, environment: str = "production"):
        super().__init__(app)
        self.environment = environment
        self.enabled = environment == "production"
    
    async def __call__(self, scope: Scope, receive: Receive, send: Send) -> None:
        if not self.enabled or scope["type"] != "http":
            await self.app(scope, receive, send)
            return
        
        # Middleware logic here
        ...
```
