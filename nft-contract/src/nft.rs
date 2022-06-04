use crate::*;

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
        collection_id: CollectionId,
        schema_id: SchemaId,
        template_id: TemplateId,
        metadata: TokenMetadata,
        receiver_id: AccountId,
    ) {
        let token_id = self.tokens_by_id.len() as u32;
        let collection_name = self
            .collections_by_id
            .get(&collection_id)
            .expect("Collection does not exists")
            .collection_name;
        let schema_name = self
            .schemas_by_id
            .get(&schema_id)
            .expect("Schema does not exists")
            .schema_name;

        let before_storage_usage = env::storage_usage(); // Dùng để tính toán lượng near thừa khi deposit

        let token = Token {
            owner_id: receiver_id,
            token_id,
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

        // // ----- NFT MINT LOG -----
        // let nft_mint_log: EventLog = EventLog {
        //     standard: "nep171".to_string(),
        //     version: "1.0.0".to_string(),
        //     event: EventLogVariant::NftMint(vec![NftMintLog {
        //         owner_id: token.owner_id.to_string(),
        //         token_ids: vec![token_id.to_string()],
        //         memo: None,
        //     }]),
        // };
        // env::log(&nft_mint_log.to_string().as_bytes());

        // Luợng data storage sử dụng = after_storage_usage - before_storage_usage
        let after_storage_usage = env::storage_usage();
        // Refund NEAR
        refund_deposit(after_storage_usage - before_storage_usage);
    }
}
