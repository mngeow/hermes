# DTO (Data Transfer Object) Reference

## DTO Basics

DTOs control data flow between client and handler, enabling:
- Data validation and transformation
- Field inclusion/exclusion
- Partial updates (PATCH)
- Read/write separation

## Built-in DTO Types

```python
from litestar.dto import DataclassDTO, MsgspecDTO
from litestar.plugins.pydantic import PydanticDTO

# For dataclasses
class UserDTO(DataclassDTO[User]): ...

# For msgspec.Struct
class ProductDTO(MsgspecDTO[Product]): ...

# For Pydantic models
class OrderDTO(PydanticDTO[Order]): ...
```

## DTO Configuration

```python
from litestar.dto import DTOConfig

class UserDTO(PydanticDTO[User]):
    config = DTOConfig(
        # Exclude fields from serialization
        exclude={"password", "internal_notes"},
        
        # Only include specific fields
        include={"id", "name", "email"},
        
        # Rename fields
        rename={"email_address": "email"},
        
        # Support partial updates (PATCH)
        partial=True,
        
        # Maximum recursion depth for nested models
        max_nested_depth=2,
    )
```

## Using DTOs

### Request DTO (input validation)

```python
@post("/users", dto=CreateUserDTO)
async def create_user(data: User) -> User:
    # data is validated and transformed by CreateUserDTO
    return await save_user(data)
```

### Response DTO (output transformation)

```python
@get("/users", return_dto=UserDTO)
async def list_users() -> list[User]:
    # Returns full User objects, but serialized through UserDTO
    return await get_all_users()
```

### Both directions

```python
@put("/users/{user_id:int}", dto=UpdateUserDTO, return_dto=UserDTO)
async def update_user(user_id: int, data: User) -> User:
    return await update_and_return(user_id, data)
```

## Partial Updates (PATCH)

```python
from litestar.dto import DTOConfig, DTOData

class PartialUserDTO(PydanticDTO[User]):
    config = DTOConfig(partial=True)

@patch("/users/{user_id:int}", dto=PartialUserDTO)
async def patch_user(
    user_id: int,
    data: DTOData[User]  # All fields are Optional
) -> User:
    # data contains only fields that were sent
    update_data = data.as_builtins()  # Convert to dict
    return await partial_update(user_id, update_data)
```

## DTOData Utility

```python
from litestar.dto import DTOData

@post("/users", dto=UserDTO)
async def create_user(data: DTOData[User]) -> User:
    # Access raw validated data as dict
    raw_data = data.as_builtins()
    
    # Create instance manually
    user = data.create_instance(id=generate_id())
    
    # Or just use the validated data directly
    return await save_user(data)
```

## Nested DTOs

```python
from pydantic import BaseModel

class Address(BaseModel):
    street: str
    city: str
    country: str

class User(BaseModel):
    id: int
    name: str
    address: Address

class AddressDTO(PydanticDTO[Address]):
    config = DTOConfig(exclude={"country"})

class UserDTO(PydanticDTO[User]):
    config = DTOConfig(
        exclude={"id"},
        nested_dto={Address: AddressDTO}
    )
```

## Custom DTO Factory

```python
from litestar.dto import AbstractDTO, DTOField
from typing import Generic, TypeVar

T = TypeVar("T")

class PublicDTO(AbstractDTO[T]):
    """DTO that only includes public fields"""
    
    def __init__(self, model_type: type[T]):
        super().__init__(model_type)
        self.config = DTOConfig(
            include={f for f in model_type.__fields__ 
                    if not f.startswith("_")}
        )
```

## Layered DTO Configuration

DTOs can be configured at different layers:

```python
from litestar import Router, Controller

# App level
app = Litestar(
    route_handlers=[...],
    dto=BaseDTO,
    return_dto=ResponseDTO,
)

# Router level
router = Router(
    path="/api",
    route_handlers=[...],
    dto=RouterDTO,  # Overrides app level
)

# Controller level
class UserController(Controller):
    dto = ControllerDTO
    return_dto = ControllerResponseDTO
    
    @get("/")
    async def list(self) -> list[User]: ...

# Handler level
@get("/special", dto=SpecialDTO, return_dto=SpecialResponseDTO)
async def special_endpoint() -> SpecialType: ...
```

## DTO with Aliases

```python
from litestar.dto import DTOConfig

class UserDTO(PydanticDTO[User]):
    config = DTOConfig(
        rename_strategy="camel",  # Convert snake_case to camelCase
    )

# Or explicit renaming
class UserDTO(PydanticDTO[User]):
    config = DTOConfig(
        rename={
            "first_name": "firstName",
            "last_name": "lastName",
            "email_address": "emailAddress",
        }
    )
```

## Validation Configuration

```python
from litestar.dto import DTOConfig

class StrictUserDTO(PydanticDTO[User]):
    config = DTOConfig(
        # Forbid extra fields not in the model
        forbid_unknown_fields=True,
        
        # Custom validation settings
        validation_schema={
            "extra": "forbid",
            "strict": True,
        }
    )
```
