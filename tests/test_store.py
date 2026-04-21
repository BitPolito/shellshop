"""Behavioral tests for the core storefront state."""

from __future__ import annotations

import unittest

from shellshop.catalog import demo_catalog, demo_merchant
from shellshop.store import StoreState


class StoreStateTests(unittest.TestCase):
    def setUp(self) -> None:
        self.store = StoreState(demo_merchant(), demo_catalog())

    def test_selection_wraps_forward(self) -> None:
        for _ in range(len(self.store.products)):
            self.store.select_next()
        self.assertEqual(self.store.selected_index, 0)

    def test_selection_wraps_backward(self) -> None:
        self.store.select_previous()
        self.assertEqual(self.store.selected_index, len(self.store.products) - 1)

    def test_add_to_cart_caps_at_stock(self) -> None:
        product = self.store.selected_product
        quantity = self.store.add_to_cart(product.sku, product.stock + 10)
        self.assertEqual(quantity, product.stock)

    def test_remove_from_cart_clears_line(self) -> None:
        product = self.store.selected_product
        self.store.add_to_cart(product.sku, 2)
        updated = self.store.remove_from_cart(product.sku, 2)
        self.assertEqual(updated, 0)
        self.assertEqual(self.store.cart_items_count(), 0)

    def test_cart_total_matches_line_items(self) -> None:
        first = self.store.products[0]
        second = self.store.products[1]
        self.store.add_to_cart(first.sku, 2)
        self.store.add_to_cart(second.sku, 1)
        expected = first.price_sats * 2 + second.price_sats
        self.assertEqual(self.store.cart_total_sats(), expected)


if __name__ == "__main__":
    unittest.main()
