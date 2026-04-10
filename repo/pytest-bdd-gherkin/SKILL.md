---
name: pytest-bdd-gherkin
description: Write and maintain behavior tests with pytest-bdd using Gherkin feature files and Python step definitions. Use when creating or refactoring Feature, Scenario, Rule, and Scenario Outline tests; mapping Given/When/Then steps to fixtures; and handling datatables, docstrings, tags, and parser-based step arguments.
---

# Pytest-BDD Gherkin Testing

Use this skill to create clear, maintainable pytest-bdd tests that align with current pytest-bdd (8.x) and Gherkin behavior.

## Follow this workflow

1. Create one `.feature` file per behavior area.
2. Write short, user-observable scenarios in domain language.
3. Bind scenarios in Python with `@scenario(...)` or `scenarios(...)`.
4. Implement steps using fixture injection and explicit parsers.
5. Use `Scenario Outline` + `Examples` for data permutations.
6. Reuse shared steps in `conftest.py`.
7. Run pytest with marker filters and fix missing or ambiguous steps.

## Start with a clean layout

Use a layout that keeps feature files and Python bindings predictable:

```text
features/
  checkout.feature
tests/bdd/
  test_checkout.py
  conftest.py
```

If you want feature discovery to be relative to a shared folder, set:

```ini
[pytest]
bdd_features_base_dir = features/
```

## Write valid Gherkin first

- Start each file with `Feature:`.
- Keep exactly one `Feature` per `.feature` file.
- Use `Scenario` or `Example` (aliases), `Rule`, `Background`, `Scenario Outline`, and `Examples` as needed.
- Keep comments on their own lines (`# comment`).
- Use `# language: <code>` on line 1 when using localized keywords.
- Use `*` as a readable continuation keyword when listing steps.
- Keep `Background` to `Given` steps only.

Use step intent consistently:

- `Given`: describe context and preconditions.
- `When`: describe an action or event.
- `Then`: assert observable outcomes (not hidden internals by default).

Keep scenarios concise (typically 3-5 steps).

## Bind scenarios explicitly

Use manual binding for selected scenarios:

```python
from pytest_bdd import scenario


@scenario("checkout.feature", "Successful payment")
def test_successful_payment() -> None:
    pass
```

Use auto-binding for many scenarios:

```python
from pytest_bdd import scenarios


scenarios("features")
```

If combining both, define manual `@scenario(...)` bindings before `scenarios(...)`.

## Implement steps with fixtures and `target_fixture`

Prefer fixture injection over global mutable context:

```python
from pytest_bdd import given, when, then


@given("a cart with one item", target_fixture="cart")
def cart_with_item():
    return {"items": 1, "paid": False}


@when("the user pays")
def pay(cart):
    cart["paid"] = True


@then("the order is paid")
def order_paid(cart):
    assert cart["paid"] is True
```

Use `target_fixture` whenever a step should create or override a fixture value for that scenario.

## Parse step parameters deliberately

Use parser choice intentionally:

- `string` (default): exact text match.
- `parsers.parse(...)`: readable typed placeholders (`{count:d}`).
- `parsers.cfparse(...)`: cardinality-aware parsing.
- `parsers.re(...)`: regex with named groups.

Example:

```python
from pytest_bdd import given, parsers


@given(parsers.parse("there are {count:d} users"), target_fixture="user_count")
def user_count(count: int) -> int:
    return count
```

Important parser rules:

- `parsers.re(...)` uses full-string matching in modern pytest-bdd.
- Do not use `<param>` in step decorator text; use parser placeholders like `{param}`.
- Step text keywords (`Given`/`When`/`Then`) are not part of lookup identity, so avoid duplicate raw step text with different intent.

## Use scenario outlines for data variations

Prefer `Scenario Outline` over copy-pasted scenarios:

```gherkin
Scenario Outline: login behavior
  Given user "<username>" exists
  When user logs in with password "<password>"
  Then login result is "<result>"

  @happy
  Examples:
    | username | password | result  |
    | alice    | s3cret   | success |

  @sad
  Examples:
    | username | password | result  |
    | alice    | wrong    | failure |
```

Guidance:

- Use multiple `Examples` tables when groups have different meaning.
- Tag `Examples` blocks to run subsets.
- Remember empty table cells map to empty strings by default; convert explicitly when you need `None`.

## Handle datatables and docstrings correctly

Use `datatable` for table-attached steps and `docstring` for multiline string-attached steps.

- `datatable` arrives as `list[list[str]]`.
- `docstring` arrives as a single multiline string.
- Omit these parameters when not needed.
- If a step function asks for `datatable` or `docstring` but the step has none, pytest fixture resolution fails.

Reserved names:

- Do not define parser argument names `datatable` or `docstring` in step text patterns; they are reserved.

## Reuse steps and aliases

- Put cross-file reusable steps in `tests/conftest.py`.
- Add multiple decorators to one function for aliases.
- Keep step phrases stable; refactor internals freely.

Example alias pattern:

```python
@given("I have an article", target_fixture="article")
@given("there is an article", target_fixture="article")
def article():
    return {"title": "sample"}
```

## Use tags and selection safely

Tags become pytest markers. Prefer marker selection for stable runs:

```bash
pytest -m "checkout and not slow"
```

If using `--strict-markers`, define all BDD tags in `pytest.ini`.

Use `pytest_bdd_apply_tag` hook when tag-to-marker behavior needs custom logic (for example `@todo` -> skip).

## Use useful CLI features

- Generate a Cucumber JSON report: `pytest --cucumberjson=report.json`
- Use terminal Gherkin reporting: `pytest -v --gherkin-terminal-reporter`
- Generate skeletons from features: `pytest-bdd generate features/my.feature`
- Suggest missing bindings/steps: `pytest --generate-missing --feature features tests`

## Apply 8.x guardrails

- Require Python 3.9+ and pytest 7+.
- Expect parser behavior aligned to `gherkin-official`.
- Keep tags space-free (for example `@smoke_fast`, not `@smoke fast`).
- Use triple-quote blocks for multiline step arguments.
- Treat text after `#` in names as significant text in modern versions.
- Keep `Feature:` keyword present; missing it invalidates the file.
- Remember step arguments are not automatic fixtures; use `target_fixture` when a step should expose a fixture.
- Do not use feature-level examples or vertical examples.

## Final quality checklist

- Ensure each feature file has one `Feature` and valid Gherkin structure.
- Ensure each scenario step has exactly one matching step definition.
- Ensure assertions live in `Then` steps and verify user-observable outcomes.
- Ensure shared setup is in fixtures/backgrounds, not duplicated imperative code.
- Ensure marker names are declared when strict marker mode is enabled.
- Ensure scenario outlines are used for data variation instead of duplicated scenarios.
