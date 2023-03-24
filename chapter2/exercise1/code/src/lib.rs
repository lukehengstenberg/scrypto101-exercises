use scrypto::prelude::*;

// Step 4: Create a `Vegetable` enum with the following fields:
//   - Tomato
//   - Carrot
//   - Broccoli
// Don't forget to add a call to the `derive` macro with ScryptoCategorize, ScryptoEncode, ScryptoDecode, LegacyDescribe and Debug.
// This last one, Debug, is neccessary as we are going to display it's value.
#[derive(ScryptoCategorize, ScryptoEncode, ScryptoDecode, LegacyDescribe, Debug)]
enum Vegetable {
    Tomato,
    Carrot,
    Broccoli
}

#[blueprint]
mod exercise_module {
    struct Exercise1 {
        // Step 1: Add two variables here. 
        //   - One u64 named `instantiated_at`. This will contain the epoch at which the component was instantiated
        //   - One String named `instantiator_name`
        instantiated_at: u64,
        instantiator_name: String,

        // Step 5: Add a variable of type Vegetable named `favorite_vegetable`
        favourite_vegetable: Vegetable
    }

    impl Exercise1 {

        // Step 2: add a String argument named `instantiator_name` to the following function
        pub fn instantiate_exercise(instantiator_name: String) -> ComponentAddress {
            Self {
                // Step 3: Assign a value to the `instantiated_at` and `instantiator_name` variables.
                // Help: You can get the current epoch with `Runtime::current_epoch()`
                instantiated_at: Runtime::current_epoch(),
                instantiator_name: instantiator_name,
                // Step 6: Assign a value to the `favorite_vegetable` variable
                favourite_vegetable: Vegetable::Broccoli
            }
            .instantiate()
            .globalize()
        }

        // Step 7: Create a `log_data` method that does the following:
        //   - Display the instantiator_name and instantiated_at variables using the `error!` macro
        //   - Display the favorite vegetable using the `debug!` macro
        // Help: You can display formatted messages using the following notation: 
        //   "blah {} blah" -> for instantiator_name and instantiated_at
        //   "blah {:?} blah" -> for the favorite_vegeratable. This is different notation than the other variables because
        //     the enum has a Debug implementation but not a Display one
        pub fn log_data(&self) {
            error!("Instantiated by {} at epoch {}", self.instantiator_name, self.instantiated_at);
            debug!("The favourite vegetable is: {:?}", self.favourite_vegetable);
        }
    }
}
