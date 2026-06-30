"""YAML catalog loader for ShellShop."""

from __future__ import annotations

import yaml
from typing import Any

from .catalog import MerchantProfile, Product


def load_yaml_catalog(path: str) -> tuple[MerchantProfile, list[Product]]:
    """Load and validate a merchant profile and catalog from a YAML file."""
    
    with open(path, "r", encoding="utf-8") as f:
        try:
            data = yaml.safe_load(f)
        except yaml.YAMLError as e:
            raise ValueError(f"Failed to parse YAML configuration: {e}")
            
    if not isinstance(data, dict):
        raise ValueError("Root of YAML configuration must be a dictionary.")

    merchant = _parse_merchant(data.get("merchant"))
    products = _parse_catalog(data.get("catalog"))

    return merchant, products


def _parse_merchant(data: Any) -> MerchantProfile:
    if not isinstance(data, dict):
        raise ValueError("Missing or invalid 'merchant' section in configuration.")

    required_fields = ["name", "headline", "location", "promise"]
    for field in required_fields:
        if field not in data:
            raise ValueError(f"Merchant profile missing required field: '{field}'")
        if not isinstance(data[field], str):
            raise ValueError(f"Merchant profile field '{field}' must be a string")

    return MerchantProfile(
        name=data["name"],
        headline=data["headline"],
        location=data["location"],
        promise=data["promise"],
    )


def _parse_catalog(data: Any) -> list[Product]:
    if not isinstance(data, list):
        raise ValueError("Missing or invalid 'catalog' section, must be a list.")
        
    products = []
    for i, prod_data in enumerate(data):
        if not isinstance(prod_data, dict):
            raise ValueError(f"Product at index {i} must be a dictionary.")
            
        identifier = prod_data.get("sku") or f"index {i}"

        # Required string fields
        for field in ["sku", "name"]:
            if field not in prod_data:
                raise ValueError(f"Product '{identifier}' missing required field: '{field}'")
            if not isinstance(prod_data[field], str):
                raise ValueError(f"Product '{identifier}' field '{field}' must be a string")
                
        # Optional string fields
        for field in ["tagline", "description", "category"]:
            val = prod_data.get(field)
            if val is None:
                prod_data[field] = ""
            elif not isinstance(val, str):
                raise ValueError(f"Product '{identifier}' field '{field}' must be a string or null")
                
        # Integer fields
        for field in ["price_sats", "stock"]:
            if field not in prod_data:
                raise ValueError(f"Product '{identifier}' missing required field: '{field}'")
            if not isinstance(prod_data[field], int):
                raise ValueError(f"Product '{identifier}' field '{field}' must be an integer")
                
        # List of strings field (optional)
        features_data = prod_data.get("features")
        if features_data is None:
            features_data = []
        elif not isinstance(features_data, list):
            raise ValueError(f"Product '{identifier}' field 'features' must be a list of strings")
        for j, feature in enumerate(features_data):
            if not isinstance(feature, str):
                raise ValueError(f"Product '{identifier}' feature at index {j} must be a string")
                
        products.append(
            Product(
                sku=prod_data["sku"],
                name=prod_data["name"],
                tagline=prod_data["tagline"],
                description=prod_data["description"],
                category=prod_data["category"],
                price_sats=prod_data["price_sats"],
                stock=prod_data["stock"],
                features=tuple(features_data),
            )
        )
        
    return products
