use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise,
};

pub use crate::internal::*;
pub use crate::nft_callback::*;
pub use crate::sale::*;
pub use crate::sale_view::*;
use crate::utils::*;

// Coi như sau mỗi lần bán qua lại thì tăng storage lên 1000 bytes
const STORAGE_PER_SALE: u128 = 1000 * env::STORAGE_PRICE_PER_BYTE;

mod internal;
mod nft_callback;
mod sale;
mod sale_view;
mod utils;

pub type TokenId = u32;
pub type NFTContractId = String;
pub type SalePriceInYoctoNear = U128;
// Để nếu có 2 Contract khác nhau cùng sử dụng market-contract này thì nếu trùng token id cũng ko sao
// Có dạng nearhub-nft.duongnh.testnet.1
pub type ContractAndTokenId = String;

pub type CollectionName = String;
pub type SchemaName = String;
pub type CollectionId = u32;
pub type SchemaId = u32;
pub type TemplateId = u32;

// Struct cho việc mua bán
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Sale {
    pub owner_id: AccountId, // Owner of the Sale
    pub approval_id: u64,
    pub nft_contract_id: NFTContractId,        // nft-contract
    pub token_id: TokenId,                     // Id of NFT
    pub token_by_template_id: TokenId,         // The position of this NFT inside the Template it belongs to
    pub collection_id: CollectionId,           // Id of the Collection this NFT belongs to
    pub collection_name: CollectionName,       // Name of the Collection this NFT belongs to
    pub schema_id: SchemaId,                   // Id of the Schema this NFT belongs to
    pub schema_name: SchemaName,               // Name of the Schema this NFT belongs
    pub template_id: TemplateId,               // Id of the Schema this NFT belongs to
    pub sale_conditions: SalePriceInYoctoNear, // Các điều kiện của sales (Giá, ...)
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MarketContract {
    pub owner_id: AccountId,                           // Owner of contract
    pub sales: UnorderedMap<ContractAndTokenId, Sale>, // Mapping thông tin của các NFT đang được Sales
    pub tokens_by_owner_id: LookupMap<AccountId, UnorderedSet<ContractAndTokenId>>, // Danh sách các token_id đang được đăng bán của 1 account_id
    pub tokens_by_contract_id: LookupMap<NFTContractId, UnorderedSet<TokenId>>, // Danh sách các token_id đang được đăng bán của 1 nft contract
    pub storage_deposit_account: LookupMap<AccountId, Balance>, // Danh sách lượng deposit của từng account để cover storage
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum StorageKey {
    SaleKey,
    ByOwnerIdKey,
    InnerByOwnerIdKey {
        // Trong UnorderedSet của ContractAndTokenId, muốn mỗi phần tử có 1 key riêng để đảm bảo ko trùng nhau
        account_id_hash: CryptoHash,
    },
    ByContractIdKey,
    InnerByContractIdKey {
        // Trong UnorderedSet của TokenId, muốn mỗi phần tử có 1 key riêng để đảm bảo ko trùng nhau
        account_id_hash: CryptoHash,
    },
    StorageDepositKey,
}

#[near_bindgen]
impl MarketContract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            sales: UnorderedMap::new(StorageKey::SaleKey.try_to_vec().unwrap()),
            tokens_by_owner_id: LookupMap::new(StorageKey::ByOwnerIdKey.try_to_vec().unwrap()),
            tokens_by_contract_id: LookupMap::new(
                StorageKey::ByContractIdKey.try_to_vec().unwrap(),
            ),
            storage_deposit_account: LookupMap::new(
                StorageKey::StorageDepositKey.try_to_vec().unwrap(),
            ),
        }
    }

    // Cho phép user deposit 1 lượng Near vào contract để cover phí storage
    // User có thể deposit cho account khác
    #[payable]
    pub fn storage_deposit(&mut self, account_id: Option<AccountId>) {
        // Nếu có gắn account_id -> deposit cho account_id
        // Nếu không có account_id -> deposit cho người gọi hàm
        let storage_account_id = account_id.unwrap_or(env::predecessor_account_id());
        let deposit = env::attached_deposit();

        assert!(
            deposit >= STORAGE_PER_SALE,
            "Required deposit minimum of {}",
            STORAGE_PER_SALE
        );

        // Cộng thêm số tiền deposit vào storage_deposit của account_id
        let mut balance = self
            .storage_deposit_account
            .get(&storage_account_id)
            .unwrap_or(0);
        balance += deposit;

        // Update dữ liệu
        self.storage_deposit_account
            .insert(&storage_account_id, &balance);
    }

    // Cho phép người dùng rút lại tiền đã deposit mà đang ko dùng để lưu trữ data gì cả
    #[payable]
    pub fn storage_withdraw(&mut self) {
        assert_one_yocto();
        let owner_id = env::predecessor_account_id();

        // Lấy ra lượng tiền đã deposit của user, đồng thời xoá user khỏi list đã deposit luôn
        let amount = self.storage_deposit_account.remove(&owner_id).unwrap_or(0);

        // Tính tổng tiền cần để cover storage của user
        // Lượng tiền đã deposit thừa ra thì refund lại cho user
        let sales = self.tokens_by_owner_id.get(&owner_id); // Danh sách các token đang đăng bán của user

        let len = sales.map(|s| s.len()).unwrap_or_default();

        // VD: user đang đăng bán 3 tokens
        // -> lượng tiền để cover data storage = 3 * lượng tiền cần cho mỗi tokens
        let storage_required = u128::from(len) * STORAGE_PER_SALE;

        // Check xem lượng deposit hiện tại có cover được data storage ko
        assert!(amount >= storage_required);

        // Tính lượng tiền thừa ra để cover storage của user
        let diff = amount - storage_required;

        // Nếu thừa -> transfer lại cho user
        if diff > 0 {
            Promise::new(owner_id.clone()).transfer(diff);
        }

        // Nếu user còn lưu trữ data -> Cập nhật lại thông tin trong list storage_deposit_account
        if storage_required > 0 {
            self.storage_deposit_account
                .insert(&owner_id, &storage_required);
        }
    }

    pub fn storage_minimun_balance(&self) -> U128 {
        U128(STORAGE_PER_SALE)
    }

    // Check lượng storage đã deposit của account_id
    pub fn storage_balance_of(&self, account_id: Option<AccountId>) -> U128 {
        let owner_id = account_id.unwrap_or(env::predecessor_account_id());

        U128(self.storage_deposit_account.get(&owner_id).unwrap_or(0))
    }
}
