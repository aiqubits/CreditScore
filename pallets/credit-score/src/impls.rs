use frame_support::pallet_macros::pallet_section;
/// Define the implementation of the pallet, like helper functions.
#[pallet_section]
mod impls {
    impl<T: Config> Pallet<T> {
        pub fn random_value(who: &T::AccountId) -> [u8; 16] {
            let nonce = frame_system::Pallet::<T>::account_nonce(&who);
            // // let nonce_u32: u32 = nonce as u32;
            // // generate a random value based on account and its nonce
            // let nonce_u32: u32 = TryInto::try_into(nonce).ok().expect("nonce is u64; qed");
            // let a: BlockNumberFor<T> = TryFrom::try_from(nonce_u32)
            //     .ok()
            //     .expect("nonce is u32; qed");
            // // payload.using_encoded(blake2_128)
            // let payload = (
            //     T::Randomness::random_seed(),
            //     a,
            //     <frame_system::Pallet<T>>::extrinsic_index()
            // );
            // payload.using_encoded(sp_io::hashing::blake2_128)
            nonce.using_encoded(sp_io::hashing::blake2_128)
        }

    }
}
