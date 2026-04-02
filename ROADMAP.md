# ShellShop Roadmap

ShellShop is a self-hostable, terminal-first e-commerce platform accessible via SSH with native Lightning Network payments.

---

## Phase 1 — Foundation ✓
**Goal**: Hello world TUI and project skeleton.
- [x] ratatui hello world TUI
- [x] Project structure scaffolded (modules, Cargo.toml, .env.example)
- [x] LICENSE, README, ROADMAP

---

## Phase 2 — SSH Server
**Goal**: Accept real SSH connections and spawn a TUI session per client.
- [x] Embed `russh` SSH server, bind to configurable host/port
- [x] Generate and persist an Ed25519 host key on first run
- [x] Spawn an isolated `App` instance per SSH session
- [x] Pass terminal dimensions from the SSH PTY to ratatui
- [x] Graceful disconnect on session close or timeout

---

## Phase 3 — Core TUI
**Goal**: Full browsable storefront with cart and checkout screens.
- [ ] Storefront screen — product grid/list with navigation
- [ ] Product detail screen — name, description, price, add-to-cart
- [ ] Cart screen — line items, quantities, total, proceed to checkout
- [ ] Checkout screen — display Lightning invoice QR + payment status polling
- [ ] Keyboard navigation (vim-style `j/k`, `Enter`, `q`, `Esc`)
- [ ] Responsive layout adapting to terminal width

---

## Phase 4 — Database
**Goal**: Persist products, orders, and cart sessions in SQLite.
- [ ] `sqlx` SQLite setup with compile-time query checking
- [ ] Migration system (`sqlx migrate`)
- [ ] `products` table — id, name, description, price_sats, stock, image_path
- [ ] `orders` table — id, created_at, status, total_sats, payment_hash
- [ ] `order_items` table — order_id, product_id, quantity, unit_price_sats
- [ ] CRUD helpers behind a `Db` trait (swappable for testing)

---

## Phase 5 — Lightning Network
**Goal**: Accept real Lightning payments via a modular backend.
- [ ] Define `LightningBackend` trait: `create_invoice`, `check_payment`
- [ ] `LdkBackend` — embedded `ldk-node`, connect to Esplora, open channels
- [ ] Invoice generation tied to an order at checkout
- [ ] Payment status polling loop in checkout screen
- [ ] Order marked `paid` on confirmed payment

---

## Phase 6 — Merchant Config
**Goal**: Zero-code shop setup via a single YAML file.
- [ ] `items.yaml` schema — shop name, products, theme colors
- [ ] Hot-reload config without restarting the server
- [ ] Validation with clear error messages on startup

---

## Phase 7 — External Lightning Backend
**Goal**: Let merchants receive payments without running a full node.
- [ ] `ExternalBackend` implementing `LightningBackend` trait
- [ ] Support Lightning Address (LNURL-pay) as settlement destination
- [ ] Platform-managed routing node forwards to merchant's address
- [ ] Merchant only needs to set `LIGHTNING_ADDRESS=name@domain.com`

---

## Phase 8 — Production Hardening
**Goal**: Ready for self-hosting and public exposure.
- [ ] Docker image + `docker-compose.yml` with volume mounts
- [ ] Rate limiting per SSH client IP
- [ ] Systemd service file
- [ ] Multi-merchant support (virtual hosts via SSH username)
- [ ] Metrics endpoint (order count, revenue, active sessions)
- [ ] End-to-end test suite using a regtest Lightning network