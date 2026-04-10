"""Textual storefront application."""

from __future__ import annotations

from textual.app import App, ComposeResult
from textual.containers import Horizontal, Vertical
from textual.widgets import Footer, Header, Static

from .catalog import demo_catalog, demo_merchant, format_price_sats
from .store import StoreState


class StorefrontApp(App[None]):
    """Textual storefront shell for local iteration."""

    TITLE = "ShellShop"
    SUB_TITLE = "Python + Textual storefront"
    CSS = """
    Screen {
        background: #11161b;
        color: #f6efe3;
    }

    #chrome {
        height: 1fr;
        padding: 1 2;
        layout: horizontal;
    }

    .column {
        width: 1fr;
        height: 1fr;
    }

    .panel {
        border: round #3e728f;
        background: #18222b;
        color: #f6efe3;
        padding: 1 2;
        margin: 0 1 1 0;
    }

    #hero {
        height: 9;
        border: heavy #d7a84b;
        background: #221910;
    }

    #catalog {
        height: 1fr;
    }

    #detail {
        height: 16;
    }

    #cart {
        height: 1fr;
        border: round #6aa877;
        background: #122018;
    }

    #status {
        height: 6;
        border: round #8a8f9a;
        background: #16191d;
    }
    """

    BINDINGS = [
        ("q", "quit", "Quit"),
        ("j,down", "cursor_down", "Next product"),
        ("k,up", "cursor_up", "Previous product"),
        ("enter,a", "add_selected", "Add to cart"),
        ("x,backspace", "remove_selected", "Remove one"),
        ("d", "toggle_theme", "Dark/Light"),
        ("c", "clear_cart", "Clear cart"),
        ("r", "refresh_view", "Refresh"),
    ]

    def __init__(self, store: StoreState) -> None:
        super().__init__()
        self.store = store
        self._status_message = "Local preview mode. SSH delivery returns in a later phase."

    def compose(self) -> ComposeResult:
        yield Header(show_clock=True)
        with Horizontal(id="chrome"):
            with Vertical(classes="column"):
                yield Static(id="hero", classes="panel")
                yield Static(id="catalog", classes="panel")
            with Vertical(classes="column"):
                yield Static(id="detail", classes="panel")
                yield Static(id="cart", classes="panel")
                yield Static(id="status", classes="panel")
        yield Footer()

    def on_mount(self) -> None:
        self.refresh_view()

    def action_cursor_down(self) -> None:
        self.store.select_next()
        self._status_message = f"Selected {self.store.selected_product.name}."
        self.refresh_view()

    def action_cursor_up(self) -> None:
        self.store.select_previous()
        self._status_message = f"Selected {self.store.selected_product.name}."
        self.refresh_view()

    def action_add_selected(self) -> None:
        product = self.store.selected_product
        quantity = self.store.add_selected_to_cart()
        if quantity >= product.stock:
            self._status_message = f"{product.name} is now capped at in-stock quantity ({product.stock})."
        else:
            self._status_message = f"Added {product.name} to cart. Quantity: {quantity}."
        self.refresh_view()

    def action_remove_selected(self) -> None:
        product = self.store.selected_product
        quantity = self.store.remove_selected_from_cart()
        if quantity == 0:
            self._status_message = f"Removed {product.name} from the cart."
        else:
            self._status_message = f"Reduced {product.name} to quantity {quantity}."
        self.refresh_view()

    def action_toggle_theme(self) -> None:
        self.dark = not self.dark
        palette = "dark" if self.dark else "light"
        self._status_message = f"Switched to {palette} mode."
        self.refresh_view()

    def action_clear_cart(self) -> None:
        self.store.clear_cart()
        self._status_message = "Cleared the cart."
        self.refresh_view()

    def action_refresh_view(self) -> None:
        self.refresh_view()

    def refresh_view(self) -> None:
        self.query_one("#hero", Static).update(self.render_hero())
        self.query_one("#catalog", Static).update(self.render_catalog())
        self.query_one("#detail", Static).update(self.render_detail())
        self.query_one("#cart", Static).update(self.render_cart())
        self.query_one("#status", Static).update(self.render_status())

    def render_hero(self) -> str:
        merchant = self.store.merchant
        return "\n".join(
            [
                merchant.name,
                merchant.headline,
                "",
                f"Location: {merchant.location}",
                f"Promise: {merchant.promise}",
            ]
        )

    def render_catalog(self) -> str:
        lines = ["Catalog", "Use j/k or arrows to move. Press Enter or a to add.", ""]
        for index, product in enumerate(self.store.products):
            cursor = ">" if index == self.store.selected_index else " "
            in_cart = self.store.cart_quantity_for(product.sku)
            suffix = f" | in cart: {in_cart}" if in_cart else ""
            lines.append(
                f"{cursor} {product.name} [{product.category}] - {format_price_sats(product.price_sats)}{suffix}"
            )
            lines.append(f"  {product.tagline}")
        return "\n".join(lines)

    def render_detail(self) -> str:
        product = self.store.selected_product
        features = ", ".join(product.features)
        return "\n".join(
            [
                f"Selected product: {product.name}",
                f"Category: {product.category}",
                f"Price: {format_price_sats(product.price_sats)}",
                f"Stock: {product.stock}",
                f"Features: {features}",
                "",
                product.description,
            ]
        )

    def render_cart(self) -> str:
        lines = ["Cart", ""]
        cart_lines = self.store.cart_lines()
        if not cart_lines:
            lines.append("Cart is empty. Add a product from the catalog pane.")
        else:
            for line in cart_lines:
                lines.append(
                    f"{line.quantity} x {line.product.name} = {format_price_sats(line.line_total_sats)}"
                )
            lines.extend(
                [
                    "",
                    f"Items: {self.store.cart_items_count()}",
                    f"Total: {format_price_sats(self.store.cart_total_sats())}",
                ]
            )
        return "\n".join(lines)

    def render_status(self) -> str:
        return "\n".join(
            [
                "Operator status",
                "",
                self._status_message,
                "Roadmap note: config loading, persistence, SSH delivery, and Lightning checkout are next.",
            ]
        )


def run(merchant_name: str | None = None) -> None:
    """Start the Textual storefront app."""

    store = StoreState(merchant=demo_merchant(merchant_name), products=demo_catalog())
    StorefrontApp(store).run()
