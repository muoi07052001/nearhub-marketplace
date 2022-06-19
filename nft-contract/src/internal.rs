use crate::*;

#[near_bindgen]
impl NFTContract {
    // Thêm 1 token vào danh sách sở hữu bởi owner
    pub(crate) fn internal_add_token_to_owner(
        &mut self,
        token_id: &TokenId,
        account_id: &AccountId,
    ) {
        // Nếu account_id đã có danh sách token rồi, thì sẽ lấy danh sách token đang có
        // Nếu account_id chưa có danh sách token (account_id chưa có trong tokens_per_owner) thì tạo mới tokens_set
        let mut tokens_set = self.tokens_per_owner.get(account_id).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::TokensPerOwnerInnerKey {
                    account_id_hash: hash_account_id(account_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });

        // Thêm token vào danh sách sở hữu của account_id
        tokens_set.insert(&token_id);

        // Update dữ liệu on-chain
        self.tokens_per_owner.insert(account_id, &tokens_set);
    }

    // Thêm 1 Lootbox NFT vào danh sách sở hữu bởi owner
    pub(crate) fn internal_add_lootbox_nft_to_owner(
        &mut self,
        lootbox_nft_id: &LootboxNftId,
        account_id: &AccountId,
    ) {
        // Nếu account_id đã có danh sách Lootbox NFT rồi, thì sẽ lấy danh sách Lootbox NFT đang có
        // Nếu account_id chưa có danh sách Lootbox NFT (account_id chưa có trong lootbox_nfts_per_owner) thì tạo mới lootbox_nfts_set
        let mut lootbox_nfts_set = self.lootbox_nfts_per_owner.get(account_id).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::TokensPerOwnerInnerKey {
                    account_id_hash: hash_account_id(account_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });

        // Thêm token vào danh sách sở hữu của account_id
        lootbox_nfts_set.insert(&lootbox_nft_id);

        // Update dữ liệu on-chain
        self.lootbox_nfts_per_owner.insert(account_id, &lootbox_nfts_set);
    }


    // Xoá token khỏi owner
    pub(crate) fn internal_remove_token_from_owner(
        &mut self,
        token_id: &TokenId,
        account_id: &AccountId,
    ) {
        let mut tokens_set = self
            .tokens_per_owner
            .get(account_id)
            .expect("Token should be owned by sender");

        // Xoá token_id khỏi tokens_set
        tokens_set.remove(token_id);
        // Nếu xoá token xong tokens_set của account rỗng -> Xoá luôn account_id khỏi tokens_per_owner
        // Ngược lại -> Cập nhật list tokens_per_owner
        if tokens_set.is_empty() {
            self.tokens_per_owner.remove(account_id);
        } else {
            self.tokens_per_owner.insert(account_id, &tokens_set);
        }
    }

