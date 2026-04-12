---
name: gin-gonic-expert
description: Expert guidance for building and maintaining Go web services with the Gin framework. Use when Codex is coding in a Gin-based project for route handlers, route groups, request binding and validation, middleware, authentication, sessions, JSON or HTML responses, file uploads, server configuration, logging, testing, deployment, or other Gin-specific development tasks.
---

# Gin Gonic Expert

## Overview

Use this skill to make Gin-native implementation choices instead of treating a Gin project like generic `net/http`. Keep the skill lean: read the main file first, then load only the reference file that matches the task.

## Workflow

1. Identify the Gin surface area before editing.
- Routing, handlers, uploads, or redirects: read `references/routing.md`.
- Request DTOs, tags, validators, or custom binding: read `references/binding.md`.
- JSON, HTML, file serving, or streaming responses: read `references/rendering.md`.
- Middleware, auth, sessions, security, or dependency injection: read `references/middleware-security.md`.
- Server runtime, proxies, context cancellation, databases, metrics, health checks, or WebSockets: read `references/operations.md`.
- Tests, deployment details, or build tags: read `references/testing-and-build.md`.

2. Match the project's existing Gin conventions before adding new ones.
- Preserve `gin.Default()` vs `gin.New()` unless there is a concrete reason to change it.
- Preserve the current route registration style: inline in `main`, `Register*Routes(...)`, or handler structs.
- Preserve the existing dependency style: closures, app struct methods, or context-injected values.
- Preserve the existing logging stack; do not stack Gin's default logger on top of custom structured logging unless the project already does that.

3. Prefer Gin-native primitives while coding.
- Use `*gin.Context` helpers for request parsing and responses.
- Use route groups for versioning and shared middleware.
- Use the narrowest binder that matches handler intent.
- Pass `c.Request.Context()` into DB and outbound HTTP calls.
- Call `c.Copy()` before a goroutine reads request-scoped data.

4. Verify through HTTP-level tests when behavior changes.
- Prefer `httptest.NewRecorder()`, `http.NewRequest(...)`, and `router.ServeHTTP(...)`.
- Set `gin.SetMode(gin.TestMode)` in tests.

## Core Defaults

- Prefer `gin.Default()` for standard services.
- Use `gin.New()` only when deliberately replacing the default logger and/or recovery. Add `gin.Recovery()` yourself.
- Prefer `net/http` status constants instead of numeric literals.
- Keep handlers thin; inject services via closures or handler structs instead of global state.
- Register larger APIs under grouped prefixes such as `/api/v1`.
- Prefer consistent JSON response and error shapes across the project.

## High-Value Rules

- Use Gin path syntax like `:id` and `*path`, not `{id}`.
- Treat `c.Query()` and `c.PostForm()` as different sources; they do not fall back to each other.
- Treat `router.MaxMultipartMemory` as a buffering knob, not a strict upload limit.
- Use `http.MaxBytesReader` when the app must reject oversized uploads.
- Bind only exported struct fields, and always add the correct source tags.
- Use `ShouldBindQuery`, `ShouldBindJSON`, `ShouldBindUri`, or `ShouldBindHeader` when the request source must be explicit.
- Use `ShouldBindBodyWith` only when the same request body must be parsed more than once.
- Never reuse the original `*gin.Context` inside a goroutine.
- Configure `SetTrustedProxies(...)` or `SetTrustedProxies(nil)` explicitly in deployed apps.
- Prefer a custom `http.Server` plus graceful shutdown for production instead of only `router.Run()`.
- Use `c.FullPath()` for metrics labels to avoid cardinality explosions.
- Skip or redact query strings in request logs by default.

## Minimal Skeleton

```go
r := gin.Default()

api := r.Group("/api/v1")
api.GET("/healthz", func(c *gin.Context) {
    c.JSON(http.StatusOK, gin.H{"status": "ok"})
})

srv := &http.Server{
    Addr:         ":8080",
    Handler:      r,
    ReadTimeout:  5 * time.Second,
    WriteTimeout: 10 * time.Second,
}
```

## Reference Map

- `references/routing.md`: route registration, params, groups, redirects, uploads, API layout.
- `references/binding.md`: binders, tags, validators, collections, custom parsing, body reuse.
- `references/rendering.md`: JSON variants, files, streaming, templates.
- `references/middleware-security.md`: middleware lifecycle, auth, sessions, security, dependency injection.
- `references/operations.md`: custom server, trusted proxies, request context, databases, health, metrics, WebSockets.
- `references/testing-and-build.md`: `httptest`, `gin.TestMode`, deployment notes, build tags.
