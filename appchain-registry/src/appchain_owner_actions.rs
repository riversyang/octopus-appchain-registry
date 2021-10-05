use near_sdk::{env, near_bindgen, AccountId};

use crate::*;

/// The actions which the owner of an appchain can perform
pub trait AppchainOwnerActions {
    /// Update custom metadata of an appchain
    fn update_appchain_custom_metadata(
        &mut self,
        appchain_id: AppchainId,
        custom_metadata: HashMap<String, String>,
    );
    /// Transfer ownership of an appchain to another account
    fn transfer_appchain_ownership(&mut self, appchain_id: AppchainId, new_owner: AccountId);
}

#[near_bindgen]
impl AppchainOwnerActions for AppchainRegistry {
    fn update_appchain_custom_metadata(
        &mut self,
        appchain_id: AppchainId,
        custom_metadata: HashMap<String, String>,
    ) {
        self.assert_appchain_owner(&appchain_id);
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        let mut metadata = appchain_basedata.metadata();
        metadata.custom_metadata.clear();
        custom_metadata.keys().for_each(|key| {
            metadata
                .custom_metadata
                .insert(key.clone(), custom_metadata.get(key).unwrap().clone());
        });
        appchain_basedata.set_metadata(&metadata);
        self.appchain_basedatas
            .insert(&appchain_id, &appchain_basedata);
        env::log(
            format!(
                "The custom metadata of appchain '{}' is updated by '{}'.",
                appchain_basedata.id(),
                env::predecessor_account_id()
            )
            .as_bytes(),
        )
    }

    fn transfer_appchain_ownership(&mut self, appchain_id: AppchainId, new_owner: AccountId) {
        self.assert_appchain_owner(&appchain_id);
        let mut appchain_basedata = self.get_appchain_basedata(&appchain_id);
        appchain_basedata.change_owner(&new_owner);
        self.appchain_basedatas
            .insert(&appchain_id, &appchain_basedata);
        env::log(
            format!(
                "The ownership of appchain '{}' is transfered to '{}'.",
                appchain_basedata.id(),
                appchain_basedata.owner()
            )
            .as_bytes(),
        )
    }
}
