# Operations Reference

## Server Runtime

- Treat Gin as the HTTP handler and prefer a custom `http.Server` for production.
- Set transport-level limits such as `ReadTimeout`, `WriteTimeout`, and `MaxHeaderBytes` explicitly.
- Prefer graceful shutdown with `http.Server.Shutdown(ctx)` on `SIGINT` and `SIGTERM`.
- Use `router.Run()` only for basic local or simple deployments.

## Trusted Proxies

- Configure `SetTrustedProxies(...)` explicitly when the app runs behind proxies or load balancers.
- Use `SetTrustedProxies(nil)` when the app is not behind a trusted proxy chain.
- Treat proxy trust as a security setting, not as optional convenience.
- Remember that proxy settings affect `ClientIP()` and any rate limiting or audit logic built on it.

## Request Context And Cancellation

- Pass `c.Request.Context()` into DB queries, outbound HTTP calls, and other cancelable work.
- Use `context.WithTimeout(...)` for handler-local timeouts when needed.
- Stop background work promptly when the request context is canceled.
- Avoid writing a response after cancellation has already ended useful work.

## Databases

- Open shared DB handles once during startup.
- Inject DB handles through closures or handler structs.
- Use context-aware calls such as `QueryContext(...)` and `QueryRowContext(...)`.
- Tune connection pools explicitly for production workloads.
- Keep transaction boundaries explicit and handle rollback paths carefully.

## Health And Metrics

- Expose a fast liveness endpoint such as `/healthz`.
- Expose a readiness endpoint such as `/readyz` that checks critical dependencies.
- Keep readiness checks timeout-bounded and cheap.
- Prefer Prometheus-style metrics for production services.
- Use `c.FullPath()` for route labels instead of raw URL paths.
- Prefer a separate internal port for metrics and admin surfaces when the deployment allows it.

## Cookies And State

- Use `c.SetCookie(...)` or `c.SetCookieData(...)` intentionally.
- Delete cookies by setting a negative `MaxAge`.
- Default to `HttpOnly` and `Secure` for sensitive cookies.
- Set `SameSite` deliberately instead of leaving browser behavior implicit.

## WebSockets

- Treat WebSockets as a standard-library or companion-package integration around Gin.
- Use `gorilla/websocket` or the project's existing WebSocket stack.
- Restrict origins in the upgrader.
- Serialize writes per connection; do not write concurrently from multiple goroutines.
- Add ping/pong handling and cleanup for long-lived connections.

## Edge Features

- Treat runtime JSON codec replacement as an advanced startup-only decision.
- Treat HTTP/2 server push as legacy behavior; do not design new features around it.
- Use automatic TLS helpers only when they match the deployment model and operational requirements.

## Common Mistakes

- Do not leave trusted proxies at implicit defaults.
- Do not ignore request cancellation for DB or HTTP work.
- Do not label metrics with raw request paths.
- Do not assume a naive WebSocket broadcast loop is production-safe.
