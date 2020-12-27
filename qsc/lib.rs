#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod qsc {

    #[ink(storage)]
    pub struct Qsc {

        my_number_map: ink_storage::collections::HashMap<AccountId, u32>,
    }

    impl Qsc {
        /// public function.
        /// Default constructor.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                my_number_map: Default::default(),
            }
        }
        /// private function
        /// Returns the number for an Advanced or 0 if it is not set.
        fn my_number_or_zero(&self, of: &AccountId) -> u32 {
            let balance = self.my_number_map.get(of).unwrap_or(&0);
            *balance
        }
    }
    impl Qsc {
        /// public function
        #[ink(message)]
        pub fn my_public_function(&self) {
            /* --snip -- */
        }
        /// Private function
        fn my_private_function(&self) {
            /* --snif-- */
        }
        /* --snif-- */
    }
    impl Qsc {
        #[ink(message)]
        pub fn set_my_number(&mut self, value: u32) {
            let caller = self.env().caller();
            self.my_number_map.insert(caller, value);
        }

        // Add a value to the existing value for the calling
        #[ink(message)]
        pub fn add_my_number(&mut self, value: u32) {
            let caller = self.env().caller();
            let my_number = self.my_number_or_zero(&caller);
            self.my_number_map.insert(caller, my_number + value);
        }

        ///
        fn my_number_or_zero(&self, of: &AccountId) -> u32 {
            *self.my_number_map.get(of).unwrap_or(&0)
        }
    }
    
    impl<T> Value<T>
    where

        T: scale::Codec,
    {
        ///  Returns an immutable reference to the wrapped value.
        pub fn get(&self) -> &T {
            self.cell.get().unwrap()
        }
        /// Returns a mutable reference to the wrapped value.
        pub fn get_mut(&mut self) -> &mut T {
            self.cell.get_mut().unwrap()
        }
        /// Sets the wrapped value to the given value.
        pub fn set(&mut self, val: T) {
            self.cell.set(val);
        }
    }
}
