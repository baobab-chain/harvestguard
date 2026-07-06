"""Submits a rainfall reading to the weather-oracle Soroban contract
using the official Python Stellar SDK.

IMPORTANT: the Soroban-related methods on stellar-sdk have moved and been
renamed across versions in the past (SorobanServer, scval helpers, and
TransactionBuilder's contract-invocation method have all shifted at
different points). This module is written against stellar-sdk >= 11, but
a contributor picking this up should confirm the exact method names
against whatever version ends up pinned in requirements.txt before
relying on it — see ISSUES.md.
"""

from __future__ import annotations

import time

from stellar_sdk import Keypair, Network, SorobanServer, TransactionBuilder, scval
from stellar_sdk.exceptions import PrepareTransactionException


def submit_reading(
    rpc_url: str,
    network_passphrase: str,
    contract_id: str,
    provider_secret: str,
    location_symbol: str,
    value_mm: int,
) -> str:
    """Submits a reading and returns the transaction hash on success."""

    keypair = Keypair.from_secret(provider_secret)
    server = SorobanServer(rpc_url)
    source_account = server.load_account(keypair.public_key)

    tx = (
        TransactionBuilder(
            source_account,
            network_passphrase=network_passphrase,
            base_fee=100_000,
        )
        .set_timeout(30)
        .append_invoke_contract_function_op(
            contract_id=contract_id,
            function_name="submit_reading",
            parameters=[
                scval.to_address(keypair.public_key),
                scval.to_symbol(location_symbol),
                scval.to_int128(value_mm),
            ],
        )
        .build()
    )

    try:
        prepared_tx = server.prepare_transaction(tx)
    except PrepareTransactionException as exc:
        raise RuntimeError(f"Failed to prepare transaction: {exc}") from exc

    prepared_tx.sign(keypair)
    send_response = server.send_transaction(prepared_tx)

    tx_hash = send_response.hash
    for _ in range(10):
        result = server.get_transaction(tx_hash)
        if result.status == "SUCCESS":
            return tx_hash
        if result.status == "FAILED":
            raise RuntimeError(f"Transaction {tx_hash} failed: {result}")
        time.sleep(1)

    raise TimeoutError(f"Timed out waiting for transaction {tx_hash}")
