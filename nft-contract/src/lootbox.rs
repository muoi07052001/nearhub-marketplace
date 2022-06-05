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
}
