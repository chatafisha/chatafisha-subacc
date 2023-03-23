use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env, Promise, Balance, AccountId, PublicKey};
use near_sdk::collections::{LookupMap};
use near_sdk::json_types::U128;
use near_sdk::serde::{Serialize, Serializer};


#[near_bindgen]
#[derive(Default)]
#[derive( BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // accounts: LookupMap<AccountId, Balance>,
}


const MIN_STORAGE: u128 = 100_000_000_000_000_000_000_000;

// #[derive(BorshSerialize)]
// pub enum StorageKey {
//     Accounts,
// }

#[near_bindgen]
impl Contract {
    pub fn create(&self, prefix: String, public_key: PublicKey) {
        let account_id = prefix + "." + &env::current_account_id().to_string();
        Promise::new(account_id.parse().unwrap())
            .create_account()
            .transfer(MIN_STORAGE)
            .add_full_access_key(public_key);
        // Self {
        //     accounts: LookupMap::new(StorageKey::Accounts.try_to_vec().unwrap())
        // }
    }



    pub fn self_delete(prefix: String, beneficiary: AccountId) {
        let account_id = prefix + "." + &env::current_account_id().to_string();
        Promise::new(account_id.parse().unwrap()).delete_account(beneficiary);
    }

    // pub fn get_balance(&self, prefix: String) -> U128 {
    //     let account_id = prefix + "." + &env::current_account_id().to_string();
    //     self.accounts.get(&account_id.parse().unwrap()).unwrap_or(0).into()
    // }

    // pub fn send_near(account: AccountId, amount: u256) {
    //     // let account_id = prefix + "." + &env::current_account_id().to_string();
    //     Promise::new(account).transfer(amount);
    }


    // pub fn add_keys(prefix: String,public_key: PublicKey, allowance: u128, function_names: String){
    // let account = prefix + "." + &env::current_account_id().to_string(); 
    // Promise::new(account.parse().unwrap()).add_access_key(public_key, allowance,account.parse().unwrap(), function_names);
    // }

    // // pub fn delete_key( prefix: String, key: PublicKey){
    // // let account_id = prefix + "." + &env::current_account_id().to_string();
    // // Promise::new(account_id.parse().unwrap()).delete_key(key);
    // }
}

// impl Default for Contract {
//     fn default() -> Self {
//         Self {
//             accounts: LookupMap::new(b"".to_vec()), // provide a default prefix for the lookup map
//         }
//     }
// }












// impl Default for Contract {
//     fn default() -> Self {
//         Self {
//             accounts: SerializedLookupMap::new(Storage::Accounts.try_to_vec().unwrap()),
//         }
//     }
// }

