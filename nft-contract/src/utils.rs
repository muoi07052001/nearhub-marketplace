use crate::*;

pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    // get the default hash algorithm
    let mut hash = CryptoHash::default();
    // We hash the account Id and return it as
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

pub(crate) fn assert_one_yocto() {
    assert_eq!(
        env::attached_deposit(),
        1,
        "Required attached deposit of exact 1 yoctoNear"
    );
}

pub(crate) fn assert_at_least_one_yocto() {
    assert!(
        env::attached_deposit() >= 1,
        "Required attached deposit of at least 1 yoctoNear"
    );
}
