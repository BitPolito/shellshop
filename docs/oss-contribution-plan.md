# ShellShop Open-Source Contribution Plan

This backlog assumes five parallel contributors working from `dev`. Each contributor gets a clear lane plus one or two GitHub issues that are small enough to review cleanly.

## Contributor 1: Catalog and Config

### Issue 1
- Title: `[Feature] Load merchant catalog from YAML`
- Problem: the app currently uses hard-coded demo data
- Scope: add a YAML loader that maps merchant metadata and products into the existing Python models
- Acceptance criteria:
  - app can start from a provided YAML file
  - validation errors point to the failing field
  - README includes a sample command

### Issue 2
- Title: `[Feature] Add sample merchant config files`
- Problem: contributors and users have no example configs to modify
- Scope: add at least two realistic sample merchant files and document them
- Acceptance criteria:
  - samples exercise different product mixes
  - samples load without code changes
  - docs explain how to copy and edit them

## Contributor 2: Checkout and Persistence

### Issue 3
- Title: `[Feature] Persist carts and draft orders in SQLite`
- Problem: all cart data is currently in memory
- Scope: add a simple persistence layer for cart snapshots and draft orders
- Acceptance criteria:
  - cart state survives app restart
  - tests cover create, update, and clear paths
  - UI still works without direct DB logic inside widgets

### Issue 4
- Title: `[Feature] Add checkout summary panel`
- Problem: the current app stops at cart aggregation
- Scope: add an order summary view with totals, item counts, and a placeholder payment state
- Acceptance criteria:
  - operator can move from browsing to checkout
  - summary reads from shared store state
  - documentation explains current placeholder behavior

## Contributor 3: Textual UI and UX

### Issue 5
- Title: `[Feature] Refactor the catalog pane into reusable Textual widgets`
- Problem: the first Textual version renders large strings into `Static` widgets
- Scope: replace string-built catalog/detail panels with reusable custom widgets
- Acceptance criteria:
  - widgets have clear responsibilities
  - selection updates remain keyboard-driven
  - code is easier to test and extend

### Issue 6
- Title: `[Feature] Add responsive layouts for narrow terminals`
- Problem: the dashboard assumes a fairly wide terminal
- Scope: adapt the storefront layout for small terminal sizes
- Acceptance criteria:
  - layout remains usable around 80 columns
  - cart and detail content do not become unreadable
  - screenshots or notes are added to the PR description

## Contributor 4: Testing and Tooling

### Issue 7
- Title: `[Chore] Add lint and test automation`
- Problem: the repo has no automated quality gate for the new Python stack
- Scope: add CI that runs unit tests and basic style checks
- Acceptance criteria:
  - pull requests run tests automatically
  - failing checks block merges into `dev`
  - contributor docs include local commands

### Issue 8
- Title: `[Chore] Expand unit tests for store and config edge cases`
- Problem: only the core cart behavior is covered
- Scope: add tests for invalid config, stock limits, and future persistence seams
- Acceptance criteria:
  - edge cases are named explicitly
  - tests stay fast and deterministic
  - failures are easy to diagnose

## Contributor 5: SSH Delivery Exploration

### Issue 9
- Title: `[Research] Propose a Python SSH session architecture for Textual`
- Problem: SSH delivery is part of the vision, but no Python design has been chosen
- Scope: evaluate candidate libraries and document the tradeoffs
- Acceptance criteria:
  - compare at least two viable approaches
  - recommend one path forward
  - list implementation risks and open questions

### Issue 10
- Title: `[Feature] Prototype SSH session bootstrap with a mock terminal client`
- Problem: there is no proof yet that the Textual app can be delivered over SSH cleanly
- Scope: build a minimal prototype that starts a session and handles disconnect or resize events
- Acceptance criteria:
  - prototype code is isolated from the main app
  - limitations are documented honestly
  - maintainers can decide whether to continue or pivot

## Suggested assignment map

- Contributor A: Issues 1 and 2
- Contributor B: Issues 3 and 4
- Contributor C: Issues 5 and 6
- Contributor D: Issues 7 and 8
- Contributor E: Issues 9 and 10
