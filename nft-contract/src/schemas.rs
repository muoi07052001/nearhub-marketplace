// Các hàm cho Schemas
use crate::*;

#[near_bindgen]
impl NFTContract {
    // Tạo 1 Schema mới thuộc 1 Collection nào đó
    pub fn create_schema(
        &mut self,
        collection_id: CollectionId,
        schema_name: String,
        schema_format: Vec<SchemaFormat>,
    ) -> Schema {
        let schema_id = self.schemas_by_id.len() as u32;

        // Check schema_id đã tồn tại chưa
        assert!(
            self.schemas_by_id.get(&schema_id).is_none(),
            "Schema id already exists"
        );

        // Check collection_id có tồn tại không
        // Lấy collection name từ id
        let collection_of_schema = self.collections_by_id.get(&collection_id).expect("Collection not exists");
        let collection_of_schema_name = collection_of_schema.collection_name;

        // Tạo collection mới
        let new_schema = Schema {
            schema_id,
            schema_name,
            collection_name: collection_of_schema_name,
            collection_id,
            schema_format,
        };

        // Insert schema mới vào schemas_by_id
        self.schemas_by_id.insert(&schema_id, &new_schema);

        new_schema
    }

    // Lấy tổng số Schemas đang có trong contract
    pub fn schema_total_supply(&self) -> U128 {
        // Đếm tổng số lượng id đang có trong token_metadata_by_id
        U128(self.schemas_by_id.len() as u128)
    }

    // Lấy tổng số Schemas đang có của Collection nào đó
    pub fn schema_supply_by_collection(&self, collection_id: CollectionId) -> U128 {
        // Check collection id có tồn tại không
        assert!(self.collections_by_id.get(&collection_id).is_some(), "Collection does not exist");

        let mut count = 0;

        let schemas_set_by_collection: Vec<Schema> = self
            .schemas_by_id
            .keys()
            .map(|schema_id| self.schemas_by_id.get(&schema_id).unwrap())
            .collect();

        for schema in schemas_set_by_collection {
            if schema.collection_id == collection_id {
                count += 1;
            }
        }

        U128(count)
    }

    // Lấy danh sách tất cả Schemas trong Contract
    pub fn get_all_schemas(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Schema> {
        let start = u128::from(from_index.unwrap_or(U128(0)));

        // Duyệt tất cả các keys -> Trả về Collection
        // self.collections_by_id.values_as_vector().to_vec()
        self.schemas_by_id
            .iter()
            .skip(start as usize)
            .take(limit.unwrap() as usize)
            .map(|(schema_id, _schema)| self.schemas_by_id.get(&schema_id).unwrap())
            .collect()
    }

    // Lấy danh sách Schema của Collection nào đó (có pagination)
    pub fn get_all_schemas_by_collection(
        &self,
        collection_id: CollectionId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Schema> {
        // Check collection id có tồn tại không
        assert!(self.collections_by_id.get(&collection_id).is_some(), "Collection does not exist");

        let start = u128::from(from_index.unwrap_or(U128(0)));

        let mut result = Vec::<Schema>::new();

        // Duyệt tất cả các keys -> Trả về Collection
        let schemas_set_for_owner: Vec<Schema> = self
            .schemas_by_id
            .keys()
            .skip(start as usize) // Pagination
            .take(limit.unwrap_or(0) as usize) // Pagination
            .map(|schema_id| self.schemas_by_id.get(&schema_id).unwrap())
            .collect();

        for schema in schemas_set_for_owner {
            if schema.collection_id == collection_id {
                result.push(schema);
            }
        }
        result
    }

    // Search Schema theo name
    // Lấy về tất cả Schema mà tên có chứa ký tự của `search_string`
    pub fn get_schemas_by_name(&self, search_string: String) -> Vec<Schema> {
        let schemas_set: Vec<Schema> = self.schemas_by_id.values().collect();

        let mut result = Vec::<Schema>::new();

        for schema in schemas_set {
            if schema.schema_name.to_lowercase().contains(&search_string.to_lowercase()) {
                result.push(schema);
            }
        }
        result
    }
}
