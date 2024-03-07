# Walkthrough

## Prerequisites (Ubuntu 22.04 Jammy)
1. Install rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Install Go according to the instruction from the link: https://go.dev/doc/install

3. Install dependencies
```
sudo apt install build-essential clang libssl-dev -y
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
Please follow the instructions below to deploy and test. ***The order with which you run the nodes matters.***

#### 1. Deploy the local EVM
```
TODO:
```

#### 2. Launch a separate terminal and run `ssal` from the cloned repository path.
Local deployment creates a "database" directory under the cloned repository path. In order to start anew, make sure to delete the "databases" directory before following the rest of the process for launching all the nodes. If you want to simply pick up from where you left off, you may leave the directory.
```
# Usage:
// To start fresh:
rm -rf databases && ./target/release/ssal "SSAL-ENDPOINT"

// Otherwise:
./target/release/ssal "SSAL-ENDPOINT"

# Examples:
rm -rf databases && ./target/release/ssal 127.0.0.1:3000
```
The following log will show up on the terminal for a successful launch.
```
INFO ssal: Starting the server at "127.0.0.1:3000"
```
At this point we can deploy an arbitrary number of `ssal-rollup` and register at `ssal`.

#### 3. Launch a separate terminal and run `ssal-rollup` to register at `ssal`
```
# Usage:
./target/release/ssal-rollup "ROLLUP-ID (Must be an unsigned integer)" "SSAL-URL"

# Examples:
// Deploy the rollup whose "ID = 1"
./target/release/ssal-rollup 1 http://127.0.0.1:3000

// Deploy the rollup whose "ID = 2"
./target/release/ssal-rollup 2 http://127.0.0.1:3000
```

A successful launch will show the following log on the terminal for each corresponding rollup:
```
// Rollup ID = 1
INFO ssal_rollup: [RegisterRollup]: Successfully registered RollupId("1")

// Rollup ID = 2
INFO ssal_rollup: [RegisterRollup]: Successfully registered RollupId("2")
```

If you have not deleted the "database" directory from the previous run, the following log will show up:
```
ERROR ssal_rollup: [RegisterRollup]: "Rollup already exists."
```
Although it says ERROR, the log simply means a rollup with the same ID cannot be registered twice. Don't worry, our rollup will continue to operate as it is supposed to.

After a successful run, a rollup will close the block every 5 seconds. However, because we have not registered any sequencer for our rollups, the following log will show up on the terminal:
```
// Rollup ID = 1
[CloseBlock]: The value returned None ("key: (\"sequencer_set\", RollupId(\"1\"), BlockHeight(1))")

// Rollup ID = 2
[CloseBlock]: The value returned None ("key: (\"sequencer_set\", RollupId(\"2\"), BlockHeight(1))")
```

Now, let's move onto launching our sequencers and registering them at our rollups.

#### 4. Launch a separate terminal and run `ssal-sequencer` 
We will launch two sequencers for each rollup we have previously deployed.
```
# Usage:
./target/release/ssal-sequencer "SEQUENCER-ADDRESS" "ROLLUP-ID" "SSAL-URL" "CHAIN-URL" "WALLET-PRIVATE-KEY"

# Examples
// The first sequencer for Rollup ID = 1
./target/release/ssal-sequencer 127.0.0.1:8001 1 http://127.0.0.1:3000 http://127.0.0.1:8545 59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d

// The second sequencer for Rollup ID = 1
./target/release/ssal-sequencer 127.0.0.1:8002 1 http://127.0.0.1:3000 http://127.0.0.1:8545 59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d

// The third sequencer for Rollup ID = 2
./target/release/ssal-sequencer 127.0.0.1:8003 2 http://127.0.0.1:3000 http://127.0.0.1:8545 59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d

// The fourth sequencer for Rollup ID = 2
./target/release/ssal-sequencer 127.0.0.1:8004 2 http://127.0.0.1:3000 http://127.0.0.1:8545 59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d
```

After a successful launch, the following log will show up:
```
INFO ssal_sequencer::task: [RegisterSequencer]: Successfully registered for RollupId("1"): BlockHeight(192)
```

At this point, our sequencers are successfully registered at each rollup's sequencer pool and randomly become a leader to build a block. Because we are not sending any transaction to the sequencer, the block is empty now. Let's move onto launching test clients to send transactions to be included in our rollups.

#### 5. Launch a separate terminal and start `ssal-client`
```
# Usage:
./target/release/ssal-client "SSAL-URL" "ROLLUP-ID"

# Examples:
// Run test-client for Rollup ID = 1
./target/release/ssal-client http://127.0.0.1:3000 1

// Run test-client for Rollup ID = 2
./target/release/ssal-client http://127.0.0.1:3000 2
```

After a successful launch, the client will emit the following log every 200 milliseconds:
```
INFO ssal_client: Some(OrderCommitment { block_height: BlockHeight(282), tx_order: TransactionOrder(23) })
```

#### 6. Query using a web browser.
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
http://127.0.0.1:3000/get-closed-sequencer-set?rollup_id=1&block_height=48

// Get the sequencer set for the block with height = 96 for Rollup ID = 2.
http://127.0.0.1:3000/get-closed-sequencer-set?rollup_id=2&block_height=96
```
