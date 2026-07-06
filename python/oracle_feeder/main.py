"""Entry point: fetch rainfall for each configured location and submit
it to weather-oracle. Intended to be run periodically via cron/systemd
timer rather than as a long-running process — a fresh reading every few
hours is plenty for a rainfall oracle.

Usage:
    python main.py
"""

from __future__ import annotations

import json
import os
from pathlib import Path

from dotenv import load_dotenv

from soroban_client import submit_reading
from weather_client import fetch_rainfall_mm

load_dotenv()

RPC_URL = os.environ["SOROBAN_RPC_URL"]
NETWORK_PASSPHRASE = os.environ["NETWORK_PASSPHRASE"]
CONTRACT_ID = os.environ["CONTRACT_ID"]
PROVIDER_SECRET = os.environ["PROVIDER_SECRET"]

LOCATIONS_FILE = Path(__file__).parent / "locations.json"


def main() -> None:
    with open(LOCATIONS_FILE) as f:
        locations = json.load(f)

    for loc in locations:
        symbol = loc["location_symbol"]
        try:
            rainfall_mm = fetch_rainfall_mm(loc["lat"], loc["lon"])
            print(f"{loc['name']} ({symbol}): {rainfall_mm}mm over the past 7 days")

            tx_hash = submit_reading(
                rpc_url=RPC_URL,
                network_passphrase=NETWORK_PASSPHRASE,
                contract_id=CONTRACT_ID,
                provider_secret=PROVIDER_SECRET,
                location_symbol=symbol,
                value_mm=rainfall_mm,
            )
            print(f"  submitted on-chain: {tx_hash}")
        except Exception as exc:  # noqa: BLE001 — top-level script, log and continue to next location
            print(f"  FAILED for {symbol}: {exc}")


if __name__ == "__main__":
    main()
