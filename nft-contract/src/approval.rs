/**
 * Manage account approved through functions
 */
use crate::*;

const GAS_FOR_NFT_APPROVE: Gas = 10_000_000_000_000;
const NO_DEPOSIT: Balance = 0;

pub trait NonFungibleTokenApproval {
    // Let other accounts (Marketplace) to transfer token to another
    fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>);
    // Let other accounts (Marketplace) to transfer token of 1 Collection to another
    fn nft_approve_for_collection(
        &mut self,
        collection_name: CollectionName,
        account_id: AccountId,
        msg: Option<String>,
    );
    // Check if the account has the approval to transfer Token or not
    // If approve account_id is valid -> return true, else return false
    fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool;
    // Delete approval of 1 account from transfering this token
    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId);
    // Delete approval of all accounts from transfering this token
    fn nft_revoke_all(&mut self, token_id: TokenId);
}

#[ext_contract(ext_non_fungible_token_approval_receiver)]
pub trait NonFungibleTokenApprovalReceiver {
    // Market Contract: A
    // NFT Contract: B
    // When Contract A call approve on Contract B -> B will call back function
    // nft_on_approve on Contract A -> A do some actions (sale, update information, ...)
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        token_by_template_id: TokenId, // Number of NFT belongs to the Template
        collection_id: CollectionId,   // Id of the Collection that the NFT belongs to
        collection_name: CollectionName, // Name of the Collection that the NFT belongs to
        schema_id: SchemaId,           // Id of the Schema that the NFT belongs to
        schema_name: SchemaName,       // Name of the Schema that the NFT belongs to
        template_id: TemplateId,       // Name of the Template that the NFT belongs to
        msg: String,
    );

    fn nft_on_approve_for_collection(
        &mut self,
        collection_name: CollectionName,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    );
}

#[near_bindgen]
impl NonFungibleTokenApproval for NFTContract {
    // Add Token transfer approval to this account_id
    // Add account_id to list approved_account_ids of the Token
    // Note: Because this function will increase the data inside the Contract -> Add payable so the user have to deposit to cover storage
    // Account ID => market contract id
    #[payable]
    fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>) {
        assert_at_least_one_yocto();

        // Check if the token has exists or not
        let mut token = self.tokens_by_id.get(&token_id).expect("Not found token");

        // check if the sender id token's owner or not
        // only owner can add approval to other account
        assert_eq!(
            &env::predecessor_account_id(),
            &token.owner_id,
            "Predecessor must be the token owner"
        );

        // Approving
        let approval_id = token.next_approval_id;
        // Check if this account has existed inside the list approved_account_ids or not
        // Add account into the list which can transfer this Token
        let is_new_approval = token
            .approved_account_ids
            .insert(account_id.clone(), approval_id)
            .is_none();

        // If approve for new account -> Increase the data storage -> User have to pay to cover storage
        let storage_used = if is_new_approval {
            bytes_for_approved_account_id(&account_id)
        } else {
            0
        };

        token.next_approval_id += 1;
        self.tokens_by_id.insert(&token_id, &token);

        // Refund if user deposit more than needed
        refund_deposit(storage_used);

        // If attached msg -> Doing Cross Contract Call to Market Contract
        // msg include: price, action, function, ...
        if let Some(msg) = msg {
            ext_non_fungible_token_approval_receiver::nft_on_approve(
                token_id,
                token.owner_id,
                approval_id,
                token.token_by_template_id.clone(),
                token.collection_id,
                token.collection_name,
                token.schema_id,
                token.schema_name,
                token.template_id,
                msg,
                &account_id,
                NO_DEPOSIT,
                env::prepaid_gas() - GAS_FOR_NFT_APPROVE,
            )
            .as_return();
        }
    }

    // Add Token transfer approval to this account_id
    // Add account_id to list approved_account_ids of the Token
    // Note: Because this function will increase the data inside the Contract -> Add payable so the user have to deposit to cover storage
    // Account ID => market contract id
    #[payable]
    fn nft_approve_for_collection(
        &mut self,
        collection_name: CollectionName,
        account_id: AccountId,
        msg: Option<String>,
    ) {
        assert_at_least_one_yocto();

        // Check if the token has exists or not
        let mut collection = self
            .collections_by_name
            .get(&collection_name)
            .expect("Not found collection");

        // check if the sender id token's owner or not
        // only owner can add approval to other account
        assert_eq!(
            &env::predecessor_account_id(),
            &collection.owner_id,
            "Predecessor must be the Collection owner"
        );

        // Approving
        let approval_id = collection.next_approval_id;
        // Check if this account has existed inside the list approved_account_ids or not
        // Add account into the list which can transfer this Token
        let is_new_approval = collection
            .approved_account_ids
            .insert(account_id.clone(), approval_id)
            .is_none();

        // If approve for new account -> Increase the data storage -> User have to pay to cover storage
        let storage_used = if is_new_approval {
            bytes_for_approved_account_id(&account_id)
        } else {
            0
        };

        collection.next_approval_id += 1;
        self.collections_by_name
            .insert(&collection_name, &collection);
        self.collections_by_id
            .insert(&collection.collection_id, &collection);

        // Refund if user deposit more than needed
        refund_deposit(storage_used);

        // If attached msg -> Doing Cross Contract Call to Market Contract
        // msg include: price, action, function, ...
        if let Some(msg) = msg {
            ext_non_fungible_token_approval_receiver::nft_on_approve_for_collection(
                collection.collection_name,
                collection.owner_id,
                approval_id,
                msg,
                &account_id,
                NO_DEPOSIT,
                env::prepaid_gas() - GAS_FOR_NFT_APPROVE,
            )
            .as_return();
        }
    }

    // Check if this account has existed inside the list approved_account_ids or not
    // Check if this account is able to transfer the NFT or not
    fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool {
        let token = self.tokens_by_id.get(&token_id).expect("Token not found");
        let approval = token.approved_account_ids.get(&approved_account_id);

        // If has existed inside the list approved_account_ids -> Check if approval_id is valid or not
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

    // Note: When deleting 1 account from approved_list_ids -> Refund storage data fee that the user has deposited
    #[payable]
    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId) {
        assert_one_yocto();

        let mut token = self.tokens_by_id.get(&token_id).expect("Not found token");
        let sender_id = env::predecessor_account_id();
        // Check if the person who call revoke() is the owner of this NFT or not
        assert_eq!(
            &sender_id, &token.owner_id,
            "Only owner of the NFT can call revoke function"
        );

        // If revoke success
        if token.approved_account_ids.remove(&account_id).is_some() {
            // Refund the deposited amount to cover storage of the user before
            refund_approved_account_ids_iter(sender_id, [account_id].iter());
            // Update list tokens
            self.tokens_by_id.insert(&token_id, &token);
        }
    }

    #[payable]
    fn nft_revoke_all(&mut self, token_id: TokenId) {
        assert_one_yocto();

        let mut token = self.tokens_by_id.get(&token_id).expect("Not found token");
        let sender_id = env::predecessor_account_id();
        // Check if the person who call revoke() is the owner of this NFT or not
        assert_eq!(
            &sender_id, &token.owner_id,
            "Only owner of the NFT can call revoke function"
        );

        if !token.approved_account_ids.is_empty() {
            // Refund the deposited amount of everyone when calling revoke_all()
            refund_approved_account_ids(sender_id, &token.approved_account_ids);
            // Delete all approved account of this token
            token.approved_account_ids.clear();
            // Update list tokens
            self.tokens_by_id.insert(&token_id, &token);
        }
    }
}
