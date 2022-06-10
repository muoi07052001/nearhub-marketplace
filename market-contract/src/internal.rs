use crate::*;

#[near_bindgen]
impl MarketContract {
    // Xoá sale của 1 NFT
    pub(crate) fn internal_remove_sale(
        &mut self,
        nft_contract_id: AccountId,
        token_id: TokenId,
    ) -> Sale {
        let contract_and_token_id = format!("{}{}{}", nft_contract_id, ".", token_id);

        let sale = self
            .sales
            .remove(&contract_and_token_id)
            .expect("Not found sale");

        // Xoá sale khỏi tokens_by_owner_id
        let mut tokens_by_owner_id = self
            .tokens_by_owner_id
            .get(&sale.owner_id)
            .expect("Not found sale by owner_id");
        tokens_by_owner_id.remove(&contract_and_token_id);

        // Nếu xoá sale đi mà user không còn sale nào nx -> Xoá luôn account đi
        if tokens_by_owner_id.is_empty() {
            self.tokens_by_owner_id.remove(&sale.owner_id);
        } else {
            self.tokens_by_owner_id
                .insert(&sale.owner_id, &tokens_by_owner_id);
        }

        let mut tokens_by_contract_id = self
            .tokens_by_contract_id
            .get(&nft_contract_id)
            .expect("Not found sale by contract_id");
        tokens_by_contract_id.remove(&token_id);
        if tokens_by_contract_id.is_empty() {
            self.tokens_by_contract_id.remove(&nft_contract_id);
        } else {
            self.tokens_by_contract_id
                .insert(&nft_contract_id, &tokens_by_contract_id);
        }

        sale
    }
}
