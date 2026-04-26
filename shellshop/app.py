"""Textual storefront application."""

from __future__ import annotations

from enum import StrEnum

from rich.console import Group
from rich.text import Text
from textual.app import App, ComposeResult
from textual.containers import Vertical
from textual.widgets import Static
from textual.containers import Container

from .catalog import demo_catalog, demo_merchant, format_price_sats
from .store import StoreState

LOGO = r"""
  ▄█████ ▄▄ ▄▄ ▄▄▄▄▄ ▄▄    ▄▄    ▄█████ ▄▄ ▄▄  ▄▄▄  ▄▄▄▄
   ▀▀▀▄▄▄ ██▄██ ██▄▄  ██    ██    ▀▀▀▄▄▄ ██▄██ ██▀██ ██▄█▀
█████▀ ██ ██ ██▄▄▄ ██▄▄▄ ██▄▄▄ █████▀ ██ ██ ▀███▀ ██
""".strip("\n")

HOST_FINGERPRINT = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIGhvc3Qta2V5LXNhdHMtcHJpdmFjeS1ub2Rl"
BINARY_EASTER_EGG = " ".join(f"{ord(char):08b}" for char in "cypherpunks write code")

ACCENT = "#f7931a"
CYAN = "#4fd6c5"
MUTED = "#7e8a97"
TEXT = "#efe7d2"
DIM = "#c9bea4"


class ViewMode(StrEnum):
    SHOP = "shop"
    CART = "cart"
    PRIVACY = "privacy"
    HELP = "help"


