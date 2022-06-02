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
    pub collection_name: String,          // Collection mà Schema thuộc vào
    pub schema_name: String,              // Tên của Schema
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
    pub collection_name: String,       // Tên Collection mà Template thuộc vào
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
    pub name: String, // Tên của Template
    pub img: Option<String>,
    // TODO: ImmutableData chứa tất cả các trường T của Schema, nhưng các trường có thể Option<T>
}
