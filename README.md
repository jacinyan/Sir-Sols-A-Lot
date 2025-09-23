# Sir-Sols-A-Lot

Automated Solana airdrop farming system. Uses multiple wallets to interact with DeFi protocols and collect potential airdrops.

## How it works

Most new Solana projects give out airdrops to early users, but they cap rewards per wallet. So instead of using one big wallet, we use hundreds of small ones to get around the limits.

The system manages all these wallets automatically - creating them, sending transactions, and collecting rewards when airdrops happen.

## Project layout

```
server/
├── engine/          # Core wallet and transaction handling
├── task_scheduler/  # Job scheduling and automation
└── src/main.rs     # Main server

dashboard/          # Web interface (not built yet)
```

Everything backend is Rust for performance. Frontend will be Next.js.

## Current status

- Wallet management is working (can create/manage hundreds of wallets)
- Transaction builder is stubbed out but not implemented yet
- Task scheduler is stubbed out but not implemented yet
- No frontend yet

## Running

```bash
cd server
cargo build
cargo test
```

## Architecture notes

The wallet manager can organize wallets into "squads" for different farming campaigns. Each squad gets assigned to farm a specific protocol.

When a new airdrop opportunity comes up, you create a squad, assign wallets to it, and let the system run automatically.

The transaction builder will handle creating the actual Solana transactions for different DeFi protocols like Kamino, MarginFi, etc.

The scheduler makes sure everything runs 24/7 without manual intervention.