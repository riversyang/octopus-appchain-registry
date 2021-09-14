use std::{collections::HashMap, fmt::Display};

use near_sdk::json_types::{I128, U64};

use crate::*;

pub type AppchainId = String;

/// Appchain metadata
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AppchainMetadata {
    pub website_url: String,
    pub github_address: String,
    pub github_release: String,
    pub commit_id: String,
    pub contact_email: String,
    pub custom_metadata: HashMap<String, String>,
}

/// The state of an appchain
#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum AppchainState {
    Registered,
    Auditing,
    InQueue,
    Staging,
    Booting,
    Active,
    Broken,
    Dead,
}

/// Appchain status
///
/// This struct should NOT be used in storage on chain
#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AppchainStatus {
    pub appchain_id: AppchainId,
    pub appchain_metadata: AppchainMetadata,
    pub appchain_anchor: AccountId,
    pub appchain_owner: AccountId,
    pub register_deposit: U128,
    pub appchain_state: AppchainState,
    pub upvote_deposit: U128,
    pub downvote_deposit: U128,
    pub voting_score: I128,
    pub registered_time: U64,
    pub go_live_time: U64,
    pub validator_count: u32,
    pub total_stake: U128,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum AppchainSortingField {
    AppchainId,
    VotingScore,
    RegisteredTime,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum SortingOrder {
    Ascending,
    Descending,
}

impl AppchainState {
    /// Get whether the state is managed by appchain anchor
    pub fn is_managed_by_anchor(&self) -> bool {
        match self {
            AppchainState::Registered => false,
            AppchainState::Auditing => false,
            AppchainState::InQueue => false,
            AppchainState::Staging => true,
            AppchainState::Booting => true,
            AppchainState::Active => true,
            AppchainState::Broken => true,
            AppchainState::Dead => false,
        }
    }
}

impl Display for AppchainState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppchainState::Registered => write!(f, "registered"),
            AppchainState::Auditing => write!(f, "auditing"),
            AppchainState::InQueue => write!(f, "inQueue"),
            AppchainState::Staging => write!(f, "staging"),
            AppchainState::Booting => write!(f, "booting"),
            AppchainState::Active => write!(f, "active"),
            AppchainState::Broken => write!(f, "broken"),
            AppchainState::Dead => write!(f, "dead"),
        }
    }
}
