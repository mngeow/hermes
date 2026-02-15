# Performance Review Guidelines

## Database queries

- Avoid N+1 query patterns; prefer batch/eager loading.
- Check that queries hitting large tables use appropriate indexes.
- Flag any use of `SELECT *` when only specific columns are needed.
- Watch for missing pagination on list endpoints.

## Loops and iteration

- Avoid doing I/O (network calls, file reads, DB queries) inside loops.
- Check for accidentally quadratic algorithms (nested loops over the same data).
- Prefer streaming or chunked processing for large datasets.

## Caching

- Verify cache invalidation logic when data is mutated.
- Check that cache keys are specific enough to avoid stale data.
- Flag any missing caching for expensive, frequently-called operations.

## Resource management

- Ensure connections, file handles, and streams are properly closed.
- Watch for unbounded in-memory collections that can grow with input size.
- Check that timeouts are set on external service calls.
