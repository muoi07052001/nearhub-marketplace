use crate::*;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DropSale {
    pub drop_id: DropId,                 // Id of the Drop Sale (auto increment)
    pub owner_id: AccountId,             // Owner account of the Drop Sale
    pub collection_name: CollectionName, // Collection that the Drop Sale belongs to
    pub template_ids: Vec<TemplateId>,   // Array of template_id that contains inside the Drop Sale
    pub price: f32,                      // Price of the Drop Sale
    pub price_type: String,              // Price Unit: (USDT | NEAR)
    pub is_public: bool,                 // Decide the Drop Sale is public for everyone or not
    pub max_supply: u32,                 // Total issued NFTs of the Drop
    pub account_limit: u32,              // The limit of how many NFTs can 1 account buy at a time
    pub account_limit_cooldown: String,  // The cooldown time between each buy of 1 account
    pub start_time: String, // When will the user can buy the Drop Sale (0 if the user can buy immediate after Drop Sale created)
    pub end_time: String, // When will the user can't buy the Drop Sale anymore (0 if don't have limit time)
    pub display_data: Option<String>, // Display data of the Drop Sale: Name, ...
    pub approved_account_ids: HashMap<AccountId, u64>, // Danh sách các accounts được approved để mua Drop Sale này
    pub next_approval_id: u64,                         // Id của approve tiếp theo
}

#[near_bindgen]
impl NFTContract {
    // Tạo 1 Drop Sale mới thuộc 1 Collection và 1 mảng các Template nào đó
    /**
     * - Yêu cầu user nạp tiền để cover phí lưu trữ
     * - Thêm DropSale vào drops_by_id
     * - Refund lại NEAR user deposit thừa
     */
    #[payable]
    pub fn create_drop(
        &mut self,
        collection_name: CollectionName,
        template_ids: Vec<TemplateId>,
        price: f32,
        price_type: String,
        is_public: bool,
        max_supply: u32,
        account_limit: u32,
        account_limit_cooldown: String,
        start_time: String,
        end_time: String,
        display_data: Option<String>,
    ) -> DropSale {
        let before_storage_usage = env::storage_usage(); // Dùng để tính toán lượng near thừa khi deposit

        let account_id = env::predecessor_account_id();
        let drop_id = self.drops_by_id.len() as u32;

        // Check drop_id đã tồn tại chưa
        assert!(
            self.drops_by_id.get(&drop_id).is_none(),
            "Drop id already exists"
        );

        // Check từng template_id trong template_ids có tồn tại không

        let collection = self
            .collections_by_name
            .get(&collection_name)
            .expect("Collection does not exist");

        // Check owner
        assert_eq!(
            account_id, collection.owner_id,
            "Only owner of this collection can create a Sale Drop"
        );

        let new_drop = DropSale {
            drop_id,
            owner_id: account_id,
            collection_name: collection_name.clone(),
            template_ids: template_ids.clone(),
            price: price.clone(),
            price_type: price_type.clone(),
            is_public: is_public.clone(),
            max_supply: max_supply.clone(),
            account_limit: account_limit.clone(),
            account_limit_cooldown: account_limit_cooldown.clone(),
            start_time: start_time.clone(),
            end_time: end_time.clone(),
            display_data: display_data.clone(),
            approved_account_ids: HashMap::default(),
            next_approval_id: 0,
        };

        // Insert new created drop into drops_by_id
        self.drops_by_id.insert(&drop_id, &new_drop);

        // Luợng data storage sử dụng = after_storage_usage - before_storage_usage
        let after_storage_usage = env::storage_usage();
        // Refund NEAR
        refund_deposit(after_storage_usage - before_storage_usage);

        new_drop
    }

    // Add 1 account to Drop Sale's approved_account_ids -> They can purchase the Drop Sale
    // Only the owner of the Collection can add
    // Only applied for non-public Drop Sale
    #[payable]
    pub fn drop_add_whitelist_account(&mut self, drop_id: DropId, account_id: AccountId) {
        assert_at_least_one_yocto();

        let mut drop = self
            .drops_by_id
            .get(&drop_id)
            .expect("Drop id does not exist");

        // Check if the Drop Sale is public or not?
        // If the Drop is public -> Don't need to add user to whitelist anymore
        assert_eq!(drop.is_public, false, "Drop Sale is already public");

        // Only owner can add an account to whitelist for Drop Sale
        assert_eq!(
            &env::predecessor_account_id(),
            &drop.owner_id,
            "Only owner can add an account to whitelist for this Drop Sale"
        );

        // Cannot add the owner to the approval_account_ids
        assert_ne!(
            &account_id, &drop.owner_id,
            "Cannot add the owner his self to the approval list"
        );

        assert!(
            drop.approved_account_ids.get(&account_id).is_none(),
            "Account already approved for purchase this Drop Sale"
        );

        let approval_id = drop.next_approval_id;
        // Check whether this account has been approved or not
        // Add the account to approved_account_ids list
        let is_new_approval = drop
            .approved_account_ids
            .insert(account_id.clone(), approval_id)
            .is_none();

        // If add a new account to whitelist -> Increase the storage data -> User should pay
        let storage_used = if is_new_approval {
            bytes_for_approved_account_id(&account_id)
        } else {
            0
        };

        drop.next_approval_id += 1;
        self.drops_by_id.insert(&drop_id, &drop);

        // Refund if user deposit more NEAR than needed
        refund_deposit(storage_used);
    }

