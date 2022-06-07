// Các hàm cho Collections
use crate::*;

#[near_bindgen]
impl NFTContract {
    // Tạo 1 Collection mới
    /**
     * - Yêu cầu user nạp tiền để cover phí lưu trữ
     * - Thêm Collection vào collections_by_name + collections_by_id
     * - Thêm Collection vào danh sách sở hữu bởi owner
     * - Refund lại NEAR user deposit thừa
     */
    #[payable]
    pub fn create_collection(
        &mut self,
        collection_name: String,
        market_fee: f32,
        data: CollectionExtraData,
    ) -> Collection {
        let before_storage_usage = env::storage_usage(); // Dùng để tính toán lượng near thừa khi deposit

        let collection_id = self.collections_by_name.len() as u32;

        let owner_id = env::predecessor_account_id();

        // Check collection_name đã tồn tại chưa
        assert!(
            self.collections_by_name.get(&collection_name).is_none(),
            "Collection name already exists! Must be unique!"
        );

        // Tạo collection mới
        let new_collection = Collection {
            collection_id,
            owner_id: owner_id.clone(),
            collection_name: collection_name.clone(),
            market_fee,
            data,
        };

        // Insert collection mới vào collections_per_owner
        let mut collection_set_for_account = self
            .collections_per_owner
            .get(&owner_id)
            .unwrap_or_else(|| {
                UnorderedSet::new(
                    StorageKey::CollectionsPerOwnerInnerKey {
                        account_id_hash: hash_account_id(&owner_id),
                    }
                    .try_to_vec()
                    .unwrap(),
                )
            });

        collection_set_for_account.insert(&collection_name.clone());
        self.collections_per_owner
            .insert(&owner_id, &collection_set_for_account);

        // Insert collection mới vào collections_by_id
        self.collections_by_id
            .insert(&collection_id, &new_collection);

        // Insert collection mới vào collections_by_name
        self.collections_by_name
            .insert(&collection_name, &new_collection);

        // Luợng data storage sử dụng = after_storage_usage - before_storage_usage
        let after_storage_usage = env::storage_usage();
        // Refund NEAR
        refund_deposit(after_storage_usage - before_storage_usage);

        new_collection
    }

    // Lấy tổng số Collections đang có trong contract
    pub fn collection_total_supply(&self) -> U128 {
        // Đếm tổng số lượng id đang có trong token_metadata_by_id
        U128(self.collections_by_name.len() as u128)
    }

    // Lấy tổng số Collections đang có của account nào đó
    pub fn collection_supply_for_owner(&self, account_id: AccountId) -> U128 {
        let collection_for_owner_set = self.collections_per_owner.get(&account_id);

        if let Some(collection_for_owner_set) = collection_for_owner_set {
            U128(collection_for_owner_set.len() as u128)
        } else {
            U128(0)
        }
    }

    // Lấy danh sách tất cả Collections trong Contract
    pub fn get_all_collections(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Collection> {
        let start = u128::from(from_index.unwrap_or(U128(0)));

        // Duyệt tất cả các keys -> Trả về Collection
        // self.collections_by_id.values_as_vector().to_vec()
        self.collections_by_name
            .iter()
            .skip(start as usize)
            .take(limit.unwrap() as usize)
            .map(|(collection_name, _collection)| self.collections_by_name.get(&collection_name).unwrap())
            .collect()
    }

    // Lấy danh sách collection của account nào đó (có pagination)
    pub fn get_all_collections_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Collection> {
        let collection_keys = self.collections_per_owner.get(&account_id);

        let keys = if let Some(collection_keys) = collection_keys {
            collection_keys
        } else {
            return vec![];
        };

        let start = u128::from(from_index.unwrap_or(U128(0)));

        // Duyệt tất cả các keys -> Trả về Collection
        keys.as_vector()
            .iter()
            .skip(start as usize) // Pagination
            .take(limit.unwrap_or(0) as usize) // Pagination
            .map(|collection_name| self.collections_by_name.get(&collection_name).unwrap())
            .collect()
    }

    // Search Collection theo name
    // Lấy về tất cả Collection mà tên có chứa ký tự của `search_string`
    pub fn get_collections_by_name(&self, search_string: String) -> Vec<Collection> {
        let collections_set: Vec<Collection> = self.collections_by_name.values().collect();

        let mut result = Vec::<Collection>::new();

        for collection in collections_set {
            if collection.collection_name.to_lowercase().contains(&search_string.to_lowercase()) {
                result.push(collection);
            }
        }
        result
    }
}
