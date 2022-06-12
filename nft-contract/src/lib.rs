use near_rng::Rng;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, ext_contract, log, near_bindgen, AccountId, Balance, CryptoHash, Gas, PanicOnDefault,
    Promise, PromiseOrValue, PromiseResult,
};
use std::collections::HashMap;
// use std::time::{SystemTime, UNIX_EPOCH};
use near_sdk::Timestamp;

pub type CollectionId = u32;
pub type CollectionName = String;
pub type SchemaId = u32;
pub type SchemaName = String;
pub type TemplateId = u32;
pub type TokenId = u32;
pub type LootboxId = u32;
pub type DropId = u32;

pub use crate::approval::*;
pub use crate::collections::*;
pub use crate::custom_struct::*;
pub use crate::drop::*;
pub use crate::event::*;
pub use crate::internal::*;
pub use crate::lootbox::*;
pub use crate::metadata::*;
pub use crate::nft::*;
pub use crate::nft_core::*;
pub use crate::schemas::*;
pub use crate::templates::*;
use crate::utils::*;

mod approval;
mod collections;
mod custom_struct;
mod drop;
mod event;
mod internal;
mod lootbox;
mod metadata;
mod nft;
mod nft_core;
mod schemas;
mod templates;
mod utils;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NFTContract {
    pub owner_id: AccountId, // Chủ sở hữu của Contract
    pub collections_per_owner: LookupMap<AccountId, UnorderedSet<CollectionName>>, // Lưu danh sách NFT Collections mà user sở hữu
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>, // Lưu danh sách NFT mà user sở hữu
    pub collections_by_name: UnorderedMap<CollectionName, Collection>, // Danh sách tất cả Collections của Contract
    pub collections_by_id: UnorderedMap<CollectionId, Collection>, // Danh sách tất cả Collections của Contract
    pub schemas_by_id: UnorderedMap<SchemaId, Schema>, // Danh sách tất cả Schemas của Contract
    pub templates_by_id: UnorderedMap<TemplateId, Template>, // Danh sách tất cả Templates của Contract
    pub token_by_template_id_counter: UnorderedMap<TemplateId, TokenId>, // Đếm stt hiện tại của từng Template
    pub tokens_by_id: UnorderedMap<TokenId, Token>, // Danh sách tất cả NFT Tokens của Contract
    pub lootboxes_by_id: UnorderedMap<LootboxId, Lootbox>, // Danh sách tất cả Lootboxs của Contract
    pub lootbox_id_counter: u32,                    // Auto increment Lootbox id
    pub drops_by_id: UnorderedMap<DropId, DropSale>, // Danh sách tất cả Lootboxs của Contract
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>, // Mapping token id với token metadata
    pub metadata: LazyOption<NFTContractMetadata>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum StorageKey {
    CollectionsPerOwnerKey,
    TokensPerOwnerKey,
    CollectionsPerOwnerInnerKey {
        account_id_hash: CryptoHash, // Để đảm bảo các account_id không trùng nhau
    },
    TokensPerOwnerInnerKey {
        account_id_hash: CryptoHash, // Để đảm bảo các account_id không trùng nhau
    },
    CollectionsByIdKey,
    CollectionsByNameKey,
    SchemasByIdKey,
    TemplatesByIdKey,
    TokenByTemplateIdCounter,
    TokensByIdKey,
    LootboxesByIdKey,
    DropsByIdKey,
    TokenMetadataByIdKey,
    ContractMetadataKey,
}

#[near_bindgen]
impl NFTContract {
    #[init]
    pub fn new(owner_id: AccountId, token_metadata: NFTContractMetadata) -> Self {
        Self {
            owner_id,
            collections_per_owner: LookupMap::new(
                StorageKey::CollectionsPerOwnerKey.try_to_vec().unwrap(),
            ),
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwnerKey.try_to_vec().unwrap()),
            collections_by_id: UnorderedMap::new(
                StorageKey::CollectionsByIdKey.try_to_vec().unwrap(),
            ),
            collections_by_name: UnorderedMap::new(
                StorageKey::CollectionsByNameKey.try_to_vec().unwrap(),
            ),
            schemas_by_id: UnorderedMap::new(StorageKey::SchemasByIdKey.try_to_vec().unwrap()),
            templates_by_id: UnorderedMap::new(StorageKey::TemplatesByIdKey.try_to_vec().unwrap()),
            token_by_template_id_counter: UnorderedMap::new(StorageKey::TokenByTemplateIdCounter.try_to_vec().unwrap()),
            tokens_by_id: UnorderedMap::new(StorageKey::TokensByIdKey.try_to_vec().unwrap()),
            lootboxes_by_id: UnorderedMap::new(StorageKey::LootboxesByIdKey.try_to_vec().unwrap()),
            lootbox_id_counter: 0,
            drops_by_id: UnorderedMap::new(StorageKey::DropsByIdKey.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataByIdKey.try_to_vec().unwrap(),
            ),
            metadata: LazyOption::new(
                StorageKey::ContractMetadataKey.try_to_vec().unwrap(),
                Some(&token_metadata),
            ),
        }
    }

    #[init]
    pub fn new_default_metadata(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: "nearhub-nft-1.0.0".to_string(),
                name: "Nearhub NFT".to_string(),
                symbol: "NHT".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }
}
