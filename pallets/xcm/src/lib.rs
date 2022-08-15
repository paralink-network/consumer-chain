#![cfg_attr(not(feature = "std"), no_std)]

use cumulus_pallet_xcm::{ensure_sibling_para, Origin as CumulusOrigin};
use cumulus_primitives_core::ParaId;
use frame_system::Config as SystemConfig;
use sp_std::prelude::*;
use xcm::latest::prelude::*;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	//use pallet_chainlink_feed::pallet::{ FeedOracle, RoundData, FeedInterface };

	//pub type FeedValue<T> = <<<T as Config>::Oracle as FeedOracle<T>>::Feed as FeedInterface<T>>::Value;
	//pub type FeedId<T> = <<T as Config>::Oracle as FeedOracle<T>>::FeedId;

	//pub trait FeedRequester<T: Config> {
	//fn request_latest_data(feed_id: FeedId<T>);
	//}

	//pub trait FeedReceiver<T: Config> {
	//fn receive_latest_data(feed_id: FeedId<T>, round_data: RoundDataOf<T>);
	//}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// The module configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Origin: From<<Self as SystemConfig>::Origin>
			+ Into<Result<CumulusOrigin, <Self as Config>::Origin>>;

		/// The overarching call type; we assume sibling chains use the same type.
		type Call: From<Call<Self>> + Encode;

		type XcmSender: SendXcm;

		//type Oracle: FeedOracle<Self>;

		//type FeedReceiver: FeedReceiver<Self>;
	}

	#[derive(Clone, Encode, Decode, Default, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	pub struct FeedId {
		pub id: u32,
	}

	#[derive(Clone, Encode, Decode, Default, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	pub struct FeedValue {
		pub value: u128,
	}

	/// Parachains interested in price feeds
	//#[pallet::storage]
	//pub(super) type RegisteredParachains<T: Config> =
	//StorageValue<_, Vec<(ParaId, FeedId)>, ValueQuery>;

	//pub struct Feed<T: Config> {
	//pub id: FeedId,
	//}

	//#[derive(Debug, Clone, PartialEq, Eq, Decode, Encode, Default, TypeInfo, Copy)]
	//pub struct RoundData<BlockNumber, Value> {
	//pub started_at: u32,
	//pub answer: u128,
	//pub updated_at: u32,
	//pub answered_in_round: u32,
	//}

	#[pallet::storage]
	#[pallet::getter(fn feeds)]
	pub(super) type Feeds<T: Config> = StorageMap<_, Twox64Concat, FeedId, FeedValue, OptionQuery>;

	//pub type RoundDataOf<T> = RoundData<<T as frame_system::Config>::BlockNumber, FeedValue<T>>;

	/// The total number of pings sent.
	//#[pallet::storage]
	//pub(super) type PingCount<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		FeedValueUpdated(FeedId, FeedValue, T::AccountId),
		CurrentFeedValue(FeedId, FeedValue, T::AccountId),
		//SendRequestForLatestPriceValue(ParaId, <T::Oracle as FeedOracle<T>>::FeedId),
		//SendRequestForLatestPriceValue(ParaId, <T::Oracle as FeedOracle<T>>::FeedId),
		//ReceiveRequestForLatestPriceValue(ParaId, <T::Oracle as FeedOracle<T>>::FeedId),
		//SendLatestPriceValue(ParaId, <T::Oracle as FeedOracle<T>>::FeedId, RoundDataOf<T>),
		//ReceiveLatestPriceValue(ParaId, <T::Oracle as FeedOracle<T>>::FeedId, RoundDataOf<T>),
		//ErrorSendingRequest(SendError, ParaId, <T::Oracle as FeedOracle<T>>::FeedId),
		//ErrorSendingLatestPriceValue(
		//SendError,
		//ParaId,
		//<T::Oracle as FeedOracle<T>>::FeedId,
		//RoundDataOf<T>,
		//),
	}

	#[pallet::error]
	pub enum Error<T> {
		FeedMissing,
	}

	//#[pallet::hooks]
	//impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
	//fn on_finalize(_n: T::BlockNumber) {
	//for (parachain_id, feed_id) in RegisteredParachains::<T>::get().into_iter() {
	//Self::send_latest_data_through_xcm(
	//parachain_id,
	//feed_id.clone(),
	//Self::get_latest_data(feed_id),
	//);
	//}
	//}
	//}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn store_latest_data(
			origin: OriginFor<T>,
			feed_id: FeedId,
			value: FeedValue,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Feeds::<T>::insert(feed_id.clone(), value.clone());

			Self::deposit_event(Event::FeedValueUpdated(feed_id, value, who));
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn get_latest_data(origin: OriginFor<T>, feed_id: FeedId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let feed = Feeds::<T>::get(feed_id.clone());
			match feed {
				Some(x) => Self::deposit_event(Event::CurrentFeedValue(feed_id, x, who)),
				None => Self::deposit_event(Event::CurrentFeedValue(
					feed_id,
					FeedValue { value: 666 },
					who,
				)),
			}

			Ok(())
		}

		//#[pallet::weight(0)]
		//pub fn request_latest_data(_origin: OriginFor<T>, feed_id: FeedId) -> DispatchResult {
		//<Pallet<T> as FeedRequester<T>>::request_latest_data(feed_id);
		//Ok(())
		//}

		//#[pallet::weight(0)]
		//pub fn receive_latest_data(
		//origin: OriginFor<T>,
		//feed_id: FeedId<T>,
		//latest_round_data: RoundDataOf<T>,
		//) -> DispatchResult {
		//log::info!("***** Sublink XCM store_latest_data called");
		//let para = ensure_sibling_para(<T as Config>::Origin::from(origin))?;

		//log::info!("***** Sublink XCM Received latest_value = {:?}", latest_round_data.clone());
		//<T as Config>::FeedReceiver::receive_latest_data(
		//feed_id.clone(),
		//latest_round_data.clone(),
		//);
		//Self::deposit_event(Event::ReceiveLatestPriceValue(para, feed_id, latest_round_data));
		//Ok(())
		//}

		//#[pallet::weight(0)]
		//pub fn send_latest_data(origin: OriginFor<T>, feed_id: FeedId<T>) -> DispatchResult {
		//log::info!("***** Sublink XCM get_latest_data called");
		//let parachain_id = ensure_sibling_para(<T as Config>::Origin::from(origin))?;

		//Self::deposit_event(Event::ReceiveRequestForLatestPriceValue(
		//parachain_id,
		//feed_id.clone(),
		//));
		//RegisteredParachains::<T>::mutate(|t| {
		//if t.iter().position(|(p, _)| p == &parachain_id) == None {
		//t.push((parachain_id, feed_id.clone()));
		//}
		//});

		//Self::send_latest_data_through_xcm(
		//parachain_id,
		//feed_id.clone(),
		//Self::get_latest_data(feed_id),
		//);

		//Ok(())
		//}
	}

	//  ---------------------- non-callable
	impl<T: Config> Pallet<T> {
		pub fn latest_data(feed_id: FeedId) -> FeedValue {
			Feeds::<T>::get(feed_id.clone()).unwrap_or_else(|| {
				log::info!("***** Sublink Parachain Oracle no round for {:?}", feed_id);
				FeedValue { value: 666 }
			})
		}
		//fn get_latest_data(feed_id: FeedId<T>) -> Option<RoundDataOf<T>> {
		//let feed = T::Oracle::feed(feed_id.clone());
		//match feed {
		//Some(feed_value) => Some(feed_value.latest_data()),
		//None => {
		//log::info!("***** Sublink XCM No feed for = {:?}", feed_id);
		//None
		//},
		//}
		//}

		//fn send_latest_data_through_xcm(
		//parachain_id: ParaId,
		//feed_id: FeedId,
		//latest_round_data: Option<RoundDataOf<T>>,
		//) {
		//if let Some(latest_round_data_value) = latest_round_data {
		//match T::XcmSender::send_xcm(
		//(1, Junction::Parachain(parachain_id.into())),
		//Xcm(vec![Transact {
		//origin_type: OriginKind::Native,
		//require_weight_at_most: 1_000,
		//call: <T as Config>::Call::from(Call::<T>::receive_latest_data {
		//feed_id: feed_id.clone(),
		//latest_round_data: latest_round_data_value.clone(),
		//})
		//.encode()
		//.into(),
		//}]),
		//) {
		//Ok(()) => {
		//log::info!("***** Sublink XCM get_latest_data called store_latest_data");
		//Self::deposit_event(Event::SendLatestPriceValue(
		//parachain_id,
		//feed_id,
		//latest_round_data_value,
		//))
		//},
		//Err(e) => {
		//log::error!(
		//"***** Sublink XCM get_latest_data cannot called store_latest_data"
		//);
		//Self::deposit_event(Event::ErrorSendingLatestPriceValue(
		//e,
		//parachain_id,
		//feed_id,
		//latest_round_data_value,
		//))
		//},
		//}
		//}
		//}
	}

	//impl<T: Config> FeedRequester<T> for Pallet<T> {
	//fn request_latest_data(feed_id: FeedId<T>) {
	//log::info!("***** Sublink XCM ask_latest_data called");

	//// TODO LTK : need to register SubLink parachain id
	//let para: ParaId = 2000.into();
	//match T::XcmSender::send_xcm(
	//(1, Junction::Parachain(para.into())),
	//Xcm(vec![Transact {
	//origin_type: OriginKind::Native,
	//require_weight_at_most: 1_000,
	//call: <T as Config>::Call::from(Call::<T>::send_latest_data {
	//feed_id: feed_id.clone(),
	//})
	//.encode()
	//.into(),
	//}]),
	//) {
	//Ok(()) => {
	//log::info!("***** Sublink XCM ask_latest_data called get_latest_data");
	//Self::deposit_event(Event::SendRequestForLatestPriceValue(para, feed_id))
	//},
	//Err(e) => {
	//log::error!("***** Sublink XCM ask_latest_data cannot called get_latest_data");
	//Self::deposit_event(Event::ErrorSendingRequest(e, para, feed_id))
	//},
	//}
	//}
	//}

	//impl<T: Config> FeedReceiver<T> for () {
	//fn receive_latest_data(_feed_id: FeedId<T>, _latest_round_data: RoundDataOf<T>) {
	//// do_nothing
	//}
	//}
}
