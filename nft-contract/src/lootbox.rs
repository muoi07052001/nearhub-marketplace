use crate::*;

#[near_bindgen]
impl NFTContract {
    // Tạo 1 Lootbox mới thuộc 1 Collection nào đó
    #[payable]
    pub fn create_lootbox(
        &mut self,
        lootbox_name: String,
        description: String,
        collection_name: CollectionName,
        img: Option<String>,
        unlock_time: Timestamp,
        display_data: Option<String>,
        config: LootboxConfig,
    ) -> Lootbox {
        let before_storage_usage = env::storage_usage(); // Dùng để tính toán lượng near thừa khi deposit

        let lootbox_id = self.lootbox_id_counter;

        self.lootbox_nft_by_lootbox_id_counter.insert(&lootbox_id, &0); // Khi tạo Lootbox -> Cho stt counter = 0

        // Check collection_id có tồn tại không
        // Lấy collection name từ id
        let collection_of_lootbox = self
            .collections_by_name
            .get(&collection_name)
            .expect("Collection not exists");
        let collection_of_lootbox_id = collection_of_lootbox.collection_id;

        // Check signer id is Collection's owner or not
        assert_eq!(
            collection_of_lootbox.owner_id,
            env::predecessor_account_id(),
            "Only owner of this collection can create Lootbox"
        );

        // Check từng template_id trong `config` có thuộc collection_id này không
        for slot in config.iter() {
            for outcome in slot.outcomes.iter() {
                assert!(
                    self.templates_by_id.get(&outcome.template_id).is_some(),
                    "Template id inside this lootbox does not exists"
                );
            }
        }

        // Tạo collection mới
        let new_lootbox = Lootbox {
            lootbox_id,
            lootbox_name,
            img,
            description,
            collection_id: collection_of_lootbox_id,
            collection_name,
            unlock_time,
            display_data,
            config,
        };

        // Insert lootbox mới vào lootboxes_by_id
        self.lootboxes_by_id.insert(&lootbox_id, &new_lootbox);

        self.lootbox_id_counter += 1;

        // Luợng data storage sử dụng = after_storage_usage - before_storage_usage
        let after_storage_usage = env::storage_usage();
        // Refund NEAR
        refund_deposit(after_storage_usage - before_storage_usage);

        new_lootbox
    }

