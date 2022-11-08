use std::{collections::HashMap, fmt::Display};

use near_sdk::json_types::{I128, U64};

use crate::*;

pub type AppchainId = String;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct RegistrySettings {
    /// The minimum deposit amount for registering an appchain.
    pub minimum_register_deposit: U128,
    /// The reduction percent of voting score of all appchain `inQueue` after each time
    /// the owner conclude the voting score.
    pub voting_result_reduction_percent: u16,
    /// The interval for calling function `count_voting_score`,
    /// in the interval this function can only be called once.
    pub counting_interval_in_seconds: U64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct RegistryRoles {
    /// The account that manages the lifecycle of appchains.
    pub appchain_lifecycle_manager: AccountId,
    /// The account that manages the settings of appchain registry.
    pub registry_settings_manager: AccountId,
    /// The account of octopus council (DAO contract)
    pub octopus_council: Option<AccountId>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum AppchainTemplateType {
    Barnacle,
    BarnacleEvm,
}

/// Appchain metadata
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AppchainMetadata {
    pub description: String,
    pub template_type: AppchainTemplateType,
    pub website_url: String,
    pub function_spec_url: String,
    pub github_address: String,
    pub github_release: String,
    pub contact_email: String,
    pub premined_wrapped_appchain_token_beneficiary: Option<AccountId>,
    pub premined_wrapped_appchain_token: U128,
    pub initial_supply_of_wrapped_appchain_token: U128,
    pub ido_amount_of_wrapped_appchain_token: U128,
    pub initial_era_reward: U128,
    pub fungible_token_metadata: FungibleTokenMetadata,
    pub custom_metadata: HashMap<String, String>,
}

/// The state of an appchain
#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum AppchainState {
    Registered,
    Audited,
    Voting,
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
    pub evm_chain_id: Option<U64>,
    pub appchain_metadata: AppchainMetadata,
    pub appchain_anchor: Option<AccountId>,
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
            AppchainState::Audited => false,
            AppchainState::Voting => false,
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
            AppchainState::Audited => write!(f, "audited"),
            AppchainState::Voting => write!(f, "voting"),
            AppchainState::Staging => write!(f, "staging"),
            AppchainState::Booting => write!(f, "booting"),
            AppchainState::Active => write!(f, "active"),
            AppchainState::Broken => write!(f, "broken"),
            AppchainState::Dead => write!(f, "dead"),
        }
    }
}
