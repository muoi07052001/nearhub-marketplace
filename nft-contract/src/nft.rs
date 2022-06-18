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
        collection_name: CollectionName,
        schema_id: SchemaId,
        template_id: TemplateId,
        // extra_metadata: TokenMetadata,
        receiver_id: AccountId,
    ) {
        let before_storage_usage = env::storage_usage(); // Dùng để tính toán lượng near thừa khi deposit

        // Check if the person who call this function is the Collection's owner or not
        let collection = self
            .collections_by_name
            .get(&collection_name)
            .expect("Collection does not exist");
        assert_eq!(
            collection.owner_id,
            env::predecessor_account_id(),
            "Only this Collection's owner can call this function"
        );

        self.internal_nft_mint(collection_name, schema_id, template_id, receiver_id);
       
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
    pub fn nft_tokens(&self, from_index: Option<u64>, limit: Option<u64>) -> Vec<JsonToken> {
        let collection_keys = self.token_metadata_by_id.keys_as_vector();

        // Duyệt tất cả các keys -> Trả về JsonToken
        collection_keys
            .iter()
            .skip(from_index.unwrap_or(0) as usize) // Pagination
            .take(limit.unwrap_or(10) as usize) // Pagination
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            .collect()
    }

    // Lấy danh sách token của account nào đó (có pagination)
    pub fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<u64>,
        limit: Option<u64>,
    ) -> Vec<JsonToken> {
        let token_keys = self.tokens_per_owner.get(&account_id);

        let keys = if let Some(token_keys) = token_keys {
            token_keys
        } else {
            return vec![];
        };

        // Duyệt tất cả các keys -> Trả về JsonToken
        keys.as_vector()
            .iter()
            .skip(from_index.unwrap_or(0) as usize) // Pagination
            .take(limit.unwrap_or(10) as usize) // Pagination
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            .collect()
    }
}
