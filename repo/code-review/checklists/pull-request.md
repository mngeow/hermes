# Pull Request Review Checklist

Work through each item below for every PR.

## Correctness

- [ ] Code does what the PR description claims.
- [ ] Edge cases are handled (empty input, nulls, boundary values).
- [ ] Error paths return meaningful errors and do not silently fail.

## Readability

- [ ] Variable and function names are clear and consistent with the codebase.
- [ ] Complex logic has comments explaining intent (not restating the code).
- [ ] No dead code, commented-out blocks, or leftover debug statements.

## Testing

- [ ] New behavior has corresponding tests.
- [ ] Tests cover both happy path and error/edge cases.
- [ ] Existing tests still pass (no unrelated breakage).

## Security

- [ ] Apply items from `guidelines/security.md` relevant to this change.

## Performance

- [ ] Apply items from `guidelines/performance.md` relevant to this change.

## Housekeeping

- [ ] No unrelated changes bundled into this PR.
- [ ] Commit messages are clear and descriptive.
- [ ] Any new dependencies are justified and documented.
