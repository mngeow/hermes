# Rendering Reference

## Default Response Helpers

- Default to `c.JSON(...)` for typical APIs.
- Use `c.XML(...)`, `c.YAML(...)`, or `c.ProtoBuf(...)` only when the contract requires those formats.
- Treat Gin's render helpers as the normal response-writing surface instead of manual header and encoder handling.

## JSON Variants

- Use `c.PureJSON(...)` when the response must keep `<`, `>`, and `&` unescaped.
- Use `c.SecureJSON(...)` only when top-level array responses need anti-hijacking prefixing for legacy clients.
- Use `c.JSONP(...)` only for legacy cross-domain browser integrations.
- Use `c.AsciiJSON(...)` only when the response must be ASCII-safe.
- Prefer plain `c.JSON(...)` unless there is a concrete compatibility or security requirement.

## Static Files and Downloads

- Use `router.Static(...)`, `router.StaticFS(...)`, and `router.StaticFile(...)` for public assets.
- Use `c.File(...)`, `c.FileAttachment(...)`, or `c.FileFromFS(...)` for handler-driven downloads.
- Use `c.DataFromReader(...)` for streaming or proxying large content.
- Prefer `FileFromFS` with a constrained filesystem when serving user-selected files.

## File Safety

- Never pass unchecked user input directly into `c.File(...)` or `c.FileAttachment(...)`.
- Never mount `router.Static(...)` on directories that contain secrets, config files, or mutable app data.
- Supply `Content-Type` and `Content-Length` carefully when streaming data from a reader.

## HTML Templates

- Use `LoadHTMLGlob(...)`, `LoadHTMLFiles(...)`, or `LoadHTMLFS(...)` to register templates.
- Use `c.HTML(...)` to render a named template.
- Use `SetHTMLTemplate(...)` for a prebuilt template set.
- Use `SetFuncMap(...)` and `Delims(...)` only when the template layer actually needs them.
- Prefer `go:embed` plus `LoadHTMLFS(...)` or `template.ParseFS(...)` for single-binary apps.

## Template Safety

- Rely on `html/template` auto-escaping by default.
- Treat `template.HTML` as a sharp tool; use it only for trusted content.
- When templates share the same basename in multiple directories, define them with unique relative-path names.

## Common Mistakes

- Do not choose `PureJSON` by default.
- Do not use `JSONP` as a substitute for proper CORS in modern APIs.
- Do not treat a file-serving path as safe just because it comes from a request parameter.
- Do not bypass template escaping with untrusted HTML.
