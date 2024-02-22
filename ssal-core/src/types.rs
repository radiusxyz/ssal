use std::collections::HashSet;

use rand::{self, seq::SliceRandom};
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockHeight(usize);

impl std::cmp::Eq for BlockHeight {}

impl std::cmp::PartialEq<usize> for BlockHeight {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

impl std::cmp::PartialEq<Self> for BlockHeight {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl std::cmp::PartialEq<&Self> for BlockHeight {
    fn eq(&self, other: &&Self) -> bool {
        self.0 == other.0
    }
}

impl std::ops::Add<usize> for BlockHeight {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl std::ops::Sub<usize> for BlockHeight {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl From<usize> for BlockHeight {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl BlockHeight {
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum BlockStatus {
    Registration,
    BuildingInProgress,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RollupSet(HashSet<RollupId>);

impl RollupSet {
    pub fn register(&mut self, rollup_id: RollupId) -> Result<(), Error> {
        match self.0.insert(rollup_id) {
            true => Ok(()),
            false => Err(Error::from("Rollup already exists.")),
        }
    }

    pub fn contains(&self, rollup_id: &RollupId) -> bool {
        self.0.contains(rollup_id)
    }

    pub fn deregister(&mut self, rollup_id: &RollupId) -> Result<(), Error> {
        match self.0.remove(rollup_id) {
            true => Ok(()),
            false => Err(Error::from("Rollup already removed.")),
        }
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
pub struct SequencerSet(HashSet<SequencerId>);

impl SequencerSet {
    pub fn register(&mut self, sequencer_id: SequencerId) -> Result<(), Error> {
        match self.0.insert(sequencer_id) {
            true => Ok(()),
            false => Err(Error::from("Sequencer is already registered")),
        }
    }

    pub fn elect_leader(&mut self) -> Result<SequencerId, Error> {
        let sequencer_vec: Vec<SequencerId> = self
            .0
            .iter()
            .map(|sequencer_id| sequencer_id.clone())
            .collect();

        match sequencer_vec.choose(&mut rand::thread_rng()) {
            Some(leader) => Ok(leader.clone()),
            None => Err(Error::from("Failed to elect the leader.")),
        }
    }
}
