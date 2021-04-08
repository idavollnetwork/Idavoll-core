// This file is part of Idavoll Node.

// Copyright (C) 2021 Idavoll Network.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit="128"]
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error,
					dispatch, traits::{Get,EnsureOrigin},
					Parameter,ensure};
use frame_system::ensure_signed;
use sp_runtime::{Permill, ModuleId, RuntimeDebug,
				 traits::{Zero, StaticLookup, AccountIdConversion,
						  Saturating,AtLeast32BitUnsigned,AtLeast32Bit,
				 }
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod organization;
mod rules;
mod voting;
mod utils;

pub use organization::{OrgInfo, Proposal,OrganizationId,ProposalDetailOf};
use idavoll_asset::{token::BaseToken};

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	/// The idavoll's module id, used for deriving its sovereign account ID,use to organization id.
	type ModuleId: Get<ModuleId>;
	/// the Asset Handler will handle all op in the voting about asset operation.
	type AssetHandle: BaseToken<Self::AccountId>;
	type AssetId: Parameter + AtLeast32Bit + Default + Copy;
}

type BalanceOf<T> = <<T as frame_system::Trait>::AccountId>::Balance;
pub type OrgCount = u32;
pub type OrgInfoOf<T> = OrgInfo<
	<T as frame_system::Trait>::AccountId,
	<<T as frame_system::Trait>::AccountId>::Balance,
	T::AssetId,
>;
type ProposalIdOf<T> = <T as frame_system::Trait>::Hash;
type ProposalOf<T> = Proposal<
	Vec<u8>,
	ProposalDetailOf<T>,
	<T as frame_system::Trait>::AccountId,
>;

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	trait Store for Module<T: Trait> as IdavollModule {
		pub Counter get(fn counter): OrgCount = 0;
		pub OrgInfos get(fn OrgInfos): map hasher(blake2_128_concat) T::AccountId => Option<OrgInfoOf<T>>;
        pub Proposals get(fn proposals): map hasher(blake2_128_concat) ProposalIdOf<T> => Option<ProposalOf<T>>;
		Something get(fn something): Option<u32>;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, AccountId),
		/// A proposal has been finalized with the following result. \[proposal id, result\]
        ProposalFinalized(ProposalIdOf<T>, dispatch::DispatchResult),
        /// A proposal has been passed. \[proposal id]
        ProposalPassed(ProposalIdOf<T>),
        /// create a proposal.		\[organization id,proposal id,creator]
        ProposalCreated(AccountId,ProposalIdOf<T>,AccountId),
        /// Proposal Refused \[proposal id]
        ProposalRefuse(AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// need the maximum number for the storage value for the fixed type.
		StorageOverflow,
		OrganizationNotFound,
		/// not found the proposal by id in the runtime storage
		ProposalNotFound,
		ProposalDecodeFailed,
		ProposalDuplicate,
		ProposalExpired,
		NotMember,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;
		const ModuleId: ModuleId = T::ModuleId::get();
		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			Something::put(something);

			// Emit an event.
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			// Return a successful DispatchResult
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn cause_error(origin) -> dispatch::DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match Something::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					Something::put(new);
					Ok(())
				},
			}
		}
	}
}

impl<T: Trait> Module<T> {

}
