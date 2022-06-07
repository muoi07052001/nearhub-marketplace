use crate::*;

#[near_bindgen]
impl NFTContract {
    // Tạo 1 Lootbox mới thuộc 1 Collection nào đó
    pub fn create_lootbox(
        &mut self,
        lootbox_name: String,
        description: String,
        collection_name: CollectionName,
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
            .collections_by_name
            .get(&collection_name)
            .expect("Collection not exists");
        let collection_of_lootbox_id = collection_of_lootbox.collection_id;

        // TODO: Check từng template_id trong `config` có thuộc collection_id này không

        // Tạo collection mới
        let new_lootbox = Lootbox {
            lootbox_id,
            lootbox_name,
            description,
            collection_id: collection_of_lootbox_id,
            collection_name,
            unlock_time,
            display_data,
            config,
        };

        // Insert lootbox mới vào lootboxes_by_id
        self.lootboxes_by_id.insert(&lootbox_id, &new_lootbox);

        new_lootbox
    }

    // Unbox Lootbox -> Mint NFTs base on Config (Ratio)
    /**
     * Số lần quay random (số NFT nhận được) = outcomes.len() (mỗi 1 phần tử của outcomes là 1 slot NFT)
     * Duyệt config => Quay random từ 0 -> total_odds -> Ra số thuộc khoảng nào thì mint ra NFT thuộc template_id tương ứng
     * Transfer NFT cho receiver_id
     * Xoá Lootbox khỏi lootboxes_by_id
     */
    #[payable]
    pub fn unbox_lootbox(
        &mut self,
        lootbox_id: LootboxId,
        // metadata: TokenMetadata,
        receiver_id: AccountId,
    ) {
        let mut rng = Rng::new(&env::random_seed());

        let lootbox = self
            .lootboxes_by_id
            .get(&lootbox_id)
            .expect("Lootbox does not exists");

        let mut _result = 0; // result: template_id random ra được
        let mut result_arr = Vec::<u32>::new(); // result_arr: Mảng chứa kết quả các template_id phần thưởng trong Lootbox
                                                // Duyệt mảng config của lootbox
        for slot in lootbox.config.iter() {
            // Trả về kết quả random -> quyết định mint ra NFT thuộc template_id nào
            // Random từ 0 -> total_odds
            let value = rng.rand_range_u32(0, slot.total_odds);

            let mut result = 0;
            // Lấy random value
            for i in 0..slot.outcomes.len() - 1 {
                // If 0 <= value < slot.outcomes[0]
                if value < slot.outcomes[0].odds {
                    result = slot.outcomes[0].template_id;
                } else if value >= slot.outcomes[i].odds && value < slot.outcomes[i + 1].odds {
                    result = slot.outcomes[i + 1].template_id;
                }
            }
            result_arr.push(result);
        }

        for template_id in result_arr.iter() {
            // Lấy ra schema_id mà template_id thuộc vào
            let template = self
                .templates_by_id
                .get(template_id)
                .expect("Template id does not exists");

            // ----------------------------------------------
            // ------------- TODO: Tạo Metadata -------------
            // ----------------------------------------------
            let metadata: TokenMetadata = TokenMetadata {
                title: None,
                description: None,
                media: None,
                media_hash: None,
                copies: None,
                issued_at: None,
                expires_at: None,
                starts_at: None,
                updated_at: None,
                extra: Some("{\"attack\": 10}".to_string()),
                reference: None,
                reference_hash: None,
            };

            // Mint ra NFT dựa trên result (template_id)
            // Chuyển NFT cho receiver_id
            self.nft_mint(
                lootbox.collection_name.clone(),
                template.schema_id,
                template.template_id,
                metadata.clone(),
                receiver_id.clone(),
            );
        }

        // Xoá Lootbox
        self.lootboxes_by_id.remove(&lootbox_id);
    }

    //  -------------------------------------- ENUMERATION --------------------------------------
    // Lấy tổng số Lootboxes đang có trong contract
    pub fn lootbox_total_supply(&self) -> U128 {
        // Đếm tổng số lượng id đang có trong token_metadata_by_id
        U128(self.lootboxes_by_id.len() as u128)
    }

    // Lấy tổng số Lootboxes đang có của Collection nào đó
    pub fn lootbox_supply_by_collection(&self, collection_name: CollectionName) -> U128 {
        // Check collection id có tồn tại không
        assert!(
            self.collections_by_name.get(&collection_name).is_some(),
            "Collection does not exist"
        );

        let mut count = 0;

        let lootboxes_set_by_collection: Vec<Lootbox> = self
            .lootboxes_by_id
            .keys()
            .map(|lootbox_id| self.lootboxes_by_id.get(&lootbox_id).unwrap())
            .collect();

        for lootbox in lootboxes_set_by_collection {
            if lootbox.collection_name == collection_name {
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
        collection_name: CollectionName,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Lootbox> {
        // Check collection id có tồn tại không
        assert!(
            self.collections_by_name.get(&collection_name).is_some(),
            "Collection does not exist"
        );

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
            if lootbox.collection_name == collection_name {
                result.push(lootbox);
            }
        }
        result
    }
}