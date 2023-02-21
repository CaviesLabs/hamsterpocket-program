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
# replace all `DL1BQwRmN4Ye4fsmDnPipNrm1XU24x8yDU7c5Ltsqvvc` with your new address
```
## Deploy swap program onto devnet/mainnet

1/ Deploy devnet 

Deploy

```bash
$  anchor deploy --program-name pocket --provider.cluster devnet --provider.wallet ~/.config/solana/id.json
```

Upgrade

```bash
$ anchor upgrade target/deploy/pocket.so --program-id DL1BQwRmN4Ye4fsmDnPipNrm1XU24x8yDU7c5Ltsqvvc --provider.cluster devnet --provider.wallet ~/.config/solana/id.json
```

2/ Deploy mainnet 

Deploy

```bash
$  anchor deploy --program-name pocket --provider.cluster mainnet-beta --provider.wallet ~/.config/solana/id.json
```

Upgrade

```bash
$ anchor upgrade target/deploy/pocket.so --program-id DL1BQwRmN4Ye4fsmDnPipNrm1XU24x8yDU7c5Ltsqvvc --provider.cluster mainnet-beta --provider.wallet ~/.config/solana/id.json
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
  ✔ [initialize_swap_program] should: deployer initializes swap registry successfully
  ✔ [initialize_swap_program] should: deployer fails to re-initialize the swap registry
  ✔ [update_swap_program] should: deployer updates registry successfully (443ms)
  ✔ [update_swap_program] should: non-owner fails to modify the swap program
  ✔ [update_swap_program] should: deployer fails to update invalid values
  ✔ [create_token_vault] should: non-deployer fails to create a token vault
  ✔ [create_token_vault] should: deployer creates a token vault successfully (446ms)
  ✔ [create_token_vault] should: deployer fails to create a token vault for an added mint account
  ✔ [create_proposal] should: fail to create proposal with un-allowed mint tokens
  ✔ [create_proposal] should: everyone can create publicly a proposal (964ms)
  ✔ [cancel_proposal] should: participants can cancel proposal anytime when proposal isn't fulfilled (930ms)
  ✔ [withdraw_assets] should: participant can withdraw assets when proposal is canceled (452ms)
  ✔ [deposit_assets] should: proposal owner deposits offered items successfully (491ms)
  ✔ [fulfil_assets] should: participant fulfill proposal successfully (447ms)
  ✔ [redeem_assets] should: proposal owner can redeem items once the proposal is fulfilled (459ms)
  ✔ [redeem_assets] should: participant can redeem items once the proposal is fulfilled (461ms)

  16 passing (12s)

```