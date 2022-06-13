use crate::*;

const DEFAULT_TOKEN_ID: u32 = 1000000000; // MAX u32 = 4294967295

#[near_bindgen]
impl NFTContract {
    /**
     * - Yêu cầu user nạp tiền để cover phí lưu trữ
     * - Thêm token (NFT) vào tokens_by_id
     * - Thêm token (NFT) metadata
     * - Thêm token (NFT) vào danh sách sở hữu bởi owner
     * - Refund lại NEAR user deposit thừa
     */
    #[payable]
    pub fn nft_mint(
        &mut self,
        collection_name: CollectionName,
        schema_id: SchemaId,
        template_id: TemplateId,
        metadata: TokenMetadata,
        receiver_id: AccountId,
    ) {
        let before_storage_usage = env::storage_usage(); // Dùng để tính toán lượng near thừa khi deposit

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

        // Thêm token metadata
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

        // Luợng data storage sử dụng = after_storage_usage - before_storage_usage
        let after_storage_usage = env::storage_usage();
        // Refund NEAR
        refund_deposit(after_storage_usage - before_storage_usage);
    }

    // Lấy thông tin 1 token dưới dạng JsonToken
    pub fn nft_token(&self, token_id: TokenId) -> Option<JsonToken> {
        let token = self.tokens_by_id.get(&token_id);

        if let Some(token) = token {
            let metadata = self.token_metadata_by_id.get(&token_id).unwrap();

            Some(JsonToken {
                owner_id: token.owner_id,
                token_id,
                token_by_template_id: token.token_by_template_id,
                collection_id: token.collection_id,
                collection_name: token.collection_name,
                schema_id: token.schema_id,
                schema_name: token.schema_name,
                template_id: token.template_id,
                metadata,
                approved_account_ids: token.approved_account_ids,
            })
        } else {
            None
        }
    }

    // -------------------------------- Enumerations --------------------------------

    // Lấy tổng số token đang có trong contract
    pub fn nft_total_supply(&self) -> U128 {
        // Đếm tổng số lượng id đang có trong token_metadata_by_id
        U128(self.token_metadata_by_id.len() as u128)
    }

    // Lấy tổng số token đang có của account nào đó
    pub fn nft_supply_for_owner(&self, account_id: AccountId) -> U128 {
        let token_for_owner_set = self.tokens_per_owner.get(&account_id);

        if let Some(token_for_owner_set) = token_for_owner_set {
            U128(token_for_owner_set.len() as u128)
        } else {
            U128(0)
        }
    }

    // Lấy danh sách token (có pagination)
    pub fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonToken> {
        let collection_keys = self.token_metadata_by_id.keys_as_vector();

        let start = u128::from(from_index.unwrap_or(U128(0)));

        // Duyệt tất cả các keys -> Trả về JsonToken
        collection_keys
            .iter()
            .skip(start as usize) // Pagination
            .take(limit.unwrap_or(0) as usize) // Pagination
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            .collect()
    }

    // Lấy danh sách token của account nào đó (có pagination)
    pub fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonToken> {
        let token_keys = self.tokens_per_owner.get(&account_id);

        let keys = if let Some(token_keys) = token_keys {
            token_keys
        } else {
            return vec![];
        };

        let start = u128::from(from_index.unwrap_or(U128(0)));

        // Duyệt tất cả các keys -> Trả về JsonToken
        keys.as_vector()
            .iter()
            .skip(start as usize) // Pagination
            .take(limit.unwrap_or(0) as usize) // Pagination
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            .collect()
    }
}
