# Security Reference

## Guards (Authorization)

Guards control access to endpoints based on custom logic.

### Basic Guard

```python
from litestar import get
from litestar.connection import ASGIConnection
from litestar.handlers.base import BaseRouteHandler
from litestar.exceptions import NotAuthorizedException

async def authenticated_guard(
    connection: ASGIConnection,
    handler: BaseRouteHandler
) -> None:
    if not connection.user or connection.user.is_anonymous:
        raise NotAuthorizedException("Authentication required")

@get("/profile", guards=[authenticated_guard])
async def profile() -> dict: ...
```

### Role-Based Guard

```python
from typing import Sequence

def require_roles(roles: Sequence[str]):
    """Factory for role-based guards"""
    
    async def role_guard(
        connection: ASGIConnection,
        handler: BaseRouteHandler
    ) -> None:
        if not connection.user:
            raise NotAuthorizedException()
        
        user_roles = getattr(connection.user, "roles", [])
        if not any(role in user_roles for role in roles):
            raise NotAuthorizedException(
                f"Required roles: {', '.join(roles)}"
            )
    
    return role_guard

# Usage
@get("/admin", guards=[require_roles(["admin"])])
async def admin_panel() -> dict: ...

@get("/moderator", guards=[require_roles(["admin", "moderator"])])
async def moderator_panel() -> dict: ...
```

### Permission-Based Guard

```python
def require_permissions(permissions: Sequence[str]):
    """Factory for permission-based guards"""
    
    async def permission_guard(
        connection: ASGIConnection,
        handler: BaseRouteHandler
    ) -> None:
        if not connection.user:
            raise NotAuthorizedException()
        
        user_perms = getattr(connection.user, "permissions", [])
        missing = [p for p in permissions if p not in user_perms]
        
        if missing:
            raise NotAuthorizedException(
                f"Missing permissions: {', '.join(missing)}"
            )
    
    return permission_guard

@post("/posts", guards=[require_permissions(["posts.create"])])
async def create_post(data: PostCreate) -> Post: ...
```

### Resource Ownership Guard

```python
async def owner_guard(
    connection: ASGIConnection,
    handler: BaseRouteHandler
) -> None:
    """Ensure user owns the resource"""
    resource_id = connection.path_params.get("resource_id")
    resource = await get_resource(resource_id)
    
    if resource.owner_id != connection.user.id:
        raise NotAuthorizedException("Not the owner")
    
    # Store resource for handler use
    connection.scope["resource"] = resource

@get("/resources/{resource_id:int}", guards=[owner_guard])
async def get_resource_handler(resource_id: int) -> dict:
    return connection.scope["resource"]
```

## JWT Authentication

### Basic JWT Setup

```python
from litestar.security.jwt import JWTAuth, Token
from litestar.connection import ASGIConnection

async def retrieve_user_handler(token: Token, connection: ASGIConnection) -> User | None:
    """Retrieve user from token subject"""
    return await get_user_by_id(token.sub)

jwt_auth = JWTAuth(
    token_secret="your-secret-key-min-32-characters",
    retrieve_user_handler=retrieve_user_handler,
    token_cls=Token,
    exclude=["/login", "/register", "/health", "/docs"],
)

app = Litestar(
    route_handlers=[...],
    on_app_init=[jwt_auth.on_app_init],
)
```

### Token Generation

```python
from litestar.security.jwt import Token

@post("/login")
async def login(data: LoginRequest) -> dict:
    user = await authenticate_user(data.email, data.password)
    
    if not user:
        raise HTTPException(status_code=401, detail="Invalid credentials")
    
    token = Token(
        sub=str(user.id),
        exp=datetime.utcnow() + timedelta(hours=24),
        # Custom claims
        email=user.email,
        roles=user.roles,
    )
    
    encoded = jwt_auth.encode_token(token)
    
    return {"access_token": encoded, "token_type": "bearer"}
```

### Protected Routes

```python
from litestar.security.jwt import JWTAuth

# Option 1: Global protection with excludes
jwt_auth = JWTAuth(
    token_secret="your-secret",
    retrieve_user_handler=retrieve_user,
    exclude=["/public/*"],  # Public routes
)

# Option 2: Per-route middleware
@get("/protected", middleware=[jwt_auth.middleware])
async def protected_route(request: Request) -> dict:
    return {"user_id": request.user.id}

# Option 3: Dependency injection
@get("/profile")
async def profile(user: User) -> dict:  # User auto-injected from JWT
    return {"email": user.email}
```

### Custom Token Claims

```python
from litestar.security.jwt import Token
from dataclasses import dataclass

@dataclass
class CustomToken(Token):
    email: str
    roles: list[str]
    permissions: list[str]

jwt_auth = JWTAuth(
    token_secret="your-secret",
    retrieve_user_handler=retrieve_user,
    token_cls=CustomToken,
)

# In login endpoint
token = CustomToken(
    sub=str(user.id),
    exp=datetime.utcnow() + timedelta(hours=24),
    email=user.email,
    roles=user.roles,
    permissions=user.permissions,
)
```

