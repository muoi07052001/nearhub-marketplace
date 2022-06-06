// Các hàm cho Templates
use crate::*;

#[near_bindgen]
impl NFTContract {
    // Tạo 1 Template mới
    pub fn create_template(
        &mut self,
        collection_name: CollectionName,
        schema_id: SchemaId,
        transferable: bool,
        burnable: bool,
        max_supply: u32,
        issued_supply: u32,
        immutable_data: ImmutableData,
    ) -> Template {
        let template_id = self.templates_by_id.len() as u32;

        // Check template_id đã tồn tại chưa
        assert!(
            self.templates_by_id.get(&template_id).is_none(),
            "Template id already exists"
        );

        // Check collection_id có tồn tại không
        // Lấy collection name từ id
        let collection_of_template = self.collections_by_name.get(&collection_name).expect("Collection not exists");
        let collection_of_template_id = collection_of_template.collection_id;

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
            issued_supply,
            immutable_data,
        };

        // Insert template mới vào templates_by_id
        self.templates_by_id.insert(&template_id, &new_template);

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
    pub fn get_all_templates(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Template> {
        let start = u128::from(from_index.unwrap_or(U128(0)));

        // Duyệt tất cả các keys -> Trả về Template
        self.templates_by_id
            .iter()
            .skip(start as usize)
            .take(limit.unwrap() as usize)
            .map(|(template_id, _template)| self.templates_by_id.get(&template_id).unwrap())
            .collect()
    }

    // Lấy danh sách Template của Collection nào đó (có pagination)
    pub fn get_all_templates_by_collection(
        &self,
        collection_name: CollectionName,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Template> {
        // Check collection id có tồn tại không
        assert!(self.collections_by_name.get(&collection_name).is_some(), "Collection does not exist");

        let start = u128::from(from_index.unwrap_or(U128(0)));

        let mut result = Vec::<Template>::new();

        // Duyệt tất cả các keys -> Trả về Collection
        let templates_set_for_owner: Vec<Template> = self
            .templates_by_id
            .keys()
            .skip(start as usize) // Pagination
            .take(limit.unwrap_or(0) as usize) // Pagination
            .map(|template_id| self.templates_by_id.get(&template_id).unwrap())
            .collect();

        for template in templates_set_for_owner {
            if template.collection_name == collection_name {
                result.push(template);
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
