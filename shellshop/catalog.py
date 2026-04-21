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
        name=name or "shellshop",
        headline="terminal-native gear for bitcoiners, tinkerers, and people who would rather ssh than click.",
        location="self-hosted merchant relay",
        promise="no trackers. no browser sludge. no surveillance defaults. just products, sats, and quiet software.",
    )


def demo_catalog() -> list[Product]:
    """Return a small demo catalog for local iteration."""

    return [
        Product(
            sku="node-box-01",
            name="Sovereign Node Box",
            tagline="a quiet home for your wallet backend, electrum server, or private relay.",
            description=(
                "a compact operator kit for running personal bitcoin infrastructure without begging "
                "cloud dashboards for permission. built for desks, closets, and low-drama uptime."
            ),
            category="hardware",
            price_sats=210_000,
            stock=7,
            features=("fanless case", "nvme image included", "dual ethernet"),
        ),
        Product(
            sku="jammer-bag-02",
            name="Faraday Travel Roll",
            tagline="keep radios, tags, and casual telemetry a little farther away.",
            description=(
                "a hardened roll-up pouch for phones, keys, and travel documents when you want "
                "fewer chirps, fewer pings, and fewer accidental disclosures."
            ),
            category="privacy gear",
            price_sats=54_000,
            stock=19,
            features=("rf shielded liner", "fold-flat design", "field repair patch"),
        ),
        Product(
            sku="field-guide-03",
            name="Ops Field Guide",
            tagline="notes for staying online, backed up, and difficult to profile.",
            description=(
                "a dense manual on bitcoin ops, backup drills, secret handling, and the boring "
                "habits that keep small sovereign systems alive."
            ),
            category="knowledge",
            price_sats=12_500,
            stock=999,
            features=("pdf + epub", "checklists", "incident worksheet"),
        ),
        Product(
            sku="signer-kit-04",
            name="Airgap Signer Kit",
            tagline="for people who prefer qr codes over browser extensions.",
            description=(
                "a desk-ready signing bundle for offline transaction review, paper backups, and "
                "the kind of setup that still makes sense six months from now."
            ),
            category="bitcoin",
            price_sats=88_000,
            stock=13,
            features=("camera shield", "metal backup cards", "qr stand"),
        ),
    ]
