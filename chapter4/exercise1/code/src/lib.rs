use scrypto::prelude::*;

// Step 1: Define the Struct that will be used to store the
// members non fungible metadata. Name the struct `MemberData` and add a Decimal field
// named `amount_staked` that is **mutable**.
#[derive(NonFungibleData)]
struct MemberData {
    #[mutable]
    amount_staked: Decimal
}

#[blueprint]
mod exercise_module {
    struct Exercise1 {
        // Step 4: Define the variables that the instantiated component
        // will have access to. We need a vault to store the staked XRD,
        // a Vault to store the member_manager badge and a ResourceAddress to store the address
        // of the member badges
        xrd_vault: Vault,
        manager_badge: Vault,
        member_resource_address: ResourceAddress
    }

    impl Exercise1 {
        pub fn instantiate_exercise() -> ComponentAddress {
            // Step 2: Create a new fungible resource with a divisibility of 0 and an initial supply of 1.
            // This will be the badge that is allowed to mint member badges. Store the token in a `member_manager_badge` Bucket variable.
            let member_manager_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1);

            // Step 3: Create a non-fungible resource with IDs of the UUID type.
            // This resource is `mintable` and `updateable_non_fungible_data` by someone showing ownership of
            // the `member_manager_badge`. Initialize the resource with no initial supply and store the returned
            // ResourceAddress in a `member_badge` variable.
            let member_badge = ResourceBuilder::new_uuid_non_fungible()
                .metadata("name", "Member Badge")
                .mintable(rule!(require(member_manager_badge.resource_address())), rule!(deny_all))
                .updateable_non_fungible_data(rule!(require(member_manager_badge.resource_address())), rule!(deny_all))
                .create_with_no_initial_supply();

            Self {
                // Step 5: Assign a value for the three variables we defined in the struct.
                xrd_vault: Vault::new(RADIX_TOKEN),
                manager_badge: Vault::with_bucket(member_manager_badge),
                member_resource_address: member_badge
            }
            .instantiate()
            .globalize()
        }

        // Step 6: Create a `become_member` method that returns a Bucket.
        // Inside, instantiate a new MemberData struct with a value of 0 for the amount_staked.
        // Then put a proof of the `manager_badge` on the AuthZone (using the .authorize() method),
        // mint a new member non fungible resource with a random local ID and return it.
        // Remember that because the ID type of the resource is UUID, you will have to call `mint_uuid_non_fungible(data)`
        // instead of `mint_non_fungible(id, data)`
        pub fn become_member(&self) -> Bucket {
            let data = MemberData {
                amount_staked: Decimal::zero()
            };

            self.manager_badge.authorize(|| {
                borrow_resource_manager!(self.member_resource_address)
                    .mint_uuid_non_fungible(data)
            })
        }

        // Step 7: Complete the `stake_xrd` method. Start by adding a Bucket parameter named `xrd`
        // and a Proof parameter named `member_proof`.
        pub fn stake_xrd(&mut self, xrd: Bucket, member_proof: Proof) {
            
            // Step 8: Validate the passed proof to make sure its resource address is the same as the one stored on the component's state.
            // Store the ValidatedProof in a variable.
            let member_proof = member_proof
                .validate_proof(self.member_resource_address)
                .expect("Wrong proof provided!");


            // Step 9: Store the amount of passed XRD in an `amount` variable
            // and put the xrd bucket inside the component's `xrd_vault`.
            let amount = xrd.amount();
            self.xrd_vault.put(xrd);


            // Step 10: Get the local ID and data of the passed-in proof
            let non_fungible: NonFungible<MemberData> = member_proof.non_fungible();
            let mut member_data = non_fungible.data();

            // Step 11: Add the amount of new XRD token staked to the `amount_staked`
            // field of the NFT metadata and update the data on the ledger (by calling the ResourceManager::update_non_fungible_data method).
            // Do not forget to put the member_manager_badge on the authzone with `.authorize()`.
            // to be able to update the data on the ledger.
            member_data.amount_staked += amount;
            self.manager_badge.authorize(|| {
                borrow_resource_manager!(self.member_resource_address)
                    .update_non_fungible_data(&non_fungible.local_id(), member_data);
            });
        }

        // Step 12: Complete the `withdraw` method. Start by adding a Proof parameter named `member_proof`
        // just like with the `stake_xrd` method
        pub fn withdraw(&mut self, member_proof: Proof) -> Bucket {
            // Step 13: validate the proof and store the ValidatedProof in a variable
            let member_proof = member_proof.validate_proof(self.member_resource_address).expect("Wrong proof provided!");
            
            // Step 14: Use the ValidatedProof to get the local ID of the member and the data attached to
            // its NFT
            let non_fungible: NonFungible<MemberData> = member_proof.non_fungible();
            let mut member_data = non_fungible.data();

            // Step 15: Store the value of the `amount_staked` field of the NFT's data in a variable
            let amount_to_withdraw = member_data.amount_staked;

            // Step 16: Set the NFT's data `amount_staked` value to 0 and save the changes to the ledger.
            member_data.amount_staked = Decimal::zero();
            self.manager_badge.authorize(|| {
                borrow_resource_manager!(self.member_resource_address)
                    .update_non_fungible_data(&non_fungible.local_id(), member_data);
            });

            // Step 17: Take the amount of tokens that was staked from the `xrd_vault` and return it.
            self.xrd_vault.take(amount_to_withdraw)
        }
    }
}