# Chatafisha-subacc Contract
This contract when deployed on near blockchain can create subaccounts of the master account and self delete.The smart contract exposes two methods to enable creating subaccounts and self delete of the created accounts in the NEAR network.
=======

# Quickstart

1. Make sure you have installed [rust](https://rust.org/).
2. Install the [`NEAR CLI`](https://github.com/near/near-cli#setup)
3. Make sure you also have git installed

<br />
To clone the chata-subacc repo simply run the command:

```bash

git clone https://github.com/chatafisha/chatafisha-subacc.git

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
