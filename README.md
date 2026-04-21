# ShellShop

ShellShop is now a Python project built around Textual. The repository no longer ships a Rust prototype. Instead, `master` contains a local-first terminal storefront that gives the project a cleaner base for rapid iteration, contributor onboarding, and future SSH delivery work.

## What is in this version

- Python package layout with a `python -m shellshop` entrypoint
- Textual storefront dashboard with catalog, detail, and cart panels
- Pure-Python store state and cart logic that is easy to test
- Updated roadmap and contribution plan for a `master` / `dev` workflow

## What changed

The previous Rust and ratatui experiment has been removed from `master`. SSH transport and Lightning integrations are still part of the product direction, but they now live in the roadmap instead of being coupled to an early Rust implementation.

## Quick start

```bash
python3 -m venv .venv
source .venv/bin/activate
pip install -e .
python -m shellshop
```

Optional:

```bash
python -m shellshop --merchant-name "Sats & Supply"
python -m unittest
```

## Current UX

The Textual app is a local storefront preview with:

- keyboard-driven product selection
- product detail panel
- live cart summary
- theme toggle and lightweight operator shortcuts

Key bindings are shown in the footer. The current app is intentionally local-first so the domain model and interface can stabilize before SSH session handling is added back.

## Branching

- `master`: stable Python/Textual baseline
- `dev`: integration branch for ongoing work
- `feat/*`: focused feature branches created from `dev`

## Next milestones

- load merchant inventory from a config file
- persist carts and orders
- add SSH session delivery for remote shoppers
- integrate Lightning checkout
