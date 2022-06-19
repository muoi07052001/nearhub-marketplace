use crate::*;

// ----------------------------------- NFT Struct -----------------------------------
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Token {
    pub owner_id: AccountId,             // Chủ sở hữu của NFT
    pub token_id: TokenId,               // Id của NFT
    pub token_by_template_id: TokenId,   // Stt của NFT trong template nó thuộc vào
    pub collection_id: CollectionId,     // Id của Collection mà NFT thuộc vào
    pub collection_name: CollectionName, // Tên Collection mà NFT thuộc vào
    pub schema_id: SchemaId,             // Id của Schema mà NFT thuộc vào
    pub schema_name: SchemaName,         // Tên Schema mà NFT thuộc vào
    pub template_id: TemplateId,         // Tên Template mà NFT thuộc vào

    pub approved_account_ids: HashMap<AccountId, u64>, // Danh sách các accounts được approved để transfer Token này
    pub next_approval_id: u64,                         // Id của approve tiếp theo
}

// Dạng Json của NFT để trả về giá trị cần thiết
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    pub owner_id: AccountId,           // Chủ sở hữu của NFT
    pub token_id: TokenId,             // Id của NFT
    pub token_by_template_id: TokenId, // Stt của NFT trong template nó thuộc vào
    pub collection_id: CollectionId,   // Id của Collection mà NFT thuộc vào
    pub collection_name: String,       // Tên Collection mà NFT thuộc vào
    pub schema_id: SchemaId,           // Id của Schema mà NFT thuộc vào
    pub schema_name: String,           // Tên Schema mà NFT thuộc vào
    pub template_id: TemplateId,       // Tên Template mà NFT thuộc vào
    pub metadata: TokenMetadata,       // Metadata của NFT

    pub approved_account_ids: HashMap<AccountId, u64>, // Danh sách các accounts được approved để transfer Token này
}

// ----------------------------------- Metadata -----------------------------------
// Các metadata theo chuẩn NEP-177 của NEAR - Metadata
// Xem thêm tại: https://nomicon.io/Standards/Tokens/NonFungibleToken/Metadata
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTContractMetadata {
    pub spec: String, // required, essentially a version like "nft-2.0.0", replacing "2.0.0" with the implemented version of NEP-177
    pub name: String, // required, ex. "Mochi Rising — Digital Edition" or "Metaverse 3"
    pub symbol: String, // required, ex. "MOCHI"
    pub icon: Option<String>, // Data URL
    pub base_uri: Option<String>, // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
    pub reference: Option<String>, // URL to a JSON file with more info
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub description: Option<String>, // free-form description
    pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
    pub media_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    pub copies: Option<u64>, // number of copies of this set of metadata in existence when token was minted.
    pub issued_at: Option<u64>, // When token was issued or minted, Unix epoch in milliseconds
    pub expires_at: Option<u64>, // When token expires, Unix epoch in milliseconds
    pub starts_at: Option<u64>, // When token starts being valid, Unix epoch in milliseconds
    pub updated_at: Option<u64>, // When token was last updated, Unix epoch in milliseconds
    pub extra: Option<String>, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
    pub reference: Option<String>, // URL to an off-chain JSON file with more info.
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
    pub nft_type: String, // Type of the NFT ("Lootbox" || "NFT")
}

pub trait NonFungibleTokenMetadata {
    fn nft_metadata(&self) -> NFTContractMetadata;
}

#[near_bindgen]
impl NonFungibleTokenMetadata for NFTContract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
