use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    /// Admin address, can update config values.
    pub admin_address: Addr,
    /// Number of blocks between user draws, if set to 30 user
    /// must wait 30 blocks before being able to draw again.
    pub cooldown: u64,
    /// Block height the canvas can no longer be drawn on at all.
    /// Optional so if not set it goes on forever.
    pub end_height: Option<u64>,
    pub start_height: Option<u64>,
    pub collection_address: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Dimensions {
    /// Number of chunks wide
    pub width: u64,
    /// Number of chunks high
    pub height: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PixelInfo {
    pub color: u8
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const DIMENSIONS: Item<Dimensions> = Item::new("dimensions");
// A chunk is a 32x32 group of pixels
pub const CHUNKS: Map<(u64, u64), Vec<Vec<PixelInfo>>> = Map::new("chunks");
pub const COOLDOWNS: Map<&Addr, u64> = Map::new("cooldowns");
