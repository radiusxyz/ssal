# Walkthrough

## Prerequisites
`cargo build --release`

## SSAL
Usage:

`./target/release/ssal "SSAL-ADDRESS"`

Example:

`./target/release/ssal 127.0.0.1:3000`

## SSAL-ROLLUP
Usage:

`./target/release/ssal-rollup "ROLLUP-ID (UNSIGNED INT)" "SSAL-URL"`

Example:
`./target/release/ssal-rollup 1234 http://127.0.0.1:3000`

## SSAL-SEQUENCER
Usage:

`./target/release/ssal-sequencer "SEQUENCER-ADDRESS" "ROLLUP-ID (UNSIGNED INT)" "SSAL-URL" "CHAIN-URL" "WALLET-PRIVATE-KEY"`

Example:
`./target/release/ssal-sequencer 127.0.0.1:8000 1234 http://127.0.0.1:3000 http://127.0.0.1:9999 59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d`
