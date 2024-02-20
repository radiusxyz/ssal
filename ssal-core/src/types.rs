use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockMetadata {
    height: BlockHeight,
    status: BlockStatus,
}

impl Default for BlockMetadata {
    fn default() -> Self {
        Self {
            height: BlockHeight::from(1),
            status: BlockStatus::Registration,
        }
    }
}

impl BlockMetadata {
    pub fn get_height(&self) -> BlockHeight {
        self.height.clone()
    }

    pub fn get_status(&self) -> BlockStatus {
        self.status.clone()
    }

    pub fn close_block(&mut self) {
        self.height.increment();
        self.status = BlockStatus::Registration;
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockHeight(usize);

impl From<usize> for BlockHeight {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl BlockHeight {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum BlockStatus {
    Registration,
    BuildingInProgress,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RollupId(String);

impl From<&str> for RollupId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for RollupId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SequencerId(String);

impl From<&str> for SequencerId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for SequencerId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SequencerPool(HashSet<SequencerId>);

impl SequencerPool {
    pub fn add(&mut self, sequencer_id: SequencerId) -> Result<(), Error> {
        match self.0.insert(sequencer_id) {
            true => Ok(()),
            false => Err(Error::from("Sequencer is already registered")),
        }
    }
}
