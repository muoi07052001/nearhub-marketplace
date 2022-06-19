// Các hàm cho Templates
use crate::*;

#[near_bindgen]
impl NFTContract {
    // Tạo 1 Template mới
    /**
     * - Yêu cầu user nạp tiền để cover phí lưu trữ
     * - Thêm Template vào templates_by_id
     * - Refund lại NEAR user deposit thừa
     */
    #[payable]
    pub fn create_template(
        &mut self,
        collection_name: CollectionName,
        schema_id: SchemaId,
        transferable: bool,
        burnable: bool,
        max_supply: u32,
        immutable_data: ImmutableData,
    ) -> Template {
        let before_storage_usage = env::storage_usage(); // Dùng để tính toán lượng near thừa khi deposit

        let template_id = self.templates_by_id.len() as u32;

        self.token_by_template_id_counter.insert(&template_id, &0); // Khi tạo Template -> Cho stt counter = 0

        // Check collection_id có tồn tại không
        // Lấy collection name từ id
        let collection_of_template = self.collections_by_name.get(&collection_name).expect("Collection not exists");
        let collection_of_template_id = collection_of_template.collection_id;

        // Check signer id is Collection's owner or not
        assert_eq!(
            collection_of_template.owner_id,
            env::predecessor_account_id(),
            "Only owner of this collection can create Template"
        );

        // Check schema_id có tồn tại không
        // Lấy schema name từ id
        let schema_of_template = self.schemas_by_id.get(&schema_id).expect("Schema not exists");
        let schema_of_template_name = schema_of_template.schema_name;

        // Check xem schema_id đó có thuộc collection_id đó không
        assert_eq!(schema_of_template.collection_name, collection_name, "Schema does not belongs to this collection");

        // Tạo Template mới
        let new_template = Template {
            template_id,
            collection_id: collection_of_template_id,
            collection_name,
            schema_id,
            schema_name: schema_of_template_name,
            transferable,
            burnable,
            max_supply,
            issued_supply: 0,
            immutable_data,
        };

        // Insert template mới vào templates_by_id
        self.templates_by_id.insert(&template_id, &new_template);

        // Luợng data storage sử dụng = after_storage_usage - before_storage_usage
        let after_storage_usage = env::storage_usage();
        // Refund NEAR
        refund_deposit(after_storage_usage - before_storage_usage);

        new_template
    }

    // Lấy tổng số Templates đang có trong contract
    pub fn template_total_supply(&self) -> U128 {
        // Đếm tổng số lượng id đang có trong token_metadata_by_id
        U128(self.templates_by_id.len() as u128)
    }

    // Lấy tổng số Templates đang có của Collection nào đó
    pub fn template_supply_by_collection(&self, collection_name: CollectionName) -> U128 {
        // Check collection id có tồn tại không
        assert!(self.collections_by_name.get(&collection_name).is_some(), "Collection does not exist");

        let mut count = 0;

        let templates_set_by_collection: Vec<Template> = self
            .templates_by_id
            .keys()
            .map(|template_id| self.templates_by_id.get(&template_id).unwrap())
            .collect();

        for template in templates_set_by_collection {
            if template.collection_name == collection_name {
                count += 1;
            }
        }

        U128(count)
    }

    // Lấy danh sách tất cả Templates trong Contract
    pub fn get_all_templates(&self, from_index: Option<u64>, limit: Option<u64>) -> Vec<Template> {
        // Duyệt tất cả các keys -> Trả về Template
        self.templates_by_id
            .iter()
            .skip(from_index.unwrap_or(0) as usize)
            .take(limit.unwrap_or(10) as usize)
            .map(|(template_id, _template)| self.templates_by_id.get(&template_id).unwrap())
            .collect()
    }

    // Lấy danh sách Template của Collection nào đó (có pagination)
    pub fn get_all_templates_by_collection(
        &self,
        collection_name: CollectionName,
        from_index: Option<u64>,
        limit: Option<u64>,
    ) -> Vec<Template> {
        let mut count = 0;

        // Check collection id có tồn tại không
        assert!(self.collections_by_name.get(&collection_name).is_some(), "Collection does not exist");

        let mut result = Vec::<Template>::new();

        // Duyệt tất cả các keys -> Trả về Collection
        let templates_set_for_owner: Vec<Template> = self
            .templates_by_id
            .keys()
            .skip(from_index.unwrap_or(0) as usize) // Pagination
            .map(|template_id| self.templates_by_id.get(&template_id).unwrap())
            .collect();

        // If limit = 0 -> Return empty Array
        if limit.unwrap() == 0 {
            return result;
        }

        for template in templates_set_for_owner {
            if template.collection_name == collection_name {
                result.push(template);
                count += 1;
            }
            if count == limit.unwrap_or(10) {
                break;
            }
        }
        result
    }

    // Lấy Template theo id
    pub fn get_template_by_id(&self, template_id: TemplateId) -> Template {
        self.templates_by_id.get(&template_id).expect("Template does not exist")
    }

    // Search Template theo name
    // Lấy về tất cả Template mà tên có chứa ký tự của `search_string`
    pub fn get_templates_by_name(&self, search_string: String) -> Vec<Template> {
        let templates_set: Vec<Template> = self.templates_by_id.values().collect();

        let mut result = Vec::<Template>::new();

        for template in templates_set {
            if template.immutable_data.name.to_lowercase().contains(&search_string.to_lowercase()) {
                result.push(template);
            }
        }
        result
    }
}
