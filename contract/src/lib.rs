use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env;
use near_sdk::{near_bindgen, AccountId, Promise, PanicOnDefault, ext_contract, Gas};
use serde_json::json;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Swapper {
    owner_id: AccountId,
    tokenA: AccountId,
    tokenB: AccountId,
    constant: u128,
}

#[ext_contract(ext_NEP141)]
trait NEP141 {
    fn ft_transfer(receiver_id: String, amount: String, memo: Option<String>);
}

#[near_bindgen]
impl Swapper {
    #[init]
    pub fn new_swapper(
        owner_id: AccountId,
        tokenA: AccountId,
        tokenB: AccountId,
        constant: u128,
    ) -> Self {
        Self {
            owner_id,
            tokenA,
            tokenB,
            constant,
        }
    }

    fn transfer(&self, token: AccountId, receiver: String, sendY: u128) -> Promise {
        Promise::new(token)
            .function_call(
                String::from("ft_transfer"),
                json!({
                    "receiver_id": receiver,
                    "amount": sendY.to_string(),
                }).to_string().into_bytes(),
                1,
                Gas(100000000000000),
            )
    }

    fn swap(&self, sender: String, tokenX: AccountId, amount: String) -> Promise {
        let in_amount = amount.parse::<u128>().unwrap();
        if tokenX == self.tokenA {
            let sendY: u128 = in_amount * self.constant;
            return self.transfer(self.tokenB.clone(), sender, sendY);
        } else {
            let sendY: u128 = in_amount / self.constant;
            return self.transfer(self.tokenA.clone(), sender, sendY);
        }
    }

    pub fn ft_on_transfer(&self, sender_id: String, amount: String, msg: String) -> String {
        let tokenX = env::predecessor_account_id();
        if tokenX == self.tokenA || tokenX == self.tokenB {
            let sender = AccountId::new_unchecked(sender_id.clone());
            if sender != self.owner_id {
                self.swap(sender_id, tokenX, amount);
            }
            return String::from("0");
        } else {
            return amount;
        }
    }
}
