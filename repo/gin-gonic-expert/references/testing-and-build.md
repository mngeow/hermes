# Testing And Build Reference

## HTTP-Level Tests

- Set `gin.SetMode(gin.TestMode)` before building routers in tests.
- Use `httptest.NewRecorder()` plus `http.NewRequest(...)` and `router.ServeHTTP(...)`.
- Test handlers through real HTTP requests instead of calling handler functions directly when routing, middleware, or binding behavior matters.
- Prefer small `setupRouter()` helpers to keep tests readable.

## Table-Driven Tests

- Use table-driven cases for route, method, status, and response-body combinations.
- Cover both success and failure paths for validation, auth, and middleware.
- Keep request-building helpers close to the test file when the payloads are simple.

## Middleware Tests

- Use `gin.New()` plus only the middleware under test and a minimal route.
- Assert both pass-through behavior and abort behavior.
- Verify status code, response body, and side effects like headers or context values.

## Deployment Notes

- Use `router.Run()` without arguments only when the deployment model expects Gin defaults.
- Remember that simple Gin deployment can honor `PORT` when using `Run()` without an explicit address.
- Configure trusted proxies as part of deployment, not as an afterthought.

## Build Tags

- Treat Gin build tags as compile-time feature switches, not handler-level behavior changes.
- Use only one JSON replacement tag at a time.
- Treat `go_json` as the safest portable JSON replacement.
- Treat `jsoniter` and `sonic` as project-level decisions, not casual per-task edits.
- Remember that `sonic` requires AVX-capable x86_64 hardware.
- Use `nomsgpack` only when the project does not rely on MsgPack support such as `c.MsgPack()`.
- Combine `nomsgpack` with a JSON replacement tag only when the project already uses that build setup.

## Common Mistakes

- Do not forget `gin.TestMode` and then chase noisy debug output in tests.
- Do not test only the happy path for auth or validation middleware.
- Do not introduce build tags casually into a project that has no existing build-tag workflow.
