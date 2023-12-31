# Coderty

| Contract      | Address                                    |
| ------------- | ------------------------------------------ |
| Contributions | 0x5fbdb2315678afecb367f032d93f642f64180aa3 |
| Project       | 0xe7f1725e7734ce288f8367e1bb143e90bb3f0512 |

## Pre-requisites

Set `PRIVATE_KEY` and `RPC_URL` environment variables in `.env`. For local Anvil blockchain you can use

``` text
PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
RPC_URL=http://127.0.0.1:8545
```

- Nix
- direnv

## Build

Install Solidity dependencies:

```sh
forge install
```

Compile contracts:

```sh
forge build
```

Compile contracts and generate Rust bindings for the CLI:

```sh
bind
```

Build the CLI:

```sh
cargo build
```

Install the CLI:

```sh
cargo install
```
