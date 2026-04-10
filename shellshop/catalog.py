"""Demo merchant and catalog data for the Textual storefront."""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True, slots=True)
class Product:
    """Single storefront product."""

    sku: str
    name: str
    tagline: str
    description: str
    category: str
    price_sats: int
    stock: int
    features: tuple[str, ...]


@dataclass(frozen=True, slots=True)
class MerchantProfile:
    """Merchant-level branding shown in the dashboard."""

    name: str
    headline: str
    location: str
    promise: str


def format_price_sats(value: int) -> str:
    """Render satoshi amounts consistently for the UI."""

    return f"{value:,} sats"


def demo_merchant(name: str | None = None) -> MerchantProfile:
    """Return the default merchant profile."""

    return MerchantProfile(
        name=name or "ShellShop Supply Co.",
        headline="Terminal-native commerce for people who would rather ship software than storefront themes.",
        location="Remote-first merchant collective",
        promise="No browser chrome. No adtech. Just products, intent, and settlement-ready order flows.",
    )


def demo_catalog() -> list[Product]:
    """Return a small demo catalog for local iteration."""

    return [
        Product(
            sku="mesh-node-01",
            name="Mesh Node Starter Kit",
            tagline="Deploy a tiny sovereign edge node in one sitting.",
            description=(
                "A travel-sized compute bundle for operators who want a low-noise way to host "
                "shop services, mirrors, and internal tools."
            ),
            category="hardware",
            price_sats=125_000,
            stock=8,
            features=("fanless enclosure", "pre-flashed image", "dual gigabit ethernet"),
        ),
        Product(
            sku="ln-sign-02",
            name="Lightning Counter Sign",
            tagline="Show invoice state on a monochrome desktop display.",
            description=(
                "An operator-facing desk display that rotates store prompts, order state, and "
                "payment status updates while keeping the aesthetic unapologetically terminal."
            ),
            category="retail ops",
            price_sats=48_000,
            stock=21,
            features=("e-ink panel", "USB-C power", "custom Textual-ready themes"),
        ),
        Product(
            sku="field-manual-03",
            name="Self-Hosting Field Manual",
            tagline="A concise playbook for running private online shops.",
            description=(
                "Practical notes on deployment, backups, secrets handling, and how to keep a "
                "small internet business understandable by one person at 2 a.m."
            ),
            category="knowledge",
            price_sats=9_500,
            stock=999,
            features=("PDF bundle", "checklists", "disaster recovery worksheet"),
        ),
        Product(
            sku="relay-rack-04",
            name="Relay Rack Patch Set",
            tagline="Clean cable management for compact self-hosted racks.",
            description=(
                "A parts bundle for merchants who want their hardware shelf to look deliberate "
                "instead of improvised."
            ),
            category="hardware",
            price_sats=19_000,
            stock=34,
            features=("reusable ties", "label cards", "right-angle patch leads"),
        ),
    ]
