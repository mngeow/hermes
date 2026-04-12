# Middleware And Security Reference

## Middleware Lifecycle

- Write middleware as a `gin.HandlerFunc` wrapper.
- Run pre-handler work before `c.Next()`.
- Run post-handler work after `c.Next()`.
- Use `c.Abort()` or `c.AbortWithStatusJSON(...)` to stop the chain.
- Remember that middleware order is stack-like: first in, last out.

## Scope

- Attach cross-cutting behavior globally with `router.Use(...)`.
- Attach feature-specific behavior on route groups.
- Attach one-off behavior per route only when it is truly route-specific.
- Prefer `gin.New()` plus explicit middleware when replacing the default logger or recovery.

## Custom Middleware Patterns

- Use `c.Set(...)` for request-scoped values.
- Use `c.MustGet(...)` only when a previous middleware guarantees the value exists.
- Inspect `c.Writer.Status()` after `c.Next()` when the middleware needs the final status code.
- Centralize API error formatting by inspecting `c.Errors` after handlers run.

## Goroutines

- Never use the original `*gin.Context` in a goroutine.
- Call `c.Copy()` first, or extract the exact values needed and pass those instead.
- Treat copied context data as read-oriented; do not write responses after the request lifecycle has ended.

## Authentication And Sessions

- Use `gin.BasicAuth(...)` only for simple internal tools or compatibility scenarios.
- Require HTTPS for Basic Auth.
- Prefer token-based or application-specific auth flows for real production APIs.
- Use `gin-contrib/sessions` for cookie or backend-backed sessions.
- Always call `session.Save()` after mutating session state.
- Set `HttpOnly`, `Secure`, and appropriate `SameSite` values on auth cookies.

## Security Controls

- Configure CORS explicitly; never combine wildcard origins with credentials.
- Add CSRF protection for cookie-authenticated browser flows.
- Add rate limiting where abuse risk exists.
- Add security headers intentionally rather than assuming middleware elsewhere already does it.
- Validate host and proxy configuration before relying on `ClientIP()` or host-derived behavior.
- Use Gin binding plus parameterized SQL; never build SQL with string concatenation.

## Dependency Injection

- Prefer closure-based injection for small and medium services.
- Prefer handler structs when many handlers share the same dependencies.
- Prefer request context propagation with `c.Request.Context()` for downstream I/O.
- Use Gin context keys for dependencies only when a middleware-produced value is truly request-scoped.

## Common Mistakes

- Do not forget `gin.Recovery()` when using `gin.New()`.
- Do not leak raw internal errors from centralized error middleware.
- Do not keep secrets or real credentials in `gin.BasicAuth(...)` literals.
- Do not treat context-key injection as the default DI style.