    // Kiểm tra account có tồn tại trong list approve để mua Drop Sale ko
    pub fn drop_is_approved(
        &self,
        drop_id: DropId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool {
        let drop = self.drops_by_id.get(&drop_id).expect("Drop sale not found");

        // If the Drop is public -> Return true for any account
        if drop.is_public == true {
            return true;
        }

        let approval = drop.approved_account_ids.get(&approved_account_id);

        // Nếu tồn tại account trong list approved_account_ids -> Check tiếp xem approval_id có đúng ko
        if let Some(approval) = approval {
            if approval == &approval_id.unwrap() {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    // Remove approve of an account from buying the Drop Sale
    // Only applied for non-public Drop Sale
    // Note: Khi xoá 1 account khỏi approved_list_ids -> Refund phí lưu trữ data mà user đã trả trước đó
    #[payable]
    pub fn drop_revoke(&mut self, drop_id: DropId, account_id: AccountId) {
        assert_one_yocto();

        let mut drop = self.drops_by_id.get(&drop_id).expect("Not found Drop");
        let sender_id = env::predecessor_account_id();
        // Check xem người gọi hàm revoke() có phải owner của token hay không
        assert_eq!(
            &sender_id, &drop.owner_id,
            "Only owner of the Drop Sale can call revoke function"
        );

        // If the Drop Sale is public -> cannot revoke approval
        assert!(
            drop.is_public == false,
            "The Drop Sale is public! Cannot revoke approval"
        );

        // Nếu xoá quyền thành công
        if drop.approved_account_ids.remove(&account_id).is_some() {
            // Refund lại số tiền đã deposit để lưu trữ data của user
            refund_approved_account_ids_iter(sender_id, [account_id].iter());
            // Cập nhật lại danh sách drops
            self.drops_by_id.insert(&drop_id, &drop);
        }
    }

    // Remove approve of all accounts from buying the Drop Sale
    // Only applied for non-public Drop Sale
    #[payable]
    pub fn drop_revoke_all(&mut self, drop_id: DropId) {
        assert_one_yocto();

        let mut drop = self.drops_by_id.get(&drop_id).expect("Not found Drop Sale");

        // If the Drop Sale is public -> cannot revoke approval
        assert!(
            drop.is_public == false,
            "The Drop Sale is public! Cannot revoke approval"
        );

        let sender_id = env::predecessor_account_id();
        // Check xem người gọi hàm revoke() có phải owner của token hay không
        assert_eq!(
            &sender_id, &drop.owner_id,
            "Only owner of the Drop Sale can call revoke function"
        );

        if !drop.approved_account_ids.is_empty() {
            // Refund lại số tiền mọi người đã deposit khi gọi hàm revoke_all()
            refund_approved_account_ids(sender_id, &drop.approved_account_ids);
            // Xoá toàn bộ list account đã approved cho token
            drop.approved_account_ids.clear();
            // Cập nhật lại danh sách drops
            self.drops_by_id.insert(&drop_id, &drop);
        }
    }

    // -------------------------------- Enumerations --------------------------------
    // Lấy tổng số Drop Sale đang có trong contract
    pub fn drop_total_supply(&self) -> U128 {
        // Đếm tổng số lượng id đang có trong token_metadata_by_id
        U128(self.drops_by_id.len() as u128)
    }

    // Lấy tổng số Drop Sale đang có của Template nào đó
    pub fn drop_supply_by_collection(&self, collection_name: CollectionName) -> U128 {
        // Check collection id có tồn tại không
        assert!(
            self.collections_by_name.get(&collection_name).is_some(),
            "Collection does not exist"
        );

        let mut count = 0;

        let drops_set_by_collection: Vec<DropSale> = self
            .drops_by_id
            .keys()
            .map(|drop_id| self.drops_by_id.get(&drop_id).unwrap())
            .collect();

        for drop in drops_set_by_collection {
            if drop.collection_name == collection_name {
                count += 1;
            }
        }

        U128(count)
    }

    // Lấy danh sách tất cả Drop Sale trong Contract
    pub fn get_all_drops(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<DropSale> {
        let start = u128::from(from_index.unwrap_or(U128(0)));

        // Duyệt tất cả các keys -> Trả về Template
        self.drops_by_id
            .iter()
            .skip(start as usize)
            .take(limit.unwrap() as usize)
            .map(|(drop_id, _drop)| self.drops_by_id.get(&drop_id).unwrap())
            .collect()
    }

    // Lấy danh sách Drop Sale của Collection nào đó (có pagination)
    pub fn get_all_drops_by_collection(
        &self,
        collection_name: CollectionName,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<DropSale> {
        // Check collection id có tồn tại không
        assert!(
            self.collections_by_name.get(&collection_name).is_some(),
            "Collection does not exist"
        );

        let start = u128::from(from_index.unwrap_or(U128(0)));

        let mut result = Vec::<DropSale>::new();

        // Duyệt tất cả các keys -> Trả về Collection
        let drops_set_for_owner: Vec<DropSale> = self
            .drops_by_id
            .keys()
            .skip(start as usize) // Pagination
            .take(limit.unwrap_or(0) as usize) // Pagination
            .map(|drop_id| self.drops_by_id.get(&drop_id).unwrap())
            .collect();

        for drop in drops_set_for_owner {
            if drop.collection_name == collection_name {
                result.push(drop);
            }
        }
        result
    }
    // Lấy Drop Sale theo id
    pub fn get_drop_by_id(&self, drop_id: DropId) -> DropSale {
        self.drops_by_id.get(&drop_id).expect("Drop does not exist")
    }
}
