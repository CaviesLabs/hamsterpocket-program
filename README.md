<p align="center">
  <a style="background: black; display: block; border-radius: 8px; padding: 4px" href="http://id.ancient8.gg/" target="blank"><img src="https://cavies.xyz/assets/images/older-hamster.png" width="320" alt="Nest Logo" /></a>
</p>


## Description

**HamsterPocket** Rust program repository.


## Prerequisites

1/ Follow this instructions to install Solana Cli and Anchor Cli (https://book.anchor-lang.com/getting_started/installation.html)

2/ Install the latest nodejs env

## Getting Started

```bash
$ yarn install
```

## Get new address of program
```bash
anchor build
solana address -k target/deploy/pocket-keypair.json
# above program will output the address of program
# replace all `BW5RwMCPY85ch6efYE3Ev43ZQpJytvvjSNbJ2beC9MzV` with your new address
```
## Deploy swap program onto devnet/mainnet

1/ Deploy devnet 

Deploy

```bash
$  anchor deploy --program-name pocket --provider.cluster devnet --provider.wallet ~/.config/solana/id.json
```

Upgrade

```bash
$ anchor upgrade target/deploy/pocket.so --program-id BW5RwMCPY85ch6efYE3Ev43ZQpJytvvjSNbJ2beC9MzV --provider.cluster devnet --provider.wallet ~/.config/solana/id.json
```

2/ Deploy mainnet 

Deploy

```bash
$  anchor deploy --program-name pocket --provider.cluster mainnet-beta --provider.wallet ~/.config/solana/id.json
```

Upgrade

```bash
$ anchor upgrade target/deploy/pocket.so --program-id BW5RwMCPY85ch6efYE3Ev43ZQpJytvvjSNbJ2beC9MzV --provider.cluster mainnet-beta --provider.wallet ~/.config/solana/id.json
```



3/ Upgrade

## Test
Have to run the solana-test-validator separately
```bash
$  solana-test-validator --no-bpf-jit --reset
```

Then 
```bash
$ anchor test --skip-local-validator
```

```txt
   pocket_registry
    ✔ [initialize_swap_program] should: deployer initializes pocket registry successfully
    ✔ [initialize_swap_program] should: deployer fails to re-initialize the pocket registry
    ✔ [update_operator] should: deployer can update operators list (453ms)

  pocket
    ✔ [create_pocket] should: anyone can create their pocket (477ms)
    ✔ [create_token_vault] should: pocket owner can create token vault successfully (473ms)
    ✔ [pause_pocket] should: owner should pause pocket successfully (469ms)
    ✔ [pause_pocket] should: owner should not pause pocket that was already paused
    ✔ [restart_pocket] should: owner can restart the paused pocket successfully (436ms)
    ✔ [close_pocket] should: owner can close pocket successfully (470ms)

  assets
    ✔ [deposit] should: owner can deposit assets to pocket successfully (485ms)
    ✔ [withdraw] should: owner can withdraw assets from pocket successfully (935ms)
```