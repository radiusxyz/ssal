use std::collections::{hash_set::Iter, HashSet};

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

impl std::fmt::Display for BlockHeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RollupId(String);

impl std::fmt::Display for RollupId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for RollupId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

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

impl From<&String> for RollupId {
    fn from(value: &String) -> Self {
        Self(value.to_string())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

impl std::fmt::Display for SequencerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for SequencerId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<&str> for SequencerId {
    fn from(value: &str) -> Self {
        Self(format!("http://{}", value))
    }
}

impl From<String> for SequencerId {
    fn from(value: String) -> Self {
        Self(format!("http://{}", value))
    }
}

impl From<&String> for SequencerId {
    fn from(value: &String) -> Self {
        Self(format!("http://{}", value))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SequencerSet {
    block_height: BlockHeight,
    set: HashSet<SequencerId>,
    leader: Option<SequencerId>,
    seed: [u8; 32],
}

impl SequencerSet {
    pub fn new(block_height: BlockHeight) -> Self {
        Self {
            block_height,
            set: HashSet::default(),
            leader: None,
            // SSAL-001
            seed: [
                1, 0, 0, 0, 23, 0, 0, 0, 200, 1, 0, 0, 210, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0,
            ],
        }
    }

    pub fn block_height(&self) -> BlockHeight {
        self.block_height.clone()
    }

    pub fn register(&mut self, sequencer_id: SequencerId) -> Result<(), Error> {
        match self.set.insert(sequencer_id) {
            true => Ok(()),
            false => Err(Error::from("Sequencer is already registered")),
        }
    }

    pub fn elect_leader(&mut self) -> Result<SequencerId, Error> {
        let sequencer_vec: Vec<SequencerId> = self.set.iter().cloned().collect();
        match sequencer_vec.choose(&mut rand::thread_rng()) {
            Some(leader) => {
                self.leader = Some(leader.clone());
                Ok(leader.clone())
            }
            None => Err(Error::from("Failed to elect the leader.")),
        }
    }

    pub fn leader(&self) -> Option<SequencerId> {
        match &self.leader {
            Some(leader) => Some(leader.clone()),
            None => None,
        }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, SequencerId> {
        self.set.iter()
    }

    pub fn seed(&self) -> [u8; 32] {
        self.seed
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct RawTransaction(String);

impl std::fmt::Display for RawTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<[u8]> for RawTransaction {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl From<&str> for RawTransaction {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for RawTransaction {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&String> for RawTransaction {
    fn from(value: &String) -> Self {
        Self(value.to_string())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderCommitment {
    block_height: BlockHeight,
    tx_order: TransactionOrder,
}

impl OrderCommitment {
    pub fn new(block_height: BlockHeight, tx_order: TransactionOrder) -> Self {
        Self {
            block_height,
            tx_order,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct TransactionOrder(usize);

impl std::fmt::Display for TransactionOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<usize> for TransactionOrder {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl TransactionOrder {
    pub fn value(&self) -> usize {
        self.0
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn iter(&self) -> TxOrderIterator {
        TxOrderIterator {
            tx_order: self,
            index: 0,
        }
    }
}

pub struct TxOrderIterator<'a> {
    tx_order: &'a TransactionOrder,
    index: usize,
}

impl<'a> Iterator for TxOrderIterator<'a> {
    type Item = TransactionOrder;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.tx_order.value() {
            let tx_order = TransactionOrder::from(self.index);
            self.index += 1;
            Some(tx_order)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockMetadata {
    is_leader: bool,
    tx_order: TransactionOrder,
}

impl BlockMetadata {
    pub fn new(is_leader: bool) -> Self {
        Self {
            is_leader,
            tx_order: TransactionOrder::default(),
        }
    }

    pub fn is_leader(&self) -> bool {
        self.is_leader
    }

    pub fn issue_tx_order(&mut self) -> TransactionOrder {
        let current_order = self.tx_order.clone();
        self.tx_order.increment();
        current_order
    }

    pub fn tx_count(&self) -> TransactionOrder {
        self.tx_order.clone()
    }
}
