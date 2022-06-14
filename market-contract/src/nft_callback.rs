use crate::*;

// Hàm nft_on_approve để nft market gọi cross-contract call sang
// Thực hiện cập nhật thông tin trạng thái, dữ liệu sales
pub trait NonFungibleTokenApprovalReceiver {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        token_by_template_id: TokenId, // Stt của NFT trong template nó thuộc vào
        collection_id: CollectionId,   // Id của Collection mà NFT thuộc vào
        collection_name: CollectionName, // Tên Collection mà NFT thuộc vào
        schema_id: SchemaId,           // Id của Schema mà NFT thuộc vào
        schema_name: SchemaName,       // Tên Schema mà NFT thuộc vào
        template_id: TemplateId,       // Tên Template mà NFT thuộc vào
        msg: String,
    );

    #[allow(unused_variables)]
    fn nft_on_approve_for_collection(
        &mut self,
        collection_name: CollectionName,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    ) {
    }
}

// Cấu trúc của msg
#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SaleArgs {
    pub sale_conditions: SalePriceInYoctoNear,
}

#[near_bindgen]
impl NonFungibleTokenApprovalReceiver for MarketContract {
    /**
     * msg: {"sale_conditions": "100000000000000"}
     */
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        token_by_template_id: TokenId, // Stt của NFT trong template nó thuộc vào
        collection_id: CollectionId,   // Id của Collection mà NFT thuộc vào
        collection_name: CollectionName, // Tên Collection mà NFT thuộc vào
        schema_id: SchemaId,           // Id của Schema mà NFT thuộc vào
        schema_name: SchemaName,       // Tên Schema mà NFT thuộc vào
        template_id: TemplateId,       // Tên Template mà NFT thuộc vào
        msg: String,
    ) {
        // User => NFT Contract => Market Contract
        // Signer account => Predecessor account => Current account
        let nft_contract_id = env::predecessor_account_id(); // NFT contract id chính là người gọi hàm
        let signer_id = env::signer_account_id();

        // NFT contract id và signer id không được trùng nhau
        // Nếu trùng nhau -> User đang gọi thẳng đến nft_on_approve của Market Contract
        assert_ne!(
            nft_contract_id, signer_id,
            "nft_on_approve should only be called via cross contract call"
        );
        assert_eq!(signer_id, owner_id, "owner_id should be signer_id");

        // --- Thêm mới Sale vào trong Market Contract ---
        // Check cover storage
        let storage_balance = self.storage_deposit_account.get(&signer_id).unwrap_or(0);
        let storage_minimum_amount = self.storage_minimun_balance().0; // .0 là hàm chuyển từ U128 -> u128
        let storage_required =
            (self.get_supply_by_owner_id(signer_id.clone()).0 + 1) * storage_minimum_amount;

        assert!(
            storage_balance >= storage_required,
            "Storage balance not enough for cover storage staking"
        );

        let SaleArgs { sale_conditions } =
            near_sdk::serde_json::from_str(&msg).expect("Not valid Sale Args"); // Parse msg từ String -> Json

        let contract_and_token_id =
            format!("{}{}{}", nft_contract_id.clone(), ".", token_id.clone());

        // Thêm vào sales
        self.sales.insert(
            &contract_and_token_id,
            &Sale {
                owner_id: owner_id.clone(),
                approval_id,
                nft_contract_id: nft_contract_id.clone(),
                token_id: token_id.clone(),
                token_by_template_id: token_by_template_id.clone(),
                collection_id: collection_id.clone(),
                collection_name: collection_name.clone(),
                schema_id: schema_id.clone(),
                schema_name: schema_name.clone(),
                template_id: template_id.clone(),
                sale_conditions,
            },
        );

        // Thêm vào tokens_by_owner_id
        // Nếu chưa tồn tại trong tokens_by_owner_id -> Tạo mới
        let mut tokens_by_owner_id = self.tokens_by_owner_id.get(&owner_id).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::InnerByOwnerIdKey {
                    account_id_hash: hash_account_id(&owner_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });

        tokens_by_owner_id.insert(&contract_and_token_id);
        self.tokens_by_owner_id
            .insert(&owner_id, &tokens_by_owner_id);

        // Thêm vào tokens_by_contract_id
        let mut tokens_by_contract_id = self
            .tokens_by_contract_id
            .get(&nft_contract_id)
            .unwrap_or_else(|| {
                UnorderedSet::new(
                    StorageKey::InnerByContractIdKey {
                        account_id_hash: hash_account_id(&nft_contract_id),
                    }
                    .try_to_vec()
                    .unwrap(),
                )
            });

        tokens_by_contract_id.insert(&token_id);
        self.tokens_by_contract_id
            .insert(&nft_contract_id, &tokens_by_contract_id);

        // TODO: Thêm vào collections_by_name
        // TODO: Thêm vào schemas_by_id
        // TODO: Thêm vào templates_by_id
    }

    // TODO : Implement nft on approve for collection callback
    #[allow(unused_variables)]
    fn nft_on_approve_for_collection(
        &mut self,
        collection_name: CollectionName,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    ) {
    }
}
