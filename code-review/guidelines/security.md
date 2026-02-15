# Security Review Guidelines

## Input validation

- All user input must be validated and sanitized before use.
- Never trust client-side validation alone; enforce on the server.
- Use allowlists over denylists where possible.

## Authentication and authorization

- Verify that endpoints enforce authentication where required.
- Check that role/permission checks happen before any data access.
- Tokens and sessions must have expiration and revocation support.

## Secrets and credentials

- No hardcoded secrets, API keys, or passwords in source code.
- Environment variables or secret managers must be used.
- `.env` files must be in `.gitignore`.

## Data exposure

- API responses should not leak internal IDs, stack traces, or debug info.
- Ensure PII is not logged or included in error messages.
- Check that database queries use parameterized statements (no string concatenation).

## Dependencies

- Flag any new dependency additions and check for known vulnerabilities.
- Prefer well-maintained packages with active security disclosure processes.
