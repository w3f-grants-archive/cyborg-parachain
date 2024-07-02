#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use scale_info::{ TypeInfo };
use frame_support::sp_runtime::RuntimeDebug;
use codec::{ Encode, Decode, MaxEncodedLen };

pub type WorkerId = u64;

pub type Domain = u8;

#[derive(PartialEq, Eq, Clone, Decode, Encode, TypeInfo, Debug, MaxEncodedLen)]
pub enum WorkerStatusType {
	Active,
	Busy,
	Inactive,
}

#[derive(Default, PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct Ip {
	pub ipv4: Option<u8>,
	pub ipv6: Option<u8>,
	pub port: u32,
}

#[derive(Default, PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen )]
pub struct WorkerAPI {
	pub ip : Option<Ip>,
	pub domain: Option<Domain>,
}

#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct Worker<AccountId, BlockNumber> {
	pub id: WorkerId,
	pub owner: AccountId,
	pub start_block: BlockNumber,
	pub status: WorkerStatusType,
	pub api: WorkerAPI,
}

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		/// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_runtime_types/index.html>
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: crate::weights::WeightInfo;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::type_value]
    pub fn WorkerCountDefault() -> WorkerId {
        0
    }

	/// Keeps track of workers per account if any
	#[pallet::storage]
	#[pallet::getter(fn account_workers)]
	pub type AccountWorkers<T: Config> = 
		StorageMap<_, Twox64Concat, T::AccountId, WorkerId, OptionQuery>;

	/// Worker Cluster information
	#[pallet::storage]
	#[pallet::getter(fn get_worker_clusters)]
	pub type WorkerClusters<T: Config> = 
		StorageMap<_, Twox64Concat, (T::AccountId, WorkerId), Worker<T::AccountId, BlockNumberFor<T>>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		WorkerRegistered{ creator: T::AccountId },
		WorkerRemoved{ creator: T::AccountId, worker_id: WorkerId },
	}

	/// Pallet Errors
	#[pallet::error]
	pub enum Error<T> {
		WorkerRegisterMissingIpOrDomain,
		WorkerExists,
		WorkerDoesNotExist,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	/// Registers a Worker with either a domain or ip and initialize it with an inactive status.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
		pub fn register_worker(
			origin: OriginFor<T>, 
			ip: Option<Ip>, 
			domain: Option<Domain>,
		) -> DispatchResultWithPostInfo {
			let creator = ensure_signed(origin)?;

			// check ip or domain exists
			ensure!(ip.clone().unwrap().ipv4.is_some() || ip.clone().unwrap().ipv6.is_some() || domain.is_some(), Error::<T>::WorkerRegisterMissingIpOrDomain);
			
			//check cluster
			ensure!(AccountWorkers::<T>::contains_key(creator.clone()) == false, 
			Error::<T>::WorkerExists);

			let worker_id: WorkerId = match AccountWorkers::<T>::get(creator.clone()) {
				Some(id) => {
					AccountWorkers::<T>::insert(creator.clone(), id + 1);
					id + 1
				},
				None => {
					AccountWorkers::<T>::insert(creator.clone(), 0);
					0
				}
			};

			let worker = Worker {
				id: worker_id.clone(),
				owner: creator.clone(),
				start_block: <frame_system::Pallet<T>>::block_number(),
				status: WorkerStatusType::Inactive,
				api: WorkerAPI {
					ip, domain
				},
			};

			// update storage
			AccountWorkers::<T>::insert(creator.clone(), worker_id.clone());
			WorkerClusters::<T>::insert((creator.clone(), worker_id.clone()), worker);

			// Emit an event.
			Self::deposit_event(Event::WorkerRegistered { creator });

			// Return a successful DispatchResultWithPostInfo
			Ok(().into())
		}

		/// Remove Worker from storage
		#[pallet::call_index(1)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
		pub fn remove_worker(
			origin: OriginFor<T>, 
			worker_id: WorkerId, 
		) -> DispatchResultWithPostInfo {
			let creator = ensure_signed(origin)?;
			
			ensure!(WorkerClusters::<T>::get((creator.clone(), worker_id)) != None, 
			Error::<T>::WorkerDoesNotExist);

			// update storage
			WorkerClusters::<T>::remove((creator.clone(), worker_id));

			// Emit an event.
			Self::deposit_event(Event::WorkerRemoved { creator, worker_id });


			// Return a successful DispatchResultWithPostInfo
			Ok(().into())
		}
	}
}
