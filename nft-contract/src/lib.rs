use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, CryptoHash};
use std::collections::HashMap;

pub type CollectionId = u32;
pub type SchemaId = u32;
pub type TemplateId = u32;
pub type TokenId = u32;

pub use crate::custom_struct::*;
pub use crate::enumeration::*;
pub use crate::metadata::*;
pub use crate::collections::*;
pub use crate::schemas::*;
pub use crate::templates::*;
use crate::utils::*;

mod custom_struct;
mod enumeration;
mod metadata;
mod collections;
mod schemas;
mod templates;
mod utils;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NFTContract {
    pub owner_id: AccountId, // Chủ sở hữu của Contract
    pub collections_per_owner: LookupMap<AccountId, UnorderedSet<CollectionId>>, // Lưu danh sách NFT Collections mà user sở hữu
    pub collections_by_id: UnorderedMap<CollectionId, Collection>, // Danh sách tất cả Collections của Contract
    pub schemas_by_id: UnorderedMap<SchemaId, Schema>, // Danh sách tất cả Schemas của Contract
    pub templates_by_id: UnorderedMap<TemplateId, Template>, // Danh sách tất cả Templates của Contract
    pub tokens_by_id: UnorderedMap<TokenId, Token>, // Danh sách tất cả NFT Tokens của Contract
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>, // Mapping token id với token metadata
    pub metadata: LazyOption<NFTContractMetadata>,
    pub collection_id_counter: CollectionId, // Bộ đếm id cho Collection
    pub schema_id_counter: SchemaId, // Bộ đếm id cho Schema
    pub template_id_counter: TemplateId, // Bộ đếm id cho Template
    pub token_id_counter: TokenId, // Bộ đếm id cho NFT
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum StorageKey {
    CollectionsPerOwnerKey,
    CollectionsPerOwnerInnerKey {
        account_id_hash : CryptoHash, // Để đảm bảo các account_id không trùng nhau
    },
    CollectionsByIdKey,
    SchemasByIdKey,
    TemplatesByIdKey,
    TokensByIdKey,
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
            collections_by_id: UnorderedMap::new(StorageKey::CollectionsByIdKey.try_to_vec().unwrap()),
            schemas_by_id: UnorderedMap::new(StorageKey::SchemasByIdKey.try_to_vec().unwrap()),
            templates_by_id: UnorderedMap::new(StorageKey::TemplatesByIdKey.try_to_vec().unwrap()),
            tokens_by_id: UnorderedMap::new(StorageKey::TokensByIdKey.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataByIdKey.try_to_vec().unwrap(),
            ),
            metadata: LazyOption::new(
                StorageKey::ContractMetadataKey.try_to_vec().unwrap(),
                Some(&token_metadata),
            ),
            collection_id_counter: 1,
            schema_id_counter: 1,
            template_id_counter: 1,
            token_id_counter: 1,
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
