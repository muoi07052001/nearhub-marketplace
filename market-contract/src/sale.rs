use crate::*;
#[ext_contract(ext_nft_contract)]
pub trait NFTContract {
    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: u64,
        memo: String,
        balance: U128,
        max_len_payout: u32,
    ) -> Payout;
}

#[ext_contract(ext_self)]
pub trait MarketContract {
    fn resolve_purchase(&mut self, buyer_id: AccountId, price: U128) -> Promise;
}

#[near_bindgen]
impl MarketContract {
    // Xoá sale
    #[payable]
    pub fn remove_sale(&mut self, nft_contract_id: AccountId, token_id: TokenId) {
        assert_one_yocto();

        // Xoá sale
        let sale = self.internal_remove_sale(nft_contract_id, token_id);

        assert_eq!(
            env::predecessor_account_id(),
            sale.owner_id,
            "Must be owner id"
        );
    }

    // Update giá của Sale
    #[payable]
    pub fn update_price(&mut self, nft_contract_id: AccountId, token_id: TokenId, price: U128) {
        assert_one_yocto();

        let contract_and_token_id =
            format!("{}{}{}", nft_contract_id.clone(), ".", token_id.clone());

        let mut sale = self
            .sales
            .get(&contract_and_token_id)
            .expect("Not found sale");

        // Check xem có phải người update giá là chủ của Sale ko
        assert_eq!(
            env::predecessor_account_id(),
            sale.owner_id,
            "Must be sale owner"
        );

        sale.sale_conditions = price;

        // Update lại thông tin
        self.sales.insert(&contract_and_token_id, &sale);
    }

    // Cho phép user mua nft
    #[payable]
    pub fn offer(&mut self, nft_contract_id: AccountId, token_id: TokenId) {
        let deposit = env::attached_deposit();
        assert!(deposit > 0, "Attached deposit must be greater than 0");

        let contract_and_token_id =
            format!("{}{}{}", nft_contract_id.clone(), ".", token_id.clone());

        let sale = self
            .sales
            .get(&contract_and_token_id)
            .expect("Not found sale");

        let buyer_id = env::predecessor_account_id();
        // Buyer và owner của NFT phải khác nhau (không thể tự mua NFT của chính mình được)
        assert_ne!(buyer_id, sale.owner_id, "Can not bid on your own sale");

        let price = sale.sale_conditions.0;
        assert!(
            deposit >= price,
            "Attached deposit must be greater than or equal current price: {}",
            price
        );

        self.process_purchase(
            nft_contract_id,
            token_id,
            U128(deposit),
            buyer_id,
            sale.owner_id,
        );
    }

    // Transfer money to the owner of the NFT
    #[private]
    pub fn process_purchase(
        &mut self,
        nft_contract_id: AccountId,
        token_id: TokenId,
        price: U128,
        _buyer_id: AccountId,
        receiver_id: AccountId,
    ) -> U128 {
        // Mua hàng -> Xoá sản phẩm đi
        self.internal_remove_sale(nft_contract_id.clone(), token_id.clone());
        // Transfer money to the owner of the NFT
        Promise::new(receiver_id).transfer(u128::from(price));

        price
    }

}