## Session-Based Authentication

### Server-Side Sessions

```python
from litestar.middleware.session import SessionMiddleware
from litestar.middleware.session.server_side import ServerSideSessionBackend

session_middleware = SessionMiddleware(
    backend=ServerSideSessionBackend(),
    session_config={
        "secret_key": "your-secret",
        "max_age": 3600,
        "cookie_name": "session_id",
        "secure": True,
        "httponly": True,
        "samesite": "lax",
    }
)

app = Litestar(
    route_handlers=[...],
    middleware=[session_middleware],
)

# Using sessions
@post("/login")
async def login(request: Request, data: LoginRequest) -> dict:
    user = await authenticate(data.email, data.password)
    request.session["user_id"] = str(user.id)
    return {"status": "logged in"}

@get("/profile")
async def profile(request: Request) -> dict:
    user_id = request.session.get("user_id")
    if not user_id:
        raise NotAuthorizedException()
    user = await get_user(user_id)
    return {"email": user.email}

@post("/logout")
async def logout(request: Request) -> dict:
    request.session.clear()
    return {"status": "logged out"}
```

### Client-Side Sessions (Encrypted Cookies)

```python
from litestar.middleware.session import SessionMiddleware
from litestar.middleware.session.client_side import ClientSideSessionBackend

# Requires: pip install litestar[cryptography]

session_middleware = SessionMiddleware(
    backend=ClientSideSessionBackend(),
    session_config={
        "secret_key": "your-secret-min-32-characters",
        "max_age": 3600,
    }
)
```

## API Key Authentication

### Custom API Key Middleware

```python
from litestar.middleware import AbstractMiddleware
from litestar.exceptions import NotAuthorizedException

class APIKeyMiddleware(AbstractMiddleware):
    def __init__(self, app, header_name: str = "X-API-Key"):
        super().__init__(app)
        self.header_name = header_name
    
    async def __call__(self, scope, receive, send):
        if scope["type"] != "http":
            await self.app(scope, receive, send)
            return
        
        headers = dict(scope.get("headers", []))
        api_key = headers.get(self.header_name.encode())
        
        if not api_key:
            raise NotAuthorizedException("API key required")
        
        user = await validate_api_key(api_key.decode())
        if not user:
            raise NotAuthorizedException("Invalid API key")
        
        scope["user"] = user
        await self.app(scope, receive, send)

app = Litestar(
    route_handlers=[...],
    middleware=[APIKeyMiddleware],
)
```

## OAuth2 Integration

### OAuth2 with JWT

```python
from litestar.security.jwt import OAuth2Login, OAuth2PasswordBearer

oauth2_scheme = OAuth2PasswordBearer(token_url="/token")

@post("/token", name="login")
async def login(data: OAuth2Login) -> dict:
    user = await authenticate_user(data.username, data.password)
    
    if not user:
        raise HTTPException(
            status_code=401,
            detail="Incorrect username or password"
        )
    
    token = jwt_auth.create_token(subject=str(user.id))
    return {"access_token": token, "token_type": "bearer"}

@get("/users/me")
async def read_users_me(user: User) -> User:
    return user
```

## Password Hashing

```python
from litestar.security.hashing import hash_password, verify_password

@post("/register")
async def register(data: RegisterRequest) -> dict:
    hashed = hash_password(data.password)
    user = await create_user(email=data.email, password_hash=hashed)
    return {"id": user.id, "email": user.email}

async def authenticate_user(email: str, password: str) -> User | None:
    user = await get_user_by_email(email)
    if not user:
        return None
    
    if not verify_password(password, user.password_hash):
        return None
    
    return user
```

## Security Headers

```python
from litestar.middleware.security import SecurityHeadersMiddleware

security_headers = SecurityHeadersMiddleware(
    config={
        "content_security_policy": "default-src 'self'",
        "strict_transport_security": "max-age=31536000; includeSubDomains",
        "x_content_type_options": "nosniff",
        "x_frame_options": "DENY",
        "x_xss_protection": "1; mode=block",
        "referrer_policy": "strict-origin-when-cross-origin",
    }
)
```

## Rate Limiting by User

```python
from litestar.middleware.rate_limit import RateLimitMiddleware, RateLimitConfig

async def rate_limit_key(request) -> str:
    """Generate rate limit key based on user or IP"""
    if request.user:
        return f"user:{request.user.id}"
    return f"ip:{request.client.host}"

rate_limit = RateLimitMiddleware(
    config=RateLimitConfig(
        rate_limit=("minute", 60),
        key_function=rate_limit_key,
    )
)
```