    #[payable]
    pub fn mint_lootbox(&mut self, lootbox_id: LootboxId, mint_number: Option<u32>, receiver_id: AccountId) {
        let before_storage_usage = env::storage_usage(); // Dùng để tính toán lượng near thừa khi deposit

        // Check if the person who call this function is the Collection's owner or not
        let lootbox = self
            .lootboxes_by_id
            .get(&lootbox_id)
            .expect("Lootbox does not exist");
        let collection = self
            .collections_by_name
            .get(&lootbox.collection_name)
            .expect("Collection does not exist");
        assert_eq!(
            collection.owner_id,
            env::predecessor_account_id(),
            "Only this Lootbox's owner can call this function"
        );

        self.internal_lootbox_nft_mint(lootbox_id, mint_number, receiver_id);

        let after_storage_usage = env::storage_usage();
        // Refund NEAR
        refund_deposit(after_storage_usage - before_storage_usage);
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
    ) {
        let mut rng = Rng::new(&env::random_seed());
        let receiver_id = env::predecessor_account_id();

        let lootbox = self
            .lootboxes_by_id
            .get(&lootbox_id)
            .expect("Lootbox does not exists");

        let collection_of_lootbox = self
            .collections_by_name
            .get(&lootbox.collection_name)
            .expect("Collection does not exist");

        // Check unbox must be call by Lootbox's owner
        assert_eq!(
            env::predecessor_account_id(),
            collection_of_lootbox.owner_id,
            "Only owner of this lootbox can unbox it!"
        );

        // Check current time is after lootbox.unlock_time or not
        let unbox_time = env::block_timestamp(); // Claim drop timestamp
        log!("Current time: {}", unbox_time);
        if lootbox.unlock_time != 0 {
            // If lootbox.unclock_time == 0 -> Can unbox at any time
            assert!(
                unbox_time >= lootbox.unlock_time,
                "Cannot unbox this Lootbox during this time"
            );
        }

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

            // Mint ra NFT dựa trên result (template_id)
            // Chuyển NFT cho receiver_id
            self.internal_nft_mint(
                lootbox.collection_name.clone(),
                template.schema_id,
                template.template_id,
                // metadata.clone(),
                Some(1),
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
    pub fn get_all_lootboxes(&self, from_index: Option<u64>, limit: Option<u64>) -> Vec<Lootbox> {
        // Duyệt tất cả các keys -> Trả về Collection
        // self.collections_by_id.values_as_vector().to_vec()
        self.lootboxes_by_id
            .iter()
            .skip(from_index.unwrap_or(0) as usize)
            .take(limit.unwrap_or(10) as usize)
            .map(|(lootbox_id, _lootbox)| self.lootboxes_by_id.get(&lootbox_id).unwrap())
            .collect()
    }

    // Lấy danh sách Lootbox của Collection nào đó (có pagination)
    pub fn get_all_lootboxes_by_collection(
        &self,
        collection_name: CollectionName,
        from_index: Option<u64>,
        limit: Option<u64>,
    ) -> Vec<Lootbox> {
        let mut count = 0;

        // Check collection id có tồn tại không
        assert!(
            self.collections_by_name.get(&collection_name).is_some(),
            "Collection does not exist"
        );

        let mut result = Vec::<Lootbox>::new();

        // Duyệt tất cả các keys -> Trả về Collection
        let lootboxes_set_for_owner: Vec<Lootbox> = self
            .lootboxes_by_id
            .keys()
            .skip(from_index.unwrap_or(0) as usize) // Pagination
            .take(limit.unwrap_or(10) as usize) // Pagination
            .map(|lootbox_id| self.lootboxes_by_id.get(&lootbox_id).unwrap())
            .collect();

        // If limit = 0 -> Return empty Array
        if limit.unwrap() == 0 {
            return result;
        }
        
        for lootbox in lootboxes_set_for_owner {
            if lootbox.collection_name == collection_name {
                result.push(lootbox);
                count += 1;
            }
            if count == limit.unwrap_or(10) {
                break;
            }
        }
        result
    }

    // -------------------------------- Enumerations --------------------------------
    // Lấy thông tin 1 lootbox_nft dưới dạng JsonToken
    pub fn lootbox_nft(&self, lootbox_nft_id: LootboxNftId) -> Option<JsonLootboxNft> {
        let lootbox_nft = self.lootbox_nfts_by_id.get(&lootbox_nft_id);

        if let Some(lootbox_nft) = lootbox_nft {
            let metadata = self
                .lootbox_nft_metadata_by_id
                .get(&lootbox_nft_id)
                .unwrap();

            Some(JsonLootboxNft {
                owner_id: lootbox_nft.owner_id, 
                lootbox_nft_id: lootbox_nft.lootbox_nft_id, 
                lootbox_id: lootbox_nft.lootbox_id, 
                lootbox_nft_by_lootbox_id: lootbox_nft.lootbox_nft_by_lootbox_id, 
                collection_id: lootbox_nft.collection_id, 
                collection_name: lootbox_nft.collection_name, 
                metadata: metadata,
                approved_account_ids: lootbox_nft.approved_account_ids, 
                next_approval_id: lootbox_nft.next_approval_id,
            })
        } else {
            None
        }
    }

    // Lấy tổng số Lootbox NFT đang có trong contract
    pub fn lootbox_nft_total_supply(&self) -> U128 {
        // Đếm tổng số lượng id đang có trong token_metadata_by_id
        U128(self.lootbox_nft_metadata_by_id.len() as u128)
    }

    // Lấy tổng số Lootbox NFT đang có của account nào đó
    pub fn lootbox_nft_supply_for_owner(&self, account_id: AccountId) -> U128 {
        let lootbox_nft_for_owner_set = self.lootbox_nfts_per_owner.get(&account_id);

        if let Some(lootbox_nft_for_owner_set) = lootbox_nft_for_owner_set {
            U128(lootbox_nft_for_owner_set.len() as u128)
        } else {
            U128(0)
        }
    }

    // Lấy danh sách Lootbox NFT (có pagination)
    pub fn lootbox_nfts(&self, from_index: Option<u64>, limit: Option<u64>) -> Vec<JsonLootboxNft> {
        let collection_keys = self.lootbox_nft_metadata_by_id.keys_as_vector();

        // Duyệt tất cả các keys -> Trả về JsonToken
        collection_keys
            .iter()
            .skip(from_index.unwrap_or(0) as usize) // Pagination
            .take(limit.unwrap_or(10) as usize) // Pagination
            .map(|lootbox_nft_id| self.lootbox_nft(lootbox_nft_id.clone()).unwrap())
            .collect()
    }

    // Lấy danh sách Lootbox NFT của account nào đó (có pagination)
    pub fn lootbox_nfts_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<u64>,
        limit: Option<u64>,
    ) -> Vec<JsonLootboxNft> {
        let lootbox_nft_keys = self.lootbox_nfts_per_owner.get(&account_id);

        let keys = if let Some(lootbox_nft_keys) = lootbox_nft_keys {
            lootbox_nft_keys
        } else {
            return vec![];
        };

        // Duyệt tất cả các keys -> Trả về JsonToken
        keys.as_vector()
            .iter()
            .skip(from_index.unwrap_or(0) as usize) // Pagination
            .take(limit.unwrap_or(10) as usize) // Pagination
            .map(|lootbox_nft_id| self.lootbox_nft(lootbox_nft_id.clone()).unwrap())
            .collect()
    }
}
