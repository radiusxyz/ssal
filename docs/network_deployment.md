# Walkthrough

## Prerequisites (Ubuntu 22.04 Jammy)
1. Install rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Install Go according to the instruction from the link: https://go.dev/doc/install

3. Install dependencies
```
# Install libraries.
sudo apt install build-essential clang libssl-dev -y

# Install foundry
curl -L https://foundry.paradigm.xyz | bash
foundryup

# Install zap-pretty
go install github.com/maoueh/zap-pretty@latest
```

4. Clone the SSAL repositories
```
git clone --recursive https://github.com/radiusxyz/ssal
```

5. Build
```
cd ssal
cargo build --release
```

## Deployment
Please follow the instructions below in order.

#### 1. Run the operator
In order to run the operator we need to change configurations at:
```
ssal/incredible-squaring-avs/config-files/operator.anvil.yaml
```

***Caveat***
`operator.anvil.yaml` file needs to change if the operator is already registered to the aggregator.

**Cold Start**
Open the file with the editor of your choice and change `eth_rpc_url`, `eth_ws_url`, `aggregator_server_ip_port_address` and `register_operator_on_startup` to followings:
```
eth_rpc_url: http://3.38.183.158:8545
eth_ws_url: ws://3.38.183.158:8545

aggregator_server_ip_port_address: 3.38.183.158:8090

register_operator_on_startup: true
```

**Re-run**
When a validator wants to re-run the operator after the successful registration of
the operator, open the file and change the `register_operator_on_startup` to the following:
```
register_operator_on_startup: false
```

Change the directory to `/ssal/incredible-squaring-avs` and run the following command:
```
make start-operator
```

#### 2. Run the sequencer
On a separate terminal, we will launch the sequencer for rollups that have already been deployed.
Currently, there are 3 rollups whose IDs are "1", "2" and "3" respectively.

Change the directory to `/ssal` and run one of the following examples:
```
# Usage:
./target/release/ssal-sequencer "SEQUENCER-ADDRESS" "ROLLUP-ID" "SSAL-URL" "CHAIN-URL" "WALLET-PRIVATE-KEY"

# Examples
// The sequencer for Rollup ID = 1, listening to 0.0.0.0:8001
./target/release/ssal-sequencer 0.0.0.0:8001 1 http://3.38.183.158:3000 http://3.38.183.158:8545 59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d

// The sequencer for Rollup ID = 2, listening to 0.0.0.0:8002
./target/release/ssal-sequencer 0.0.0.0:8002 2 http://3.38.183.158:3000 http://3.38.183.158:8545 59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d

// The sequencer for Rollup ID = 3, listening to 0.0.0.0:8003
./target/release/ssal-sequencer 0.0.0.0:8003 3 http://3.38.183.158:3000 http://3.38.183.158:8545 59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d
```

After a successful launch, the following log will show up:
```
INFO ssal_sequencer::task: [RegisterSequencer]: Successfully registered for RollupId("1"): BlockHeight(192)
```

#### 3. Run the client
Now we can run the test-client to send transactions to rollups.
```
# Usage:
./target/release/ssal-client "SSAL-URL" "ROLLUP-ID"

# Examples:
// Run test-client for Rollup ID = 1
./target/release/ssal-client http://3.38.183.158:3000 1

// Run test-client for Rollup ID = 2
./target/release/ssal-client http://3.38.183.158:3000 2

// Run test-client for Rollup ID = 3
./target/release/ssal-client http://3.38.183.158:3000 3
```

#### 4. Query using a web browser.
Now everything is up and running, we can query things using a web browser.

Query the block:
```
# Usage:
"SEQUENCER-URL"/get-block?rollup_id="ROLLUP-ID"&block_height="BLOCK-HEIGHT"

# Examples:
// Get the block with height = 48 for Rollup ID = 1 from the sequencer listening to port 8001.
http://127.0.0.1:8001/get-block?rollup_id=1&block_height=50

// Get the block with height = 96 for Rollup ID = 2 from the seqencer listening to port 8003.
http://127.0.0.1:8003/get-block?rollup_id=2&block_height=96
```

Query the block commitment:
```
# Usage:
"SEQUENCER-URL"/get-block-commitment?rollup_id="ROLLUP-ID"&block_height="BLOCK-HEIGHT"

# Examples:
// Get the block commitment of the block with height = 50 for Rollup ID = 1 from the sequencer listening to port 8001.
http://127.0.0.1:8001/get-block-commitment?rollup_id=1&block_height=50

// Get the block commitment of the block with height = 96 for Rollup ID = 2 from the sequencer listening to port 8003.
http://127.0.0.1:8003/get-block-commitment?rollup_id=2&block_height=96
```

Query the sequencer set:
```
# Usage:
"SSAL-URL"/get-closed-sequencer-set?rollup_id="ROLLUP-ID"&block_height="BLOCK-HEIGHT"

# Examples:
// Get the sequencer set for the block with height = 48 for Rollup ID = 1.
http://3.38.183.158:3000/get-closed-sequencer-set?rollup_id=1&block_height=48

// Get the sequencer set for the block with height = 96 for Rollup ID = 2.
http://3.38.183.158:3000/get-closed-sequencer-set?rollup_id=2&block_height=96
```
