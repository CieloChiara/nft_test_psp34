#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod nft_psp34 {

    //let tokenId: u32 = 0;

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
    pub struct NftPsp34 {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: metadata::Data,
        initial_id:Id,
    }

    impl PSP34 for NftPsp34 {}
    impl PSP34Metadata for NftPsp34 {}
    impl PSP34Mintable for NftPsp34 {}

    impl NftPsp34 {
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
        pub fn token_uri(&self, id: u32) -> String {
            let base_uri_key: Vec<u8> = "base_uri".as_bytes().to_vec();
            let base_uri = match self.get_attribute(self.initial_id.clone(), base_uri_key) {
                Some(value) => value,
                None => return "".to_string(),
            };
            //let extention: &str = ".json".to_string();
            //String::from_utf8(base_uri.clone()).unwrap() + &self._get_id_string(id) + ".json"
            let id_str: &str = &id.to_string();
            String::from_utf8(base_uri.clone()).unwrap() + id_str + ".json"
        }

        #[inline]
        fn _get_id_string(&self, id: Id) -> String {

            match id {
                Id::U8(u8) => {
                    let tmp: u8 = u8;
                    tmp.to_string()
                }
                Id::U16(u16) => {
                    let tmp: u16 = u16;
                    tmp.to_string()
                }
                Id::U32(u32) => {
                    let tmp: u32 = u32;
                    tmp.to_string()
                }
                Id::U64(u64) => {
                    let tmp: u64 = u64;
                    tmp.to_string()
                }
                Id::U128(u128) => {
                    let tmp: u128 = u128;
                    tmp.to_string()
                }
                Id::Bytes(value) => String::from_utf8(value.clone()).unwrap(),
            }
            
        }
    }

}