class StorefrontApp(App[None]):
    """Textual storefront shell for local iteration."""

    TITLE = "ShellShop"
    SUB_TITLE = "ssh-first storefront preview"
    CSS = """
    App {
        background: ansi_default;
        color: ansi_default;
    }

    Screen {
        background: ansi_default;
        color: #e7dfcf;
    }

    #frame {
        height: 1fr;
        padding: 1 2 0 2;
        layout: vertical;
        background: ansi_default;
    }

    #hero-container {
        width: 100%;
        height: auto;
        min-height: 14;
        align: center middle;
    }

    #hero {
        height: auto;
        min-height: 10;
        width: 100%;
        max-width: 108;
        border: heavy #f7931a;
        background: ansi_default;
        color: #f6e7c6;
        content-align: center middle;
        text-align: center;
        padding: 1 2;
    }

    #container-for-tabs {
        width: 100%;
        height: auto;
        min-height: 3;
        align: center middle;
    }

    #tabs {
        height: 3;
        width: 100%;
        max-width: 80;
        border: round #2f8f83;
        background: ansi_default;
        color: #88d0c6;
        content-align: center middle;
    }

    #main {
        height: 1fr;
        margin: 1 0 0 0;
        layout: vertical;
        background: ansi_default;
    }

    #content-wrap {
        width: 1fr;
        height: 1fr;
        align-horizontal: center;
        background: ansi_default;
    }

    #content {
        width: 100%;
        max-width: 108;
        height: 1fr;
        border: heavy #f7931a;
        background: ansi_default;
        color: #f4efe4;
        padding: 1 2;
    }

    #status {
        height: auto;
        min-height: 6;
        border: round #577283;
        border-subtitle-align: center;
        background: ansi_default;
        color: #d8e9f3;
        padding: 1 2;
        margin: 1 0 0 0;
    }

    #bindings {
        height: auto;
        min-height: 2;
        background: ansi_default;
        color: #d8e9f3;
        padding: 0 1;
        content-align: center middle;
    }
    """

    BINDINGS = [
        ("q", "quit", "Quit"),
        ("s", "show_shop", "Shop"),
        ("c", "show_cart", "Cart"),
        ("a", "show_privacy", "Privacy"),
        ("h", "show_help", "Help"),
        ("j,down", "cursor_down", "Next product"),
        ("k,up", "cursor_up", "Previous product"),
        ("enter", "add_selected", "Add to cart"),
        ("x,backspace", "remove_selected", "Remove one"),
        ("d", "toggle_theme", "Dark/Light"),
        ("ctrl+c", "clear_cart", "Clear cart"),
        ("r", "refresh_view", "Refresh"),
    ]

    def __init__(self, store: StoreState) -> None:
        super().__init__()
        self.theme = "textual-ansi"
        self.store = store
        self.mode = ViewMode.SHOP
        self._status_message = "local preview mode. ssh delivery and lightning checkout land in later phases."

    def compose(self) -> ComposeResult:
        with Vertical(id="frame"):
            with Container(id="hero-container"):
                yield Static(id="hero")
            with Container(id="container-for-tabs"):
                yield Static(id="tabs")
            with Vertical(id="main"):
                with Vertical(id="content-wrap"):
                    yield Static(id="content")
                yield Static(id="status")
            yield Static(id="bindings")

    def on_mount(self) -> None:
        self.query_one("#status", Static).border_subtitle = BINARY_EASTER_EGG
        self.refresh_view()

    def action_show_shop(self) -> None:
        self.mode = ViewMode.SHOP
        self._status_message = "shop view. browse inventory with j/k and add with enter."
        self.refresh_view()

    def action_show_cart(self) -> None:
        self.mode = ViewMode.CART
        self._status_message = "cart view. this is the state we will later turn into checkout."
        self.refresh_view()

    def action_show_privacy(self) -> None:
        self.mode = ViewMode.PRIVACY
        self._status_message = "privacy view. the project is being rewritten around quiet defaults."
        self.refresh_view()

    def action_show_help(self) -> None:
        self.mode = ViewMode.HELP
        self._status_message = "help view. the old a/c/s/h terminal rhythm is back."
        self.refresh_view()

    def action_cursor_down(self) -> None:
        self.store.select_next()
        self.mode = ViewMode.SHOP
        self._status_message = f"selected {self.store.selected_product.name.lower()}."
        self.refresh_view()

    def action_cursor_up(self) -> None:
        self.store.select_previous()
        self.mode = ViewMode.SHOP
        self._status_message = f"selected {self.store.selected_product.name.lower()}."
        self.refresh_view()

    def action_add_selected(self) -> None:
        product = self.store.selected_product
        quantity = self.store.add_selected_to_cart()
        self.mode = ViewMode.SHOP
        if quantity >= product.stock:
            self._status_message = f"{product.name.lower()} is capped at in-stock quantity ({product.stock})."
        else:
            self._status_message = f"added {product.name.lower()} to cart. quantity: {quantity}."
        self.refresh_view()

    def action_remove_selected(self) -> None:
        product = self.store.selected_product
        quantity = self.store.remove_selected_from_cart()
        self.mode = ViewMode.CART
        if quantity == 0:
            self._status_message = f"removed {product.name.lower()} from the cart."
        else:
            self._status_message = f"reduced {product.name.lower()} to quantity {quantity}."
        self.refresh_view()

    def action_toggle_theme(self) -> None:
        self.dark = not self.dark
        palette = "dark" if self.dark else "light"
        self._status_message = f"switched to {palette} mode."
        self.refresh_view()

    def action_clear_cart(self) -> None:
        self.store.clear_cart()
        self.mode = ViewMode.CART
        self._status_message = "cleared the cart."
        self.refresh_view()

    def action_refresh_view(self) -> None:
        self.refresh_view()

    def refresh_view(self) -> None:
        self.query_one("#hero", Static).update(self.render_hero())
        self.query_one("#tabs", Static).update(self.render_tabs())
        self.query_one("#content", Static).update(self.render_content())
        self.query_one("#status", Static).update(self.render_status())
        self.query_one("#bindings", Static).update(self.render_bindings())

    def render_hero(self) -> str:
        merchant = self.store.merchant
        return "\n".join(
            [
                LOGO,
                "",
                "# use the command below to enter the shop from your terminal",
                f"ssh {merchant.name}",
                "",
                f"# host key: {HOST_FINGERPRINT}",
                f"# {merchant.headline}",
                f"# {merchant.promise}",
            ]
        )

    def render_tabs(self) -> Text:
        tabs = {
            ViewMode.PRIVACY: ("a", "privacy"),
            ViewMode.CART: ("c", "cart"),
            ViewMode.SHOP: ("s", "shop"),
            ViewMode.HELP: ("h", "help"),
        }
        text = Text(justify="center")
        for mode in (ViewMode.PRIVACY, ViewMode.CART, ViewMode.SHOP, ViewMode.HELP):
            key, label = tabs[mode]
            if text:
                text.append("   ")
            if self.mode == mode:
                text.append("▶ ", style=f"bold {ACCENT}")
                text.append(key, style=f"bold {ACCENT}")
                text.append(f" {label}", style=f"bold {TEXT}")
                text.append(" ◀", style=f"bold {ACCENT}")
            else:
                text.append("· ", style=MUTED)
                text.append(key, style=f"bold {CYAN}")
                text.append(f" {label}", style=MUTED)
        return text

    def render_shop(self) -> str:
        selected = self.store.selected_product
        lines = [
            f"mode      : {self.mode.value}",
            f"merchant  : {self.store.merchant.name}",
            f"location  : {self.store.merchant.location}",
            "",
            "inventory",
            "browse with j/k. press enter to add the selected item to the cart.",
            "",
        ]
        for index, product in enumerate(self.store.products):
            marker = ">>>" if index == self.store.selected_index else " · "
            in_cart = self.store.cart_quantity_for(product.sku)
            suffix = f" | in cart {in_cart}" if in_cart else ""
            lines.append(
                f"{marker} {index + 1}. {product.name.lower()} :: {format_price_sats(product.price_sats)}{suffix}"
            )
            lines.append(f"     [{product.category}] {product.tagline.lower()}")
            lines.append("")
        lines.extend(
            [
                "selected product",
                f"- {selected.description.lower()}",
                f"- features: {', '.join(feature.lower() for feature in selected.features)}",
            ]
        )
        return "\n".join(lines)

    def render_cart(self) -> str:
        lines = [
            f"mode      : {self.mode.value}",
            f"merchant  : {self.store.merchant.name}",
            "",
            "cart",
            "cart is local state for now. payment flow is intentionally not faked here.",
            "",
        ]
        cart_lines = self.store.cart_lines()
        if not cart_lines:
            lines.append("cart is empty. add a product from the shop view.")
        else:
            for line in cart_lines:
                lines.append(
                    f"{line.quantity} x {line.product.name.lower()} = {format_price_sats(line.line_total_sats)}"
                )
            lines.extend(
                [
                    "",
                    f"items : {self.store.cart_items_count()}",
                    f"total : {format_price_sats(self.store.cart_total_sats())}",
                    "",
                    "next",
                    "- wire this into a real checkout pane",
                    "- generate a lightning invoice",
                    "- preserve the operator-first terminal feel",
                ]
            )
        return "\n".join(lines)

    def render_privacy(self) -> str:
        return "\n".join(
            [
                f"mode      : {self.mode.value}",
                f"merchant  : {self.store.merchant.name}",
                "",
                "privacy",
                "this shop is being rebuilt around privacy-first assumptions:",
                "",
                "- no browser dependency for the primary buying flow",
                "- no adtech, pixels, or behavioural profiling",
                "- minimal merchant data collection",
                "- bitcoin and lightning as first-class payment rails",
                "- ssh delivery as a product feature, not a gimmick",
                "",
                "the old rust prototype proved the vibe.",
                "this rewrite is about making the interface tighter, easier to extend,",
                "and credible for people who actually care about privacy.",
            ]
        )

    def render_help(self) -> str:
        return "\n".join(
            [
                f"mode      : {self.mode.value}",
                f"merchant  : {self.store.merchant.name}",
                "",
                "help",
                "the selector is the full-width box at the bottom.",
                "",
                "- a opens privacy",
                "- c opens cart",
                "- s opens shop",
                "- h opens help",
                "- j / k move the product cursor",
                "- enter adds the selected product",
                "- x removes one unit of the selected product",
                "- ctrl+c clears the cart in this local preview",
                "- q exits",
            ]
        )

    def render_content(self) -> str:
        if self.mode == ViewMode.CART:
            return self.render_cart()
        if self.mode == ViewMode.PRIVACY:
            return self.render_privacy()
        if self.mode == ViewMode.HELP:
            return self.render_help()
        return self.render_shop()

    def render_selector_line(self) -> Text:
        text = Text()
        text.append("selector  ", style=f"bold {MUTED}")
        for index, product in enumerate(self.store.products, start=1):
            if index > 1:
                text.append("  //  ", style=MUTED)
            label = f"{index}:{product.name.lower()}"
            if index - 1 == self.store.selected_index:
                text.append("▶ ", style=f"bold {ACCENT}")
                text.append(label, style=f"bold {TEXT}")
                text.append(" ◀", style=f"bold {ACCENT}")
            else:
                text.append("▷ ", style=CYAN)
                text.append(label, style=CYAN)

        return text

    def render_current_line(self) -> Text:
        selected = self.store.selected_product
        text = Text()
        text.append("current   ", style=f"bold {MUTED}")
        text.append(
            f"{self.store.selected_index + 1}/{len(self.store.products)} ",
            style=f"bold {ACCENT}",
        )
        text.append(selected.name.lower(), style=f"bold {TEXT}")
        text.append(f"  [{selected.category}]  ", style=CYAN)
        text.append(format_price_sats(selected.price_sats), style=f"bold {ACCENT}")
        text.append(f"  stock {selected.stock}", style=DIM)
        return text

    def render_message_line(self) -> Text:
        text = Text()
        text.append("status    ", style=f"bold {MUTED}")
        text.append(self._status_message, style=DIM)
        return text

    def render_status(self) -> Group:
        return Group(
            self.render_selector_line(),
            self.render_current_line(),
            self.render_message_line(),
        )

    def render_bindings(self) -> Text:
        items = [
            ("q", "quit"),
            ("a", "privacy"),
            ("c", "cart"),
            ("s", "shop"),
            ("h", "help"),
            ("j/k", "move"),
            ("enter", "add"),
            ("x", "remove"),
            ("ctrl+c", "clear cart"),
        ]
        text = Text(justify="center")
        for key, label in items:
            if text:
                text.append("   ")
            text.append(f" {key} ", style=f"bold black on {ACCENT}")
            text.append(f" {label}", style=CYAN)
        return text


def run(merchant_name: str | None = None) -> None:
    """Start the Textual storefront app."""

    store = StoreState(merchant=demo_merchant(merchant_name), products=demo_catalog())
    StorefrontApp(store).run()
