# Contribution Guide for ShellShop

## Branching Strategy

ShellShop uses a simple `master` / `dev` flow:

- `master`: stable Python/Textual baseline
- `dev`: integration branch for upcoming work
- `feat/<name>` and `fix/<name>`: short-lived branches created from `dev`

If you open a pull request, target `dev` unless the change is a release-only fix that maintainers explicitly want on `master`.

## Development Setup

```bash
python3 -m venv .venv
source .venv/bin/activate
pip install -e .
python -m shellshop
python -m unittest
```

## Commit Structure

Use clear conventional prefixes:

- `feat:` new functionality
- `fix:` bug fixes
- `docs:` documentation changes
- `refactor:` structural changes without a user-facing feature
- `test:` test-only changes
- `chore:` tooling or maintenance

Example: `feat: add YAML catalog loader`

## Pull Request Expectations

- keep PRs focused on one problem
- add or update tests when changing behavior
- update docs when changing workflows or commands
- avoid drive-by refactors in unrelated files

## Issue Structure

Good issues are small enough to finish, but complete enough to start without guessing:

- `Title`: short and descriptive
- `Type`: `[Feature]`, `[Bug]`, `[Docs]`, or `[Chore]`
- `Problem`: what is missing or broken today
- `Proposed change`: the concrete scope of work
- `Acceptance criteria`: how reviewers know the issue is done

The starter backlog for open-source contributors lives in `docs/oss-contribution-plan.md`.
