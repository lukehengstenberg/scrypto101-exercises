use scrypto::prelude::*;

#[blueprint]
mod token_sale_module {
    struct TokenSale {
        luke_tokens_vault: Vault,
        collected_xrd: Vault,
        price: Decimal,
    }

    impl TokenSale {
        // given a price in XRD, creates a ready-to-use Token Sale
        pub fn instantiate_token_sale(price: Decimal) -> ComponentAddress {
            // create a new luke token resource, with a fixed quantity of 100
            let bucket_of_luke_tokens: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Luke Tokens")
                .metadata("symbol", "LUKE")
                .metadata("description", "This is a useful token created by Luke!")
                .mint_initial_supply(100);

            // populate a TokenSale struct and instantiate a new component
            Self {
                luke_tokens_vault: Vault::with_bucket(bucket_of_luke_tokens),
                collected_xrd: Vault::new(RADIX_TOKEN),
                price: price,
            }
            .instantiate()
            .globalize()
        }

        pub fn get_price(&self) -> Decimal {
            self.price
        }

        pub fn buy_luke_token(&mut self, mut payment: Bucket) -> (Bucket, Bucket) {
            // take our price in XRD out of the payment. If the caller has sent too 
            // few, or sent something other than XRD, they'll get a runtime error
            self.collected_xrd.put(payment.take(self.price));

            // return a tuple containing a luke token, plus whatever change is
            // left on the input payment (if any). If we're out of luke tokens to
            // give, we'll see a runtime error when we try to grab one
            (self.luke_tokens_vault.take(1), payment)
        }
    }
}
