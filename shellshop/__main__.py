"""CLI entrypoint for running the ShellShop Textual app."""

from __future__ import annotations

import argparse

from . import __version__


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Run the ShellShop Textual storefront.")
    parser.add_argument(
        "--merchant-name",
        help="Override the demo merchant name shown in the storefront.",
    )
    parser.add_argument(
        "--version",
        action="version",
        version=f"%(prog)s {__version__}",
    )
    return parser


def main() -> None:
    args = build_parser().parse_args()
    from .app import run

    run(merchant_name=args.merchant_name)


if __name__ == "__main__":
    main()
