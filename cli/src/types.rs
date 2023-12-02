use serde::{Deserialize, Serialize};

pub const URL: &str = "https://jup-airdrop.zhen8558.workers.dev/allocation";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub owner: String,
    #[serde(rename = "tokens_final")]
    pub tokens_final: String,
    #[serde(rename = "tokens_tier")]
    pub tokens_tier: String,
    #[serde(rename = "tokens_og_bonus")]
    pub tokens_og_bonus: String,
    #[serde(rename = "tokens_welcome")]
    pub tokens_welcome: String,
    pub tier: String,
    pub score: String,
    pub volume: String,
    #[serde(rename = "check_og")]
    pub check_og: String,
    #[serde(rename = "check_2023_multiplier")]
    pub check_2023_multiplier: String,
    #[serde(rename = "check_consistency")]
    pub check_consistency: String,
    #[serde(rename = "check_lo_dca")]
    pub check_lo_dca: String,
    #[serde(rename = "likely_bot")]
    pub likely_bot: String,
    #[serde(rename = "likely_cluster")]
    pub likely_cluster: String,
    #[serde(rename = "debug_product_score")]
    pub debug_product_score: String,
    #[serde(rename = "debug_swap_score")]
    pub debug_swap_score: String,
    #[serde(rename = "debug_consistency_score")]
    pub debug_consistency_score: String,
    #[serde(rename = "debug_2023")]
    pub debug_2023: String,
    #[serde(rename = "debug_tx_failure")]
    pub debug_tx_failure: String,
    pub hacker: String,
    #[serde(rename = "debug_rank")]
    pub debug_rank: String,
}


pub fn format_url(addr: &str) -> String {
    format!("{URL}/{addr}")
}

pub async fn fetch_allocation(
    addr: &str
) -> anyhow::Result<Root> {
    let req = reqwest::Client::new().get(format_url(addr)).build()?;
    let res = reqwest::Client::default().execute(req).await?;
    Ok(res.json().await?)
}