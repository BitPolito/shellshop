"""Pure-Python storefront state used by the Textual UI."""

from __future__ import annotations

from dataclasses import dataclass

from .catalog import MerchantProfile, Product


@dataclass(frozen=True, slots=True)
class CartLine:
    """Rendered cart line item."""

    product: Product
    quantity: int

    @property
    def line_total_sats(self) -> int:
        return self.product.price_sats * self.quantity


class StoreState:
    """State container for catalog browsing and cart interactions."""

    def __init__(self, merchant: MerchantProfile, products: list[Product]) -> None:
        if not products:
            raise ValueError("StoreState requires at least one product")
        self.merchant = merchant
        self.products = products
        self.selected_index = 0
        self._cart: dict[str, int] = {}

    @property
    def selected_product(self) -> Product:
        return self.products[self.selected_index]

    def select_next(self) -> Product:
        self.selected_index = (self.selected_index + 1) % len(self.products)
        return self.selected_product

    def select_previous(self) -> Product:
        self.selected_index = (self.selected_index - 1) % len(self.products)
        return self.selected_product

    def add_selected_to_cart(self, quantity: int = 1) -> int:
        return self.add_to_cart(self.selected_product.sku, quantity)

    def add_to_cart(self, sku: str, quantity: int = 1) -> int:
        if quantity < 1:
            raise ValueError("quantity must be positive")
        product = self.product_by_sku(sku)
        updated = min(product.stock, self._cart.get(sku, 0) + quantity)
        self._cart[sku] = updated
        return updated

    def remove_selected_from_cart(self, quantity: int = 1) -> int:
        return self.remove_from_cart(self.selected_product.sku, quantity)

    def remove_from_cart(self, sku: str, quantity: int = 1) -> int:
        if quantity < 1:
            raise ValueError("quantity must be positive")
        current = self._cart.get(sku, 0)
        updated = max(0, current - quantity)
        if updated == 0:
            self._cart.pop(sku, None)
        else:
            self._cart[sku] = updated
        return updated

    def clear_cart(self) -> None:
        self._cart.clear()

    def product_by_sku(self, sku: str) -> Product:
        for product in self.products:
            if product.sku == sku:
                return product
        raise KeyError(f"unknown product sku: {sku}")

    def cart_quantity_for(self, sku: str) -> int:
        return self._cart.get(sku, 0)

    def cart_lines(self) -> list[CartLine]:
        return [
            CartLine(product=self.product_by_sku(sku), quantity=quantity)
            for sku, quantity in self._cart.items()
        ]

    def cart_items_count(self) -> int:
        return sum(self._cart.values())

    def cart_total_sats(self) -> int:
        return sum(line.line_total_sats for line in self.cart_lines())
