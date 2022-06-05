use crate::*;

#[near_bindgen]
impl NFTContract {
    // Tạo 1 Lootbox mới thuộc 1 Collection nào đó
    pub fn create_lootbox(
        &mut self,
        lootbox_name: String,
        description: String,
        collection_id: CollectionId,
        unlock_time: Option<u32>,
        display_data: Option<String>,
        config: LootboxConfig,
    ) -> Lootbox {
        let lootbox_id = self.lootboxes_by_id.len() as u32;

        // Check lootbox_id đã tồn tại chưa
        assert!(
            self.lootboxes_by_id.get(&lootbox_id).is_none(),
            "Lootbox id already exists"
        );

        // Check collection_id có tồn tại không
        // Lấy collection name từ id
        let collection_of_lootbox = self
            .collections_by_id
            .get(&collection_id)
            .expect("Collection not exists");
        let collection_of_lootbox_name = collection_of_lootbox.collection_name;

        // TODO: Check từng template_id trong `config` có thuộc collection_id này không

        // Tạo collection mới
        let new_lootbox = Lootbox {
            lootbox_id,
            lootbox_name,
            description,
            collection_id,
            collection_name: collection_of_lootbox_name,
            unlock_time,
            display_data,
            config,
        };

        // Insert lootbox mới vào lootboxes_by_id
        self.lootboxes_by_id.insert(&lootbox_id, &new_lootbox);

        new_lootbox
    }

    // Lấy tổng số Lootboxes đang có trong contract
    pub fn lootbox_total_supply(&self) -> U128 {
        // Đếm tổng số lượng id đang có trong token_metadata_by_id
        U128(self.lootboxes_by_id.len() as u128)
    }

    // Lấy tổng số Lootboxes đang có của Collection nào đó
    pub fn lootbox_supply_by_collection(&self, collection_id: CollectionId) -> U128 {
        // Check collection id có tồn tại không
        assert!(self.collections_by_id.get(&collection_id).is_some(), "Collection does not exist");

        let mut count = 0;

        let lootboxes_set_by_collection: Vec<Lootbox> = self
            .lootboxes_by_id
            .keys()
            .map(|lootbox_id| self.lootboxes_by_id.get(&lootbox_id).unwrap())
            .collect();

        for lootbox in lootboxes_set_by_collection {
            if lootbox.collection_id == collection_id {
                count += 1;
            }
        }

        U128(count)
    }

    // Lấy danh sách tất cả Lootboxes trong Contract
    pub fn get_all_lootboxes(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Lootbox> {
        let start = u128::from(from_index.unwrap_or(U128(0)));

        // Duyệt tất cả các keys -> Trả về Collection
        // self.collections_by_id.values_as_vector().to_vec()
        self.lootboxes_by_id
            .iter()
            .skip(start as usize)
            .take(limit.unwrap() as usize)
            .map(|(lootbox_id, _lootbox)| self.lootboxes_by_id.get(&lootbox_id).unwrap())
            .collect()
    }

    // Lấy danh sách Lootbox của Collection nào đó (có pagination)
    pub fn get_all_lootboxes_by_collection(
        &self,
        collection_id: CollectionId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Lootbox> {
        // Check collection id có tồn tại không
        assert!(self.collections_by_id.get(&collection_id).is_some(), "Collection does not exist");

        let start = u128::from(from_index.unwrap_or(U128(0)));

        let mut result = Vec::<Lootbox>::new();

        // Duyệt tất cả các keys -> Trả về Collection
        let lootboxes_set_for_owner: Vec<Lootbox> = self
            .lootboxes_by_id
            .keys()
            .skip(start as usize) // Pagination
            .take(limit.unwrap_or(0) as usize) // Pagination
            .map(|lootbox_id| self.lootboxes_by_id.get(&lootbox_id).unwrap())
            .collect();

        for lootbox in lootboxes_set_for_owner {
            if lootbox.collection_id == collection_id {
                result.push(lootbox);
            }
        }
        result
    }
}
