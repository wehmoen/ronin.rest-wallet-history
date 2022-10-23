# Ronin.Rest Wallet History 

This serves as the backend api for the ronin.rest wallet history api.

## Requirements

1. MongoDB running at 127.0.0.1:27017
2. Ronin HTTP RPC running at 127.0.0.1:8545

## Setup

```
git clone https://github.com/wehmoen/ronin.rest-wallet-history.git
cd ronin.rest-wallet-history
cargo run -r
```

## Usage:

```
http://localhost:9090/archive/wallet_history/WALLET_ADDRESS/ERC20_TOKEN_ADDRESS/BLOCK
```