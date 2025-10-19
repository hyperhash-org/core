HYPER HASH CORE
Repository: hyperhash-core
License: GNU Affero General Public License v3.0 (AGPL-3.0)
Visibility: Public

OVERVIEW

Hyper Hash Core is the central control layer of the Hyper Hash Network.
It manages the coordination, accounting, and governance of all connected nodes, miners, and treasury operations.

This component acts as the network’s “brain,” providing authenticated APIs for pool management, node registration, payout scheduling, and network telemetry.
All other subsystems (pool, node agents, template providers, and treasury) connect to Core for authorization and data flow.

KEY FUNCTIONS

Share Accounting

Validates and aggregates shares submitted by the pool or translators.

Maintains rolling and historical hash rate statistics per miner and node.

Supports payout share weighting and difficulty adjustments.

Node Directory and Eligibility Tracking

Tracks all registered Hyper Nodes (public and private).

Monitors node uptime, channel status, and Lightning wallet connectivity.

Determines node eligibility for reward distribution.

Payout Manifest Generation

Calculates miner share allocations per block.

Prepares signed manifests for the Treasury to execute Lightning payouts.

Implements 1% pool fee split (0.5% Hyper Hash, 0.5% distributed among eligible nodes).

API Services

REST and gRPC endpoints for telemetry, node registration, and mining statistics.

Provides JSON API to the web UI and public dashboards.

Exposes admin endpoints for internal orchestration and monitoring.

Treasury Hooks

Sends payout manifests and confirmation signals to hyperhash-treasury.

Logs reconciliation events for every successful or failed payout.

Maintains accounting parity between on-chain and off-chain balances.

Job Management

Coordinates template provider updates for Stratum v2 jobs.

Integrates with the hyperhash-tp service to push candidate block templates to the pool.

Tracks job timing, propagation, and acceptance ratios.

Governance and Fee Control

Defines and enforces pool fee policies.

Handles treasury funding, operational reserves, and node network incentives.

Can issue signed governance messages for future protocol upgrades.

ARCHITECTURE SUMMARY

Written in Rust with modular service architecture.

Uses PostgreSQL for persistence and metrics storage.

Communicates with Pool, Translator, and Treasury via gRPC and WebSocket streams.

Includes telemetry hooks for Prometheus, Grafana, and external dashboards.

Built with security-first principles: key-based authentication, HMAC-signed messages, and encrypted inter-service traffic.

COMPONENT STRUCTURE

src/
api/ REST + gRPC endpoints
db/ Database schema and migrations
ledger/ Accounting and payout logic
treasury/ Treasury interface layer
nodes/ Node directory and status service
metrics/ Prometheus metrics and telemetry
jobs/ Job control and template coordination
auth/ Token and signature verification
config/
core.toml Default configuration file
secrets.env Environment variables template
tests/
integration/ Integration test suites
unit/ Module-level tests

CONFIGURATION

Configuration file: /etc/hyperhash/core.toml

Key parameters:

rpc_bind: 127.0.0.1:8800

db_url: postgresql://hyperhash:password@localhost/hh_core

treasury_endpoint: http://127.0.0.1:8810

pool_endpoint: http://127.0.0.1:34254

metrics_port: 9100

auth_public_key: /etc/hyperhash/public.pem

All credentials and keys are stored in /etc/hyperhash/secrets.env (never in Git).

BUILD AND INSTALLATION

Dependencies:

Rust stable (1.75 or newer)

PostgreSQL 15+

OpenSSL development headers

Git and pkg-config

Build steps:

git clone https://github.com/hyperhash-org/hyperhash-core.git

cd hyperhash-core

cargo build --release

sudo install -m 0755 target/release/hh-core /usr/local/bin/hh-core

Copy config/core.toml to /etc/hyperhash/core.toml

Enable systemd service (example below)

Systemd unit example:
[Unit]
Description=Hyper Hash Core Service
After=network.target postgresql.service

[Service]
User=hyperhash
ExecStart=/usr/local/bin/hh-core --config /etc/hyperhash/core.toml
Restart=always
RestartSec=10
EnvironmentFile=/etc/hyperhash/secrets.env

[Install]
WantedBy=multi-user.target

TESTING AND VERIFICATION

Local tests:
cargo test

Integration tests (requires running PostgreSQL instance):
cargo test -- --ignored

Metrics verification:
curl http://127.0.0.1:9100/metrics

API verification:
curl http://127.0.0.1:8800/api/status

NETWORK INTERACTIONS

Inbound connections:

From hyperhash-pool (share submission, stats updates)

From hyperhash-node-agent (node registration)

From hyperhash-ui (web queries)

From hyperhash-treasury (payout confirmation)

Outbound connections:

To hyperhash-treasury (payout manifests)

To hyperhash-tp (template updates)

To hyperhash-overlay (future mesh propagation)

SECURITY PRACTICES

All network communication uses mutual TLS or signed JSON.

JWT tokens with short expiry control API access.

All private keys are stored only on the host running Core.

Backup encryption uses AES-256 with rotation every 90 days.

CI/CD pipelines enforce signed commits and reproducible builds.

MAINTAINERS

Lead Architect: @caldefenwycke
Ops Integration: @ops-team
Security & Treasury: @treasury-lead
Contributors: Hyper Hash community

CONTACT

Website: https://hyperhash.net

Discord: https://discord.com/channels/1409546566116839490/1410138982204838009

LICENSE

Copyright © 2025 Hyper Hash Network
Licensed under the GNU Affero General Public License v3.0 (AGPL-3.0)
You may copy, modify, and distribute this software under the terms of that license.
A full copy of the license is available in the LICENSE file.

STATUS

Version: 0.0.1 (Baseline MVP build)
Stage: Planned
Next milestone: Mainnet-ready pool integration with Treasury signing

END OF FILE — README.md (Hyper Hash Core)
