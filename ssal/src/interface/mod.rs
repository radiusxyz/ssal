pub mod rollup;
pub mod sequencer;
pub mod prelude {
    pub use ssal_core::{
        axum::{extract::State, http::StatusCode, response::IntoResponse, Json},
        error::{Error, WrapError},
        serde::{Deserialize, Serialize},
        tracing,
        types::*,
    };
    pub use ssal_database::{Database, Lock};

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(crate = "ssal_core::serde")]
    pub enum Key {
        BlockMetadata(RollupId),
        SequencerPool(RollupId, BlockHeight),
    }
}
