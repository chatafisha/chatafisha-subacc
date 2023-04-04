# chatafisha-subacc
This contract when deployed on near blockchain can create subaccounts of the master account and self delete.The smart contract exposes two methods to enable creating subaccounts and self delete of the created accounts in the NEAR network.

```rust
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


```

<br />

# Quickstart

1. Make sure you have installed [rust](https://rust.org/).
2. Install the [`NEAR CLI`](https://github.com/near/near-cli#setup)
3. Make sure you also have git installed

<br />
To clone the chata-subacc repo simply run the command:
```bash
git clone
```


## 1. Build and Deploy the Contract
You can automatically compile and deploy the contract in the NEAR testnet by running:

```bash
cd chata-subacc/contract
./deploy.sh
```

Once finished, check the `neardev/dev-account` file to find the address in which the contract was deployed:

```bash
cat ./neardev/dev-account
# e.g. dev-1659899566943-21539992274727
```
To deploy the contract on testnet or mainnet, you have to chose the default network by prepending the env variable to the command
```bash
NEAR_ENV=mainnet
# changing network to mainnet, the network is testnet by default
```

To create a new near account navigate to [https://wallet.testnet.near.org](testnet) or [https://wallet.near.org])(mainnet) and click on "Create Account"
To login or create a new account with testnet or mainnet, simply run the command and allow full access to the near dapps to connect to your wallet
```bash
near login

```
and then use the logged account to sign the transaction: `--accountId <your-account>`.
<br />

## 2. Creating subaccounts

`create` is a method which changes the state of the blockchain by creating new sub-accounts of the **current accountId**(The account that initially deployed this contract).

To create a new subaccount parameters `prefix` and `public_key` have to be passed.**prefix** is simply the string name of the subaccount to be generated under the main account(eg rose.chatafisha.near), where `rose` is `prefix` and subaccount generated while `chatafisha` is the main accountId. Luckily enough we get to chose the `public_key` of type `ed25519` we wish to insert(example `"ed25519:9sfQQ21YYh65mMwZs3ePcHXyV9DBjS98VFjfve5po4wB"`)

```bash
near call <deployed-to-account> create '{"prefix":"<new-subaacc-name>", "public_key":"ed25519:<data>"}' --accountId <predecessor-account>
```
If the transaction succeeds a new subaccount is created and the transactions can be viewed on the near explorer(example `https://explorer.testnet.near.org/transactions/4inN3rsj4V1FS25FSxca1iczyoK2ZfH2B6cccbDydJ1X` for near testnet)

<br />

## 3. Sending funds to the newly created subaccounts
To send fund to the newly created sub accounts, simply run the command

```bash
near send <sender> <receiver> amount
```
where sender is the accountId of the sender(ie sender.testnet), while receiver is the accountId of the receiver(receiver.testnet) and amount is the numerical value of the amount of `near` tokens to send

<br />

## 4. Account self-delete
The newly created subaccounts will also have the ability to self delete, to self delete simply run the command
```bash
near call <deployed-to-account> delete '{"prefix":"subaccount-name to be deleted","beneficiary":"account that will inherit funds"}'
```
