pub mod client;
pub mod rollup;
pub mod sequencer;
pub mod prelude {
    pub use ssal_core::{
        axum::{extract::State, http::StatusCode, response::IntoResponse, Json},
        error::{Error, WrapError},
        rand::{self, seq::SliceRandom},
        serde::{Deserialize, Serialize},
        tracing,
        types::*,
    };
    pub use ssal_database::{Database, Lock};

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(crate = "ssal_core::serde")]
    pub enum Key {
        BlockHeight(RollupId),
        Leader(RollupId, BlockHeight),
        RollupSet,
        SequencerPool(RollupId, BlockHeight),
        SequencerSet(RollupId, BlockHeight),
    }
}
