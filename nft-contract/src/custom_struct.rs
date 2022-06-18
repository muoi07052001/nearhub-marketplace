use crate::*;

// ----------------------------------- Collection Struct -----------------------------------
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Collection {
    pub collection_id: CollectionId, // Id của Collection
    pub owner_id: AccountId,         // Người sở hữu collection
    pub collection_name: String,     // Tên của collection
    pub market_fee: f32,             // Phần trăm nhận lại với mỗi giao dịch NFT
    pub data: CollectionExtraData, // Extra data của collection (nếu collection có field data mới thì cho vào đây)

    pub approved_account_ids: HashMap<AccountId, u64>, // Danh sách các accounts được approved để transfer Token thuộc Collection này
    pub next_approval_id: u64,                         // Id của approve tiếp theo
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CollectionExtraData {
    pub name: String,        // Display name của Collection
    pub img: Option<String>, // Mã hash của ảnh của Collection
}

// ----------------------------------- Schema Struct -----------------------------------
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Schema {
    // pub authorized_creator: String,       // Tên người tạo Schema (owner của collection_name)
    pub schema_id: SchemaId,              // Id của Schema
    pub schema_name: String,              // Tên của Schema
    pub collection_id: CollectionId,      // Id của Collection mà Schme thuộc vào
    pub collection_name: String,          // Collection mà Schema thuộc vào
    pub schema_format: Vec<SchemaFormat>, // Mảng các thuộc tính của Schema
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SchemaFormat {
    pub attribute_name: String, // Tên của attribute
    pub attribute_type: String, // Kiểu dữ liệu của attribute
}

// ----------------------------------- Template Struct -----------------------------------
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Template {
    pub template_id: TemplateId,       // Id của Template
    pub collection_id: CollectionId,   // Id của Collection mà Template thuộc vào
    pub collection_name: String,       // Tên Collection mà Template thuộc vào
    pub schema_id: SchemaId,           // Id của Schema mà Template thuộc vào
    pub schema_name: String,           // Tên Schema mà Template thuộc vào
    pub transferable: bool,            // NFT có thể bị transfer không
    pub burnable: bool,                // NFT có thể bị burn không
    pub max_supply: u32,               // Tổng số NFT cung cấp
    pub issued_supply: u32,            // Số NFT đã cung cấp
    pub immutable_data: ImmutableData, // Những attribute trong Schema mà được fixed sẵn giá trị
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ImmutableData {
    pub name: String,                         // Tên của Template
    pub img: Option<String>,                  // Link tới ảnh của Template
    pub rarity: Option<String>,               // Rarity of the Template
    pub extra_immutable_data: Option<String>, // Stringified JSON chứa các thông tin fix sẵn dựa theo Schema gửi từ Front-end lên
}

// ----------------------------------- Lootbox Struct -----------------------------------
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Lootbox {
    pub lootbox_id: LootboxId,        // Id của Lootbox
    pub lootbox_name: String,         // Tên của Lootbox
    pub img: Option<String>,          // Link to image of the Lootbox
    pub description: String,          // Description của Lootbox
    pub collection_id: CollectionId,  // Id của Collection mà Lootbox thuộc vào
    pub collection_name: String,      // Collection mà Lootbox thuộc vào
    pub unlock_time: Timestamp,       // Thời điểm cho phép mở Lootbox
    pub display_data: Option<String>, // Dữ liệu cần thiết hiển thị cho Front-end (stringified JSON)
    pub config: LootboxConfig,        // Config độ random để ra các loại NFT
}

// 1 mảng chứa các Slot NFT
// Độ dài mảng là số NFT chứa trong Lootbox
// Mỗi Slot có config tỷ lệ ra TemplateId khác nhau
pub type LootboxConfig = Vec<Slot>;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Slot {
    pub total_odds: u32,        // Giới hạn trên của số random
    pub outcomes: Vec<Outcome>, // Mảng config: Template A tỉ lệ bn, Template B tỉ lệ bn
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Outcome {
    pub template_id: TemplateId, // Tỷ lệ roll ra template id nào
    pub odds: u32,               // Trong khoảng (a, odds) thì roll ra template_id này
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct LootboxNft {
    pub owner_id: AccountId,                // Chủ sở hữu của NFT Lootbox
    pub lootbox_nft_id: TokenId,            // Id của NFT Lootbox
    pub lootbox_id: LootboxId,              // Id of the Lootbot that this NFT belongs to
    pub lootbox_nft_by_lootbox_id: TokenId, // Stt của NFT Lootbox trong Lootbox nó thuộc vào
    pub collection_id: CollectionId,        // Id của Collection mà NFT thuộc vào
    pub collection_name: CollectionName,    // Tên Collection mà NFT thuộc vào
    // pub schema_id: SchemaId,                // Id của Schema mà NFT thuộc vào
    // pub schema_name: SchemaName,            // Tên Schema mà NFT thuộc vào
    // pub template_id: TemplateId,            // Tên Template mà NFT thuộc vào

    // pub metadata: TokenMetadata,                       // Metadata của NFT
    pub approved_account_ids: HashMap<AccountId, u64>, // Danh sách các accounts được approved để transfer NFT Lootbox này
    pub next_approval_id: u64,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonLootboxNft {
    pub owner_id: AccountId,                // Chủ sở hữu của NFT Lootbox
    pub lootbox_nft_id: TokenId,            // Id của NFT Lootbox
    pub lootbox_id: LootboxId,              // Id of the Lootbot that this NFT belongs to
    pub lootbox_nft_by_lootbox_id: TokenId, // Stt của NFT Lootbox trong Lootbox nó thuộc vào
    pub collection_id: CollectionId,        // Id của Collection mà NFT thuộc vào
    pub collection_name: CollectionName,    // Tên Collection mà NFT thuộc vào
    pub metadata: TokenMetadata,
    pub approved_account_ids: HashMap<AccountId, u64>, // Danh sách các accounts được approved để transfer NFT Lootbox này
    pub next_approval_id: u64,
}