    // Return data token cũ trước khi thực hiện transfer
    /**
     * - Kiểm tra token_id có tồn tại không?
     * - sender_id có phải là owner của token hay không?
     * - sender_id và receiver_id trùng nhau (gửi cho chính mình) không?
     * - Xoá token khỏi owner cũ
     * - Thêm token cho receiver_id
     */
    pub(crate) fn internal_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        token_id: &TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) -> Token {
        // Kiểm tra token_id có tồn tại không?
        let token = self.tokens_by_id.get(token_id).expect("Not found token");
        // sender_id có phải là owner của token hay không?
        // Nếu sender_id không phải owner của token -> Check xem sender_id có phải approval_id không (có quyền transfer token thay owner không)
        // Chỉ có owner của Token HOẶC người được approved mới có quyền transfer Token
        if sender_id != &token.owner_id {
            if !token.approved_account_ids.contains_key(sender_id) {
                env::panic("Sender must be the token owner or the approved account".as_bytes());
            }

            if let Some(enforced_approval_id) = approval_id {
                let actual_approval_id = token
                    .approved_account_ids
                    .get(sender_id)
                    .expect("Sender is not approved to transfer token");

                assert_eq!(
                    actual_approval_id, &enforced_approval_id,
                    "The actual approval id {} is different from the given approval id {}",
                    actual_approval_id, enforced_approval_id
                );
            }
        };
        // sender_id và receiver_id trùng nhau (gửi cho chính mình) không?
        assert_ne!(
            &token.owner_id, receiver_id,
            "The token owner and the receiver should be different"
        );

        // Xoá token khỏi owner cũ
        self.internal_remove_token_from_owner(&token_id, &token.owner_id);
        // Thêm token cho receiver_id
        self.internal_add_token_to_owner(&token_id, receiver_id);

        let new_token = Token {
            owner_id: receiver_id.clone(),
            token_id: token.token_id.clone(),
            token_by_template_id: token.token_by_template_id.clone(),
            collection_id: token.collection_id.clone(),
            collection_name: token.collection_name.clone(),
            schema_id: token.schema_id.clone(),
            schema_name: token.schema_name.clone(),
            template_id: token.template_id.clone(),
            approved_account_ids: HashMap::default(), // Sau khi chuyển token cho người khác, xoá toàn bộ approved_account_ids
            next_approval_id: token.next_approval_id,
        };

        // Thêm token mới vào list tất cả tokens
        self.tokens_by_id.insert(token_id, &new_token);

        // Nếu có memo thì in ra memo
        if let Some(memo) = memo.as_ref() {
            log!("Memo: {}", memo);
        }

        // -------------------------------------------------------------------
        // NFT TRANSFER LOG
        let mut authorized_id = None;
        // Nếu có approval_id -> authorized_id chính là người gửi NFT
        if approval_id.is_some() {
            authorized_id = Some(sender_id.to_string());
        }

        let nft_transfer_log: EventLog = EventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::NftTransfer(vec![NftTransferLog {
                authorized_id,
                old_owner_id: token.owner_id.to_string(),
                new_owner_id: receiver_id.to_string(),
                token_ids: vec![token_id.to_string()],
                memo,
            }]),
        };

        env::log(&nft_transfer_log.to_string().as_bytes());
        // -------------------------------------------------------------------

        // Return token cũ
        token
    }

    // Internal mint NFTs
    pub(crate) fn internal_nft_mint(
        &mut self,
        collection_name: CollectionName,
        schema_id: SchemaId,
        template_id: TemplateId,
        receiver_id: AccountId,
    ) {
        let token_id = DEFAULT_TOKEN_ID + self.tokens_by_id.len() as u32; // TokeId: 1000000001, ...

        // Lấy ra stt của NFT hiện tại trong Template này
        let mut token_by_template_id = self
            .token_by_template_id_counter
            .get(&template_id)
            .expect("Not found Template");

        // Check collection_name có tồn tại không
        // Lấy collection id từ name
        let collection = self
            .collections_by_name
            .get(&collection_name)
            .expect("Collection does not exists");
        let collection_id = collection.collection_id;

        // Check schema_id có tồn tại không
        // Lấy schema name từ id
        let schema = self
            .schemas_by_id
            .get(&schema_id)
            .expect("Schema does not exists");
        let schema_name = schema.schema_name;

        // Check template_id có tồn tại không
        // Lấy template name từ id
        let mut template = self
            .templates_by_id
            .get(&template_id)
            .expect("Template does not exists");
        let _template_name = template.schema_name.clone();

        // Check xem schema_id đó có thuộc collection_id đó không
        assert_eq!(
            schema.collection_name, collection_name,
            "Schema does not belongs to this collection"
        );
        // Check xem template_id đó có thuộc schema_id đó không
        assert_eq!(
            template.schema_name, schema_name,
            "Template does not belongs to this schema"
        );

        // Check if that template has issued all the NFTs or not
        assert!(
            template.issued_supply < template.max_supply,
            "This template has issued all the NFTs"
        );
        // Tạo NFT mới
        let token = Token {
            owner_id: receiver_id,
            token_id,
            token_by_template_id,
            collection_id,
            collection_name,
            schema_id,
            schema_name,
            template_id,
            approved_account_ids: HashMap::default(),
            next_approval_id: 0,
        };

        // Nếu token_id đã tồn tại trong list tokens_by_id thì báo lỗi
        // Trong LookupMap, nếu key chưa tồn tại trong map -> Hàm insert return None
        assert!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        // Add token metadata due to Template's immutable data
        let metadata = TokenMetadata {
            title: Some(template.immutable_data.name.clone()),
            description: None,
            media: template.immutable_data.img.clone(),
            media_hash: None,
            copies: Some(template.max_supply as u64),
            issued_at: Some(env::block_timestamp()),
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: template.immutable_data.extra_immutable_data.clone(),
            reference: None,
            reference_hash: None,
            nft_type: "NFT".to_string(),
        };

        self.token_metadata_by_id.insert(&token_id, &metadata);

        // Thêm token vào danh sách sở hữu bởi owner
        self.internal_add_token_to_owner(&token_id, &token.owner_id);

        // Update stt của NFT hiện tại trong token_by_template_id_counter
        token_by_template_id += 1;
        self.token_by_template_id_counter
            .insert(&template_id, &token_by_template_id);

        // -------------------------------------------------------------------
        // NFT MINT LOG
        let nft_mint_log: EventLog = EventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::NftMint(vec![NftMintLog {
                owner_id: token.owner_id.to_string(),
                token_ids: vec![token_id.to_string()],
                memo: None,
            }]),
        };
        env::log(&nft_mint_log.to_string().as_bytes());
        // -------------------------------------------------------------------

        // Increase issued_supply of this template by 1
        template.issued_supply += 1;
        // Update data of template
        self.templates_by_id.insert(&template_id, &template);
    }

    pub(crate) fn internal_lootbox_nft_mint(
        &mut self,
        lootbox_id: LootboxId,
        receiver_id: AccountId,
    ) {
        let lootbox_nft_id = DEFAULT_LOOTBOX_NFT_ID + self.lootbox_nfts_by_id.len() as u32; // TokeId: 1000000001, ...

        // Lấy ra stt của NFT Lootbox hiện tại trong Lootbox này
        let mut lootbox_nft_by_lootbox_id = self
            .lootbox_nft_by_lootbox_id_counter
            .get(&lootbox_id)
            .expect("Not found Lootbox NFT's number");

        // Get the information of the Lootbox
        let lootbox = self.lootboxes_by_id.get(&lootbox_id).expect("Not found Lootbox");
        let collection_of_lootbox = self.collections_by_name.get(&lootbox.collection_name).expect("Not found Collection");

        // Tạo NFT mới
        let lootbox_nft = LootboxNft {
            owner_id: receiver_id.clone(),
            lootbox_nft_id,
            lootbox_id,
            lootbox_nft_by_lootbox_id,
            collection_id: collection_of_lootbox.collection_id,
            collection_name: collection_of_lootbox.collection_name,
            approved_account_ids: HashMap::default(),
            next_approval_id: 0,
        };

        // Nếu token_id đã tồn tại trong list tokens_by_id thì báo lỗi
        // Trong LookupMap, nếu key chưa tồn tại trong map -> Hàm insert return None
        assert!(
            self.lootbox_nfts_by_id.insert(&lootbox_nft_id, &lootbox_nft).is_none(),
            "NFT Lootbox already exists"
        );

        // Add token metadata due to Template's immutable data
        let metadata = TokenMetadata {
            title: lootbox.display_data.clone(), // TODO: Define name in display_data
            description: None,
            media: lootbox.img.clone(),
            media_hash: None,
            copies: None,
            issued_at: Some(env::block_timestamp()),
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
            nft_type: "Lootbox".to_string(),
        };

        self.lootbox_nft_metadata_by_id.insert(&lootbox_nft_id, &metadata);

        // Thêm Lootbox NFT vào danh sách sở hữu bởi owner
        self.internal_add_lootbox_nft_to_owner(&lootbox_nft_id, &receiver_id);

        // Update stt của Lootbox NFT hiện tại trong lootbox_nft_by_lootbox_id_counter
        lootbox_nft_by_lootbox_id += 1;
        self.lootbox_nft_by_lootbox_id_counter
            .insert(&lootbox_id, &lootbox_nft_by_lootbox_id);

        // -------------------------------------------------------------------
        // NFT MINT LOG
        let nft_mint_log: EventLog = EventLog {
            standard: "nep171".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::NftMint(vec![NftMintLog {
                owner_id: receiver_id.to_string(),
                token_ids: vec![lootbox_nft_id.to_string()],
                memo: None,
            }]),
        };
        env::log(&nft_mint_log.to_string().as_bytes());
        // -------------------------------------------------------------------

        // // Increase issued_supply of this template by 1
        // template.issued_supply += 1;
        // // Update data of template
        // self.templates_by_id.insert(&template_id, &template);
    }
}
