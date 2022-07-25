use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, Balance, BorshStorageKey, env, PanicOnDefault};
use near_sdk::collections::{UnorderedMap, LookupSet};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    accounts: UnorderedMap<AccountId, Balance>,
    whitelisted_tokens: LookupSet<AccountId>
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NewAccounts,
    Tokens
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self { 
            owner_id, 
            accounts: UnorderedMap::new(StorageKey::NewAccounts),
            whitelisted_tokens: LookupSet::new(StorageKey::Tokens)
        }
    }
    pub fn get_accounts(&self) -> Vec<AccountId> {
        self.assert_owner();
        let mut result:Vec<AccountId> = Vec::new();
        for (account, _balance) in self.accounts.iter() {
            result.push(account);
        }
        result
    }
    #[payable]
    pub fn deposit(&mut self) {
        // deposit-account retrieve
        let deposit = env::attached_deposit();
        let account_id = env::predecessor_account_id();

        assert!(deposit > 0, "ERR_NEGATIVE_DEPOSIT");

        self.accounts.insert(&account_id, &deposit);
    }

    fn assert_owner(&self) {
        assert!(self.owner_id.clone() == env::predecessor_account_id(), "ERR_NOT_ALLOWED")
    }
}