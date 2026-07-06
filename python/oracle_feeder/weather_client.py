"""Fetches real rainfall data from Open-Meteo (open, no API key required).

Open-Meteo's historical/forecast API returns daily precipitation sums in
millimeters, which is exactly the unit weather-oracle expects. See
https://open-meteo.com/en/docs for the full API reference — this module
uses only the `daily=precipitation_sum` field with `past_days`.
"""

from __future__ import annotations

import requests

OPEN_METEO_URL = "https://api.open-meteo.com/v1/forecast"


def fetch_rainfall_mm(lat: float, lon: float, past_days: int = 7) -> int:
    """Returns total rainfall (mm) over the past `past_days`, rounded to
    the nearest whole millimeter (weather-oracle stores integer i128
    values, so sub-mm precision isn't meaningful here).

    Raises requests.RequestException on network/API failure — callers
    should handle retries (see ISSUES.md #6, this is intentionally not
    handled here yet).
    """
    params = {
        "latitude": lat,
        "longitude": lon,
        "daily": "precipitation_sum",
        "past_days": past_days,
        "timezone": "auto",
    }

    response = requests.get(OPEN_METEO_URL, params=params, timeout=15)
    response.raise_for_status()
    data = response.json()

    daily_values = data.get("daily", {}).get("precipitation_sum", [])
    total_mm = sum(v for v in daily_values if v is not None)

    return round(total_mm)
