# WebSockets Reference

## WebSocket Handler (Low-level)

The basic WebSocket handler provides direct control over the connection.

```python
from litestar import websocket, WebSocket

@websocket("/ws")
async def ws_handler(socket: WebSocket) -> None:
    await socket.accept()
    
    try:
        while True:
            # Receive text
            message = await socket.receive_text()
            await socket.send_text(f"Echo: {message}")
            
            # Or receive JSON
            data = await socket.receive_json()
            await socket.send_json({"received": data})
            
            # Or receive bytes
            binary = await socket.receive_bytes()
            await socket.send_bytes(binary)
            
    except WebSocketDisconnect:
        print("Client disconnected")
    finally:
        await socket.close()
```

## WebSocket Listener (High-level)

WebSocket listeners provide a simpler, event-driven interface.

### Basic Listener

```python
from litestar import websocket_listener

@websocket_listener("/ws")
async def echo_listener(data: str) -> str:
    return f"Echo: {data}"
```

### JSON Data

```python
from pydantic import BaseModel

class Message(BaseModel):
    user: str
    content: str

@websocket_listener("/chat")
async def chat_listener(data: Message) -> dict:
    return {
        "user": data.user,
        "response": f"Received: {data.content}"
    }
```

### Binary Data

```python
@websocket_listener("/binary")
async def binary_listener(data: bytes) -> bytes:
    # Process binary data
    processed = process_binary(data)
    return processed
```

### Accessing WebSocket Instance

```python
@websocket_listener("/ws")
async def listener_with_socket(data: str, socket: WebSocket) -> str:
    # Access connection info
    client = socket.client
    headers = socket.headers
    
    # Send to specific client
    await socket.send_text(f"Hello {client.host}")
    
    return f"Received: {data}"
```

### Dependency Injection

```python
async def get_user(token: str) -> User: ...

@websocket_listener("/ws", dependencies={"user": Provide(get_user)})
async def secured_listener(data: str, user: User) -> str:
    return f"Hello {user.name}, you sent: {data}"
```

### Yield Dependencies

```python
async def get_db_connection() -> AsyncGenerator[Connection, None]:
    conn = await create_connection()
    try:
        yield conn
    finally:
        await conn.close()

@websocket_listener("/ws", dependencies={"db": Provide(get_db_connection)})
async def db_listener(data: str, db: Connection) -> str:
    # Connection held for entire WebSocket session
    result = await db.query(data)
    return str(result)
```

## WebSocket Streams

Streams allow proactive pushing of data to clients.

### Basic Stream

```python
from litestar import websocket_stream
import asyncio

@websocket_stream("/time")
async def time_stream() -> AsyncGenerator[str, None]:
    """Stream current time every second"""
    while True:
        yield datetime.now().isoformat()
        await asyncio.sleep(1)
```

### Stream with Serialization

```python
class StatusUpdate(BaseModel):
    status: str
    progress: float
    timestamp: datetime

@websocket_stream("/progress")
async def progress_stream(task_id: str) -> AsyncGenerator[StatusUpdate, None]:
    for i in range(100):
        yield StatusUpdate(
            status="processing",
            progress=i / 100,
            timestamp=datetime.now()
        )
        await asyncio.sleep(0.1)
```

### Stream with Direct Socket Access

```python
@websocket_stream("/notifications")
async def notification_stream(socket: WebSocket) -> AsyncGenerator[dict, None]:
    user_id = socket.scope.get("user_id")
    
    async for notification in notification_queue.subscribe(user_id):
        yield notification
```

## Class-Based WebSocket Listeners

For more complex logic, use class-based listeners.

```python
from litestar.handlers import WebsocketListener

class ChatHandler(WebsocketListener):
    path = "/chat"
    
    async def on_accept(self, socket: WebSocket) -> None:
        """Called when connection is accepted"""
        user = await self.authenticate(socket)
        socket.scope["user"] = user
        await self.join_room(user, "general")
    
    async def on_receive(self, data: str, socket: WebSocket) -> str:
        """Handle incoming message"""
        user = socket.scope["user"]
        await self.broadcast_to_room(user, "general", data)
        return f"{user.name}: {data}"
    
    async def on_disconnect(self, socket: WebSocket) -> None:
        """Called on disconnect"""
        user = socket.scope.get("user")
        if user:
            await self.leave_room(user, "general")
    
    async def authenticate(self, socket: WebSocket) -> User:
        token = socket.headers.get("authorization", "").replace("Bearer ", "")
        return await verify_token(token)
```

## Custom Connection Handling

### Connection Acceptance

```python
async def custom_accept(socket: WebSocket) -> None:
    # Custom headers or subprotocols
    await socket.accept(
        headers=[(b"X-Custom", b"value")],
        subprotocol="chat-v1"
    )

@websocket_listener("/ws", connection_accept_handler=custom_accept)
async def listener(data: str) -> str:
    return data
```

### Rejecting Connections

```python
@websocket("/ws")
async def secured_ws(socket: WebSocket) -> None:
    token = socket.query_params.get("token")
    
    if not await validate_token(token):
        await socket.close(code=4001, reason="Invalid token")
        return
    
    await socket.accept()
    # ... handle connection
```

## Broadcasting

### In-Memory Broadcasting

```python
from litestar.channels import ChannelsPlugin

channels = ChannelsPlugin(
    backend=MemoryChannelsBackend(),
    arbitrary_channels_allowed=True,
)

app = Litestar(
    route_handlers=[...],
    plugins=[channels],
)

@websocket_listener("/chat/{room:str}")
async def chat_listener(data: str, room: str) -> None:
    # Publish to channel
    await channels.publish(data, channels=[room])
    # No return = no direct response

@websocket_stream("/chat/{room:str}/stream")
async def chat_stream(room: str) -> AsyncGenerator[str, None]:
    # Subscribe to channel
    async with channels.start_subscription([room]) as subscriber:
        async for message in subscriber:
            yield message
```

### Redis Broadcasting

```python
from litestar.channels import ChannelsPlugin
from litestar.channels.backends.redis import RedisChannelsBackend

channels = ChannelsPlugin(
    backend=RedisChannelsBackend(redis_url="redis://localhost"),
    arbitrary_channels_allowed=True,
)
```

## WebSocket with HTTP Together

```python
class ChatController(Controller):
    path = "/chat"
    
    @get("/history")
    async def get_history(self) -> list[Message]:
        return await load_chat_history()
    
    @websocket_listener("/live")
    async def live_chat(self, data: str) -> str:
        await save_message(data)
        return data
```

## Error Handling

```python
@websocket("/ws")
async def error_handling_ws(socket: WebSocket) -> None:
    await socket.accept()
    
    try:
        while True:
            try:
                message = await socket.receive_text()
                result = await process_message(message)
                await socket.send_json({"success": True, "data": result})
            except ValidationError as e:
                await socket.send_json({
                    "success": False,
                    "error": "Validation failed",
                    "details": e.errors()
                })
            except Exception as e:
                await socket.send_json({
                    "success": False,
                    "error": "Internal error"
                })
                logger.exception("WebSocket error")
    except WebSocketDisconnect:
        pass
    finally:
        await socket.close()
```

## Testing WebSockets

```python
from litestar.testing import TestClient

def test_websocket():
    with TestClient(app=app) as client:
        with client.websocket_connect("/ws") as ws:
            ws.send_text("Hello")
            response = ws.receive_text()
            assert response == "Echo: Hello"
            
            ws.send_json({"message": "test"})
            response = ws.receive_json()
            assert response == {"received": {"message": "test"}}
```
