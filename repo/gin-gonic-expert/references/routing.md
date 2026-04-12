# Routing Reference

## Route Registration

- Register handlers with explicit methods like `GET`, `POST`, `PUT`, `PATCH`, and `DELETE`.
- Prefer grouped registration for non-trivial APIs:

```go
api := r.Group("/api/v1")
users := api.Group("/users")
users.GET(":id", getUser)
users.POST("", createUser)
```

- Prefer resource-scoped registration helpers like `RegisterUserRoutes(api *gin.RouterGroup)` over dumping all routes into `main.go`.

## Path, Query, and Form Access

- Use `c.Param("id")` for path params.
- Use `c.Query("q")` or `c.DefaultQuery("page", "1")` for URL query params.
- Use `c.PostForm("name")` or `c.DefaultPostForm("role", "user")` for form fields.
- Use `c.QueryMap(...)` and `c.PostFormMap(...)` for flat dynamic maps.
- Treat query access and form access as separate sources. They do not automatically fall back to each other.

## Route Syntax and Matching

- Use Gin syntax like `:id` for named params and `*path` for catch-all params.
- Remember that wildcard params include the leading slash.
- Avoid ambiguous route patterns; conflicting param and wildcard patterns can panic at startup.

## Groups and Middleware Scope

- Use `router.Use(...)` for global middleware.
- Use `group.Use(...)` for middleware that applies to one subtree.
- Let broader middleware run first: global before group before per-route.
- Keep API versioning in the path unless the project already uses a different versioning scheme.

## Redirects and Rewrites

- Use `c.Redirect(status, location)` for redirects.
- Prefer `302` or `307` for POST redirects; avoid `301` when clients must preserve method semantics.
- Use `router.HandleContext(c)` only for intentional internal rerouting after rewriting `c.Request.URL.Path`.

## File Uploads

- Use `c.FormFile("file")` for one file.
- Use `c.MultipartForm()` for repeated file fields.
- Use `c.SaveUploadedFile(...)` only after validating destination and file naming.
- Never trust `file.Filename`; sanitize it with `filepath.Base(...)` or replace it entirely.
- Treat `router.MaxMultipartMemory` as memory buffering, not as a hard request-size limit.
- Use `http.MaxBytesReader` when the handler must reject oversized uploads with `413`.

## API Shape

- Keep JSON responses consistent across handlers.
- Use one pagination style per API: offset or cursor.
- Validate sortable fields against an allow-list instead of trusting arbitrary query params.
- Prefer structured application errors over ad hoc JSON bodies.

## Common Mistakes

- Do not use `{id}` route syntax.
- Do not put auth middleware on a group after registering routes inside that group.
- Do not assume `MaxMultipartMemory` prevents large uploads.
- Do not expose raw uploaded filenames or paths back to the filesystem unchecked.
