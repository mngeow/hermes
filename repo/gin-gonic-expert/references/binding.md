# Binding Reference

## Choose the Narrowest Binder

- Use `c.ShouldBind(...)` when source selection should follow method and `Content-Type`.
- Use `c.ShouldBindJSON(...)` for JSON-only request bodies.
- Use `c.ShouldBindQuery(...)` when the handler must read only query params.
- Use `c.ShouldBindUri(...)` for path params bound into a struct.
- Use `c.ShouldBindHeader(...)` for request metadata.
- Use `c.ShouldBindBodyWith(...)` only when the same request body must be parsed more than once.

## Tag Fields Correctly

- Add the right source tag to every bound field: `json`, `form`, `uri`, `header`, `xml`, or `yaml`.
- Bind only exported struct fields.
- Keep request DTOs separate from persistence models when tags or validation rules diverge.

```go
type ListUsersRequest struct {
    Page int    `form:"page,default=1" binding:"gte=1"`
    Sort string `form:"sort" binding:"omitempty,oneof=name created_at"`
}
```

## Validation

- Put validation rules in `binding` tags.
- Use `binding:"required"` for required fields.
- Use validator tags like `email`, `uuid`, `gte`, `lte`, and `oneof` where appropriate.
- Tag nested struct fields too; nested validation is not automatic unless the fields are tagged.
- Use `binding:"-"` to skip binding and validation for a field.

## URI, Time, and Collections

- Match `uri` tag names exactly to route param names.
- Use `time_format:"2006-01-02"` and related time tags when parsing dates from query or form data.
- Use `collection_format:"multi|csv|ssv|tsv|pipes"` for slices and arrays.
- Remember that `multi` is the default collection format.
- When binding HTML checkbox arrays, match the field name exactly, including `[]` when present.

## Defaults

- Use `form:"field,default=value"` for form and query defaults.
- For collection defaults in `csv` or `multi` formats, use semicolon-separated values in the tag.
- Prefer explicit defaults in the DTO over implicit zero-value semantics when API behavior depends on them.

## Custom Validation and Parsing

- Register custom validators at startup through `binding.Validator.Engine()`.
- Prefer custom validators for reusable business rules, not one-off handler checks.
- Implement `encoding.TextUnmarshaler` or Gin's `BindUnmarshaler` when custom scalar parsing is needed.
- Use a custom `binding.Binding` only when integrating external structs or non-standard tags.

## Body Reuse

- Assume normal body binding consumes `c.Request.Body` once.
- Reach for `ShouldBindBodyWith` only when the handler truly needs to try multiple schemas.
- Avoid repeated body parsing in hot paths because cached-body binding adds overhead.

## Common Mistakes

- Do not rely on `ShouldBind` when handler intent is query-only or header-only.
- Do not forget source tags and then debug silent zero values.
- Do not expect binding to work on unexported fields.
- Do not assume repeated binds are free.
