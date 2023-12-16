#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {

    use super::*;
    use frame_support::pallet_prelude::{*};
    use frame_system::pallet_prelude::*;
    use scale_info::prelude::vec::Vec;

    use frame_support::
    {
        sp_runtime::traits::AccountIdConversion,
        traits:: {
            Currency, ExistenceRequirement
        },
        PalletId,
    };

    // type BalanceOf<T> = 
    //     <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configure the module by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {

        // Event definition
        type RuntimeEvent: From<Event<Self>> 
            + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        // Currency 
        type Currency: Currency<Self::AccountId>;

        /*********Remove soon***********/
        // Randomness
        //type MyRandomness: Randomness<Self::Hash, BlockNumberFor<Self>>;

        // Ticket cost
        // #[pallet::constant]
        // type TicketCost: Get<BalanceOf<Self>>;

        // Maximum number of participants
        // #[pallet::constant]
        // type MaxParticipants: Get<u32>;

        // Module Id
        // #[pallet::constant]
        // type PalletId: Get<PalletId>;
    }


    /// Hashmap to record anchor status, Anchor => ( Owner, last block )
	#[pallet::storage]
	#[pallet::getter(fn owner)]
	pub(super) type AnchorOwner<T: Config> = StorageMap<_, Twox64Concat, Vec<u8>, (T::AccountId,BlockNumberFor<T>)>;

	/// Selling anchor status, Anchor => ( Owner, Price, Target customer )
	#[pallet::storage]
	#[pallet::getter(fn selling)]
	pub(super) type SellList<T: Config> = StorageMap<_, Twox64Concat, Vec<u8>, (T::AccountId, u32,T::AccountId)>;


    // The pallet's runtime storage items.
    // #[pallet::storage]
    // #[pallet::getter(fn get_participants)]
    // pub(super) type Participants<T: Config> = StorageValue<
    //     _,
    //     BoundedVec<T::AccountId, T::MaxParticipants>,
    //     OptionQuery
    // >;

    // #[pallet::storage]
    // #[pallet::getter(fn get_nonce)]
    // pub(super) type Nonce<T: Config> = StorageValue<
    //     _,
    //     u64,
    //     ValueQuery
    // >;

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/main-docs/build/events-errors/
    // #[pallet::event]
    // #[pallet::generate_deposit(pub(super) fn deposit_event)]
    // pub enum Event<T: Config> {
    //     /// Event emitted when a ticket is bought
    //     TicketBought { who: T::AccountId },
    //     /// Event emitted when the prize is awarded
    //     PrizeAwarded { winner: T::AccountId },
    //     /// Event emitted when the prize is to be awarded, but there are no participants
    //     ThereAreNoParticipants,
    // }

    ///Anchor event to trigger
    #[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		//An anchor is set to selling status.
		//AnchorToSell(T::AccountId,u32,T::AccountId),	//(owner, price , target)
	}

    // Errors inform users that something went wrong
    // #[pallet::error]
    // pub enum Error<T> {
    //     NotEnoughCurrency,
    //     AccountAlreadyParticipating,
    //     CanNotAddParticipant,
    // }

    //Anchor pallet error
    #[pallet::error]
	pub enum Error<T> {
		/// Anchor key length over load.
		LengthMaxLimited,

		///Anchor name max length.
		KeyMaxLimited,

		///Anchor raw data max limit.
		Base64MaxLimited,

		///Anchor protocola max length
		ProtocolMaxLimited,

		///Pre number errror
		PreAnchorFailed,

		///Anchor sell value error.
		PriceValueLimited,

		///Anchor exists already, can not be created.
		AnchorExistsAlready,

		///Anchor do not exist, can not change status.
		AnchorNotExists,

		///unknown anchor owner data in storage.
		UnexceptDataError,

		///Anchor do not belong to the account
		AnchorNotBelogToAccount,

		///Anchor is not in the sell list.
		AnchorNotInSellList,

		///Not enough balance
		InsufficientBalance,

		///Transfer error.
		TransferFailed,

		///User can not buy the anchor owned by himself
		CanNotBuyYourOwnAnchor,

		///Anchor was set to sell to target buyer
		OnlySellToTargetBuyer,
	}


    // #[pallet::call]
    // impl<T: Config> Pallet<T> {

    //     #[pallet::call_index(0)]
    //     #[pallet::weight(0)]
    //     pub fn buy_ticket(origin: OriginFor<T>) -> DispatchResult {

    //         // 1. Validates the origin signature
    //         let buyer = ensure_signed(origin)?;

    //         // 2. Checks that the user has enough balance to afford the ticket price
    //         ensure!(
    //             T::Currency::free_balance(&buyer) >= T::TicketCost::get(),
    //             Error::<T>::NotEnoughCurrency
    //         );

    //         // 3. Checks that the user is not already participating
    //         if let Some(participants) = Self::get_participants() {
    //             ensure!(
    //                 !participants.contains(&buyer),
    //                 Error::<T>::AccountAlreadyParticipating
    //             );
    //         }

    //         // 4. Adds the user as a new participant for the prize
    //         match Self::get_participants() {
    //             Some(mut participants) => { 
    //                 ensure!(
    //                     participants.try_push(buyer.clone()).is_ok(), 
    //                     Error::<T>::CanNotAddParticipant
    //                 );
    //                 Participants::<T>::set(Some(participants));
    //             }, 
    //             None => {
    //                 let mut participants = BoundedVec::new();
    //                 ensure!(
    //                     participants.try_push(buyer.clone()).is_ok(), 
    //                     Error::<T>::CanNotAddParticipant
    //                 );
    //                 Participants::<T>::set(Some(participants));
    //             }
    //         };

    //         // 5. Transfers the ticket cost to the module's account
    //         // to be hold until transferred to the winner
    //         T::Currency::transfer(
    //             &buyer, 
    //             &Self::get_pallet_account(), 
    //             T::TicketCost::get(), 
    //             ExistenceRequirement::KeepAlive)?;

    //         // 6. Notify the event
    //         Self::deposit_event(Event::TicketBought { who: buyer });
    //         Ok(())
    //     }

    //     #[pallet::call_index(1)]
    //     #[pallet::weight(0)]
    //     pub fn award_prize(origin: OriginFor<T>) -> DispatchResult {

    //         // 1. Validates the origin signature
    //         let _who = ensure_root(origin)?;

    //         match Self::get_participants() {
    //             Some(participants) => { 

    //                 // 2. Gets a random number from the randomness module
    //                 let nonce = Self::get_and_increment_nonce();
    //                 let (random_seed, _) = T::MyRandomness::random(&nonce);
    //                 let random_number = <u32>::decode(&mut random_seed.as_ref())
    //                     .expect("secure hashes should always be bigger than u32; qed");

    //                 // 3. Selects the winner from the participants lit
    //                 let winner_index = random_number as usize % participants.len();
    //                 let winner = participants.as_slice().get(winner_index).unwrap();

    //                 // 4. Transfers the total prize to the winner's account
    //                 let prize = T::Currency::free_balance(&Self::get_pallet_account());
    //                 T::Currency::transfer(
    //                     &Self::get_pallet_account(), 
    //                     &winner, 
    //                     prize, 
    //                     ExistenceRequirement::AllowDeath)?;

    //                 // 5. Resets the participants list, and gets ready for another lottery round
    //                 Participants::<T>::kill();

    //                 // 6. Notify the event
    //                 Self::deposit_event(Event::PrizeAwarded { winner: winner.clone() } );
    //             }, 
    //             None => {
    //                 // Notify the event (No participants)
    //                 Self::deposit_event(Event::ThereAreNoParticipants);
    //             }
    //         };

    //         Ok(())
    //     }
    // }

    #[pallet::call]
	impl<T: Config> Pallet<T> {
		/// set a new anchor or update an exist anchor
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn set_anchor(
			origin: OriginFor<T>,
			key: Vec<u8>,
			raw: Vec<u8>,
			protocol: Vec<u8>,
			pre:BlockNumberFor<T>
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			//0.check is on sell

			//1.param check
			ensure!(key.len() < 40, Error::<T>::KeyMaxLimited);				//1.1.check key length, <40
			ensure!(raw.len() < 4*1024*1024, Error::<T>::Base64MaxLimited);	//1.2.check raw(base64) length，<4M
			ensure!(protocol.len() < 256, Error::<T>::ProtocolMaxLimited);	//1.3.check protocal length, <256

			//1.1.convert key to lowcase
			let mut nkey:Vec<u8>;
			nkey=key.clone().as_mut_slice().to_vec();
			nkey.make_ascii_lowercase();

			let data = <AnchorOwner<T>>::get(&nkey); 		//check anchor status
			let current_block_number = <frame_system::Pallet<T>>::block_number();

			//2.check anchor to determine add or update
			if data.is_none() {
				//let val:u64=0;
				//let zero :BlockNumberFor<T> = val.saturated_into();
				//ensure!(pre == zero, Error::<T>::PreAnchorFailed);

                //ensure!(pre.is_zero(), Error::<T>::PreAnchorFailed);

				//2.1.create new anchor
				<AnchorOwner<T>>::insert(nkey, (&sender,current_block_number));
				
			}else{
				//2.2.update exists anchor
				let owner=data.ok_or(Error::<T>::AnchorNotExists)?;
				ensure!(sender == owner.0, Error::<T>::AnchorNotBelogToAccount);
				ensure!(pre == owner.1, Error::<T>::PreAnchorFailed);

				<AnchorOwner<T>>::try_mutate(&nkey, |status| -> DispatchResult {
					let d = status.as_mut().ok_or(Error::<T>::UnexceptDataError)?;
					d.1 = current_block_number;
					Ok(())
				})?;
			}

			Ok(())
		}
	}

    // impl<T: Config> Pallet<T> {

    //     fn get_pallet_account() -> T::AccountId {
    //         T::PalletId::get().into_account_truncating()
    //     }

    //     fn get_and_increment_nonce() -> Vec<u8> {
    //         let nonce = Nonce::<T>::get();
    //         Nonce::<T>::put(nonce.wrapping_add(1));
    //         nonce.encode()
    //     }
    // }
}
