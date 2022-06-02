// Các hàm cho Collections
use crate::*;

#[near_bindgen]
impl NFTContract {
    // Tạo 1 Collection mới
    pub fn create_collection(
        &mut self,
        collection_name: String,
        market_fee: f32,
        data: CollectionExtraData,
    ) -> Collection {
        let collection_id = self.collection_id_counter;

        let owner_id = env::predecessor_account_id();

        // Check collection_id đã tồn tại chưa
        assert!(
            self.collections_by_id.get(&collection_id).is_none(),
            "Collection id already exists"
        );

        // Tạo collection mới
        let new_collection = Collection {
            collection_id,
            owner_id: owner_id.clone(),
            collection_name,
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

        collection_set_for_account.insert(&collection_id.clone());
        self.collections_per_owner
            .insert(&owner_id, &collection_set_for_account);

        // Insert collection mới vào collections_by_id
        self.collections_by_id
            .insert(&collection_id, &new_collection);

        // Tăng collection_id_counter lên 1 đơn vị
        self.collection_id_counter += 1;

        new_collection
    }

    // Lấy tổng số Collections đang có trong contract
    pub fn collection_total_supply(&self) -> U128 {
        // Đếm tổng số lượng id đang có trong token_metadata_by_id
        U128(self.collections_by_id.len() as u128)
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
        self.collections_by_id
            .iter()
            .skip(start as usize)
            .take(limit.unwrap() as usize)
            .map(|(collection_id, _collection)| self.collections_by_id.get(&collection_id).unwrap())
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
            .map(|collection_id| self.collections_by_id.get(&collection_id).unwrap())
            .collect()
    }

    // Search Collection theo id
    pub fn get_collection_by_id(&self, collection_id: u32) -> Collection {
        self.collections_by_id.get(&collection_id).expect("Collection does not exist")
    }

    // Search Collection theo name
    // Lấy về tất cả Collection mà tên có chứa ký tự của `search_string`
    pub fn get_collections_by_name(&self, search_string: String) -> Vec<Collection> {
        let collections_set: Vec<Collection> = self.collections_by_id.values().collect();

        let mut result = Vec::<Collection>::new();

        for collection in collections_set {
            if collection.collection_name.to_lowercase().contains(&search_string.to_lowercase()) {
                result.push(collection);
            }
        }
        result
    }
}
