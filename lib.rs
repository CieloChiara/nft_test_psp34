#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod test_psp34 {
    use ink_prelude::{
        vec::Vec,
        string::{
            String,
            ToString,
        },
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp34::extensions::{
            metadata::*,
            mintable::*,
        },
        traits::Storage,
    };

    #[derive(SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct TestPsp34 {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: metadata::Data,
        initial_id:Id,
    }

    impl PSP34 for TestPsp34 {}
    impl PSP34Metadata for TestPsp34 {}
    impl PSP34Mintable for TestPsp34 {}

    impl TestPsp34 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(id: Id, name: String, symbol: String, base_uri: String) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                // let name_key: Vec<u8> = String::from("name");
                // let symbol_key: Vec<u8> = String::from("symbol");
                let name_key: Vec<u8> = "name".as_bytes().to_vec();
                let symbol_key: Vec<u8> = "symbol".as_bytes().to_vec();
                let base_uri_key: Vec<u8> = "base_uri".as_bytes().to_vec();
                instance._set_attribute(id.clone(), name_key, name.as_bytes().to_vec());
                instance._set_attribute(id.clone(), symbol_key, symbol.as_bytes().to_vec());
                instance._set_attribute(id.clone(), base_uri_key, base_uri.as_bytes().to_vec());
                instance.initial_id = id;
            })
        }

        #[ink(message)]
        #[ink(payable)]
        pub fn mint_for_sale(&mut self, account:AccountId, id: Id) -> Result<(), PSP34Error> {
            let transfered_value = self.env().transferred_value();
            ink_env::debug_println!("     ########## tranfered_value: {:?}", transfered_value);
            if transfered_value < 1000000000000000000 {
                return Err(PSP34Error::Custom("You don't pay enough.".to_string()));
            }
            self._mint_to(account, id)
        }

        #[ink(message)]
        pub fn token_uri(&self, id: Id) -> String {
            let base_uri_key: Vec<u8> = "base_uri".as_bytes().to_vec();
            let base_uri = match self.get_attribute(self.initial_id.clone(), base_uri_key) {
                Some(value) => value,
                None => return "".to_string(),
            };
            //let extention: &str = ".json".to_string();
            String::from_utf8(base_uri.clone()).unwrap() + &self._get_id_string(id) + ".json"
        }

        #[inline]
        fn _get_id_string(&self, id:Id) -> String {
            match id {
                Id::U8(u8) => {
                    let tmp:u8 = u8;
                    tmp.to_string()
                },
                _ => "test".to_string(),

            }
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let test_psp34 = TestPsp34::default();
            assert_eq!(test_psp34.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut test_psp34 = TestPsp34::new(false);
            assert_eq!(test_psp34.get(), false);
            test_psp34.flip();
            assert_eq!(test_psp34.get(), true);
        }
    }
}