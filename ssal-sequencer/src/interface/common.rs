use super::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "ssal_core::serde")]
pub struct SendTransaction {
    rollup_id: RollupId,
    raw_tx: RawTransaction,
}

impl SendTransaction {
    pub async fn handler(
        State(state): State<Database>,
        Json(payload): Json<Self>,
    ) -> Result<impl IntoResponse, Error> {
        // let (is_leader, leader_id): Lock<(bool, SequencerId)> =
        //     state.get_mut(&("leader", &payload.rollup_id))?;
        Ok((StatusCode::OK, Json(())))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "ssal_core::serde")]
pub struct CheckLeader {
    rollup_id: RollupId,
}

impl CheckLeader {
    pub async fn handler(
        State(state): State<Database>,
        Query(parameter): Query<Self>,
    ) -> Result<impl IntoResponse, Error> {
        Ok(().into_response())
    }
}
