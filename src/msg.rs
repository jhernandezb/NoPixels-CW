use crate::state::PixelInfo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin_address: String,
    pub cooldown: u64,
    pub end_height: Option<u64>,
    pub start_height: Option<u64>,
    pub collection_address: Option<String>,
    /// How many chunks wide
    pub width: u64,
    /// How many chunks high
    pub height: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Draw {
        chunk_x: u64,
        chunk_y: u64,
        x: u64,
        y: u64,
        color: u8,
    },
    UpdateAdmin {
        new_admin_address: String,
    },
    UpdateCooldown {
        new_cooldown: u64,
    },
    UpdateDimensions {
        new_width: u64,
        new_height: u64,
    },
    UpdateEndHeight {
        new_end_height: Option<u64>,
    },
    UpdateStartHeight {
        new_start_height: Option<u64>,
    },
    UpdateCollection {
        new_collection_address: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetChunk { x: u64, y: u64 },
    GetConfig {},
    GetDimensions {},
    GetCooldown { address: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ChunkResponse {
    pub grid: Vec<Vec<PixelInfo>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CooldownResponse {
    pub current_cooldown: u64,
}
