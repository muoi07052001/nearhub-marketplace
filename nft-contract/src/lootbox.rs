use crate::*;

#[near_bindgen]
impl NFTContract {
    // Tạo 1 Lootbox mới thuộc 1 Collection nào đó
    #[payable]
    pub fn create_lootbox(
        &mut self,
        // lootbox_name: String,
        // description: String,
        collection_name: CollectionName,
        schema_id: SchemaId,
        // img: Option<String>,
        transferable: bool,
        burnable: bool,
        max_supply: u32,
        immutable_data: ImmutableData,
        unlock_time: Option<Timestamp>,
        // display_data: Option<String>,
        config: Option<LootboxConfig>,
    ) -> Template {
        let before_storage_usage = env::storage_usage(); // Dùng để tính toán lượng near thừa khi deposit

        let lootbox_id = self.templates_by_id.len() as u32;

        self.lootbox_nft_by_lootbox_id_counter
            .insert(&lootbox_id, &0); // Khi tạo Lootbox -> Cho stt counter = 0

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
        for slot in config.clone().unwrap().iter() {
            for outcome in slot.outcomes.iter() {
                assert!(
                    self.templates_by_id.get(&outcome.template_id).is_some(),
                    "Template id inside this lootbox does not exists"
                );
            }
        }

        // TODO: Check Schema co thuoc Collection nay khong

        let schema_of_lootbox = self
            .schemas_by_id
            .get(&schema_id)
            .expect("Schema does not exist");

        // Tạo Lootbox mới
        let new_lootbox = Template {
            template_id: lootbox_id,
            collection_id: collection_of_lootbox_id,
            collection_name,
            schema_id,
            schema_name: schema_of_lootbox.schema_name,
            transferable,
            burnable,
            max_supply,
            issued_supply: 0,
            immutable_data,
            is_lootbox: true,
            unlock_time,
            config,
        };

        // Insert lootbox mới vào templates_by_id
        self.templates_by_id.insert(&lootbox_id, &new_lootbox);

        // Luợng data storage sử dụng = after_storage_usage - before_storage_usage
        let after_storage_usage = env::storage_usage();
        // Refund NEAR
        refund_deposit(after_storage_usage - before_storage_usage);

        new_lootbox
    }

    #[payable]
    pub fn mint_lootbox(
        &mut self,
        lootbox_id: LootboxId,
        mint_number: Option<u32>,
        receiver_id: AccountId,
    ) {
        let before_storage_usage = env::storage_usage(); // Dùng để tính toán lượng near thừa khi deposit

        // Check if the person who call this function is the Collection's owner or not
        let lootbox = self
            .templates_by_id
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

        // Check if the template is of kind lootbox or not
        assert_eq!(
            lootbox.is_lootbox, true,
            "This function can only be called on a Lootbox"
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
        lootbox_nft_id: TokenId,
        // metadata: TokenMetadata,
    ) {
        let mut rng = Rng::new(&env::random_seed());
        let receiver_id = env::predecessor_account_id();

        let lootbox_nft = self
            .tokens_by_id
            .get(&lootbox_nft_id)
            .expect("Lootbox Token does not exists");
        let lootbox_nft_metadata = self
            .token_metadata_by_id
            .get(&lootbox_nft_id)
            .expect("Lootbox Token metadata does not exist");

        // Check if this NFT is of type "Lootbox" or not
        // Only "Lootbox" can be unbox
        assert!(
            lootbox_nft_metadata.nft_type == "Lootbox".to_string(),
            "Only Lootbox can be unboxed"
        );

        let collection_of_lootbox = self
            .collections_by_name
            .get(&lootbox_nft.collection_name)
            .expect("Collection does not exist");

        // Check unbox must be call by Lootbox's owner
        assert_eq!(
            env::predecessor_account_id(),
            collection_of_lootbox.owner_id,
            "Only owner of this lootbox nft can unbox it!"
        );

        // Check current time is after lootbox.unlock_time or not
        let unbox_time = env::block_timestamp(); // Claim drop timestamp
        log!("Current time: {}", unbox_time);
        if lootbox_nft_metadata.starts_at.is_some() {
            // If lootbox.unclock_time == 0 -> Can unbox at any time
            assert!(
                unbox_time >= lootbox_nft_metadata.starts_at.unwrap(),
                "Cannot unbox this Lootbox during this time"
            );
        }

        let mut _result = 0; // result: template_id random ra được
        let mut result_arr = Vec::<u32>::new(); // result_arr: Mảng chứa kết quả các template_id phần thưởng trong Lootbox

        let config: LootboxConfig =
            serde_json::from_str(&lootbox_nft_metadata.extra.unwrap().as_str())
                .expect("Not valid Config");

        // Duyệt mảng config của lootbox
        for slot in config.iter() {
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
                lootbox_nft.collection_name.clone(),
                template.schema_id,
                template.template_id,
                // metadata.clone(),
                Some(1),
                receiver_id.clone(),
            );
        }

        // Delete Lootbox NFT
        self.tokens_by_id.remove(&lootbox_nft_id);

        // Delete Lootbox NFT Metadata
        self.token_metadata_by_id.remove(&lootbox_nft_id);

        // Delete Lootbox NFT from Owner
        let mut tokens_per_owner_set = self
            .tokens_per_owner
            .get(&lootbox_nft.owner_id)
            .expect("Not found Tokens set of this account");

        tokens_per_owner_set.remove(&lootbox_nft_id);
        // Update data
        self.tokens_per_owner
            .insert(&lootbox_nft.owner_id, &tokens_per_owner_set);
    }

    //  -------------------------------------- ENUMERATION --------------------------------------
    // Lấy tổng số Lootboxes đang có trong contract
    pub fn lootbox_total_supply(&self) -> U128 {
        // Đếm tổng số lượng id đang có trong token_metadata_by_id

        let template_set: Vec<Template> = self.templates_by_id.values().collect();
        let mut lootbox_set = Vec::<Template>::new();

        for template in template_set {
            if template.is_lootbox == true {
                lootbox_set.push(template);
            }
        }

        U128(lootbox_set.len() as u128)
    }

    // Lấy tổng số Lootboxes đang có của Collection nào đó
    pub fn lootbox_supply_by_collection(&self, collection_name: CollectionName) -> U128 {
        // Check collection id có tồn tại không
        assert!(
            self.collections_by_name.get(&collection_name).is_some(),
            "Collection does not exist"
        );

        let mut count = 0;

        let templates_set_by_collection: Vec<Template> = self
            .templates_by_id
            .keys()
            .map(|template_id| self.templates_by_id.get(&template_id).unwrap())
            .collect();

        for template in templates_set_by_collection {
            if template.collection_name == collection_name && template.is_lootbox == true {
                count += 1;
            }
        }

        U128(count)
    }
}
