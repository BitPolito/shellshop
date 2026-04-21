# ShellShop Roadmap

ShellShop is now a Python/Textual codebase. The goal remains the same: a terminal-first commerce platform that can eventually be delivered both locally and over SSH.

## Phase 1 — Python Reset ✓
Goal: establish a clean Python baseline after removing the Rust prototype.

- [x] Replace the Rust crate with a Python package
- [x] Ship a Textual storefront shell on `master`
- [x] Move cart and catalog state into testable Python modules
- [x] Reframe the repository around `master` and `dev`

## Phase 2 — Catalog and Merchant Config
Goal: make the storefront merchant-editable without code changes.

- [ ] Load products, merchant branding, and theme hints from config
- [ ] Validate config shape with clear startup errors
- [ ] Support local asset references for product media
- [ ] Add sample merchant configs for demos and tests

## Phase 3 — Local Checkout and Persistence
Goal: turn the UI shell into a real local application.

- [ ] Persist cart sessions and orders in SQLite
- [ ] Add order status and checkout flow screens
- [ ] Record inventory changes and stock validation
- [ ] Build a service layer between Textual widgets and storage

## Phase 4 — SSH Delivery
Goal: reintroduce remote terminal access on top of the Python app.

- [ ] Evaluate AsyncSSH or another Python SSH server approach
- [ ] Spawn isolated Textual sessions per client connection
- [ ] Forward terminal resize and disconnect events cleanly
- [ ] Define auth and host-key management for self-hosted shops

## Phase 5 — Lightning Integration
Goal: accept real payments without losing the terminal-first UX.

- [ ] Create a payment backend interface for invoice creation and polling
- [ ] Add a mock backend for local development
- [ ] Implement a first real Lightning backend
- [ ] Reflect payment state live inside checkout views

## Phase 6 — Production Readiness
Goal: make the app easy to operate and safe to contribute to.

- [ ] Add CI for tests, linting, and packaging checks
- [ ] Add container and process-manager deployment targets
- [ ] Improve observability, logging, and crash reporting
- [ ] Publish contributor-ready issue templates and release steps
