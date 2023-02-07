// This file is part of lattice

//! Lattice inherents for block category


#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use sp_inherents::{InherentData, InherentIdentifier, IsFatalError};

/// The identifier for the block `category` inherent.
pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"telblock";

/// The type of the inherent.
pub type InherentType = Category;

/// Unit type wrapper that represents a Category.
///
/// Categories are assigned to blocks and determine base fees.
#[derive(Debug, Encode, Decode, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Cat1,
    Cat2,
}

impl Default for Category {
    fn default() -> Self {
        Self::Cat1
    }
}

/// Errors that can occur while checking the category inherent.
#[derive(Encode, sp_runtime::RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Decode, thiserror::Error))]
pub enum InherentError {
	#[cfg_attr(feature = "std", error("Unknown block category inherent error."))]
	Unknown,
	// /// The timestamp is valid in the future.
	// /// This is a non-fatal-error and will not stop checking the inherents.
	// #[cfg_attr(feature = "std", error("Block will be valid at {0}."))]
	// ValidAtTimestamp(InherentType),
	// /// The block timestamp is too far in the future
	// #[cfg_attr(feature = "std", error("The timestamp of the block is too far in the future."))]
	// TooFarInFuture,
}

impl IsFatalError for InherentError {
    fn is_fatal_error(&self) -> bool {
        match self {
            InherentError::Unknown => false,
            // InherentError::CategoryMissing => true,
        }
    }
}

impl InherentError {
    /// Try to create an instance out of the given identifier and data.
    #[cfg(feature = "std")]
    pub fn try_from(id: &InherentIdentifier, mut data: &[u8]) -> Option<Self> {
        if id == &INHERENT_IDENTIFIER {
            <InherentError as codec::Decode>::decode(&mut data).ok()
        } else {
            None
        }
    }
}

/// Auxiliary trait to extract block category inherent data.
pub trait BlockCategoryInherentData {
    /// Get block category inherent data.
    fn category_inherent_data(&self) -> Result<Option<InherentType>, sp_inherents::Error>;
	/// Replace block category inherent type.
	fn category_replace_inherent_data(&mut self, new: InherentType);
}

impl BlockCategoryInherentData for InherentData {
    fn category_inherent_data(&self) -> Result<Option<InherentType>, sp_inherents::Error> {
        self.get_data(&INHERENT_IDENTIFIER)
    }

	fn category_replace_inherent_data(&mut self, new: InherentType) {
		self.replace_data(INHERENT_IDENTIFIER, &new);
	}
}

#[cfg(feature = "std")]
pub struct InherentDataProvider {
    category: InherentType,
}

#[cfg(feature = "std")]
impl InherentDataProvider {
    /// Create `Self` using the given `Category`.
    pub fn new(category: Category) -> Self {
        Self { category }
    }

    /// Return the category of this inherent data provider.
    pub fn category(&self) -> InherentType {
        self.category
    }
}

#[cfg(feature = "std")]
impl sp_std::ops::Deref for InherentDataProvider {
	type Target = InherentType;

	fn deref(&self) -> &Self::Target {
		&self.category
	}
}


#[cfg(feature = "std")]
#[async_trait::async_trait]
impl sp_inherents::InherentDataProvider for InherentDataProvider {
	fn provide_inherent_data(
		&self,
		inherent_data: &mut InherentData,
	) -> Result<(), sp_inherents::Error> {
		inherent_data.put_data(INHERENT_IDENTIFIER, &self.category)
	}

	async fn try_handle_error(
		&self,
		identifier: &InherentIdentifier,
		error: &[u8],
	) -> Option<Result<(), sp_inherents::Error>> {
		if *identifier != INHERENT_IDENTIFIER {
			return None
		}
		// TODO: handle these errors
        todo!("try handle error inside lattice inherent data provider")

		// match InherentError::try_from(&INHERENT_IDENTIFIER, error)? {
		// 	InherentError::ValidAtTimestamp(valid) => {
		// 		let max_drift = self.max_drift;
		// 		let timestamp = self.timestamp;
		// 		// halt import until timestamp is valid.
		// 		// reject when too far ahead.
		// 		if valid > timestamp + max_drift {
		// 			return Some(Err(sp_inherents::Error::Application(Box::from(
		// 				InherentError::TooFarInFuture,
		// 			))))
		// 		}

		// 		let diff = valid.checked_sub(timestamp).unwrap_or_default();
		// 		log::info!(
		// 			target: "timestamp",
		// 			"halting for block {} milliseconds in the future",
		// 			diff.0,
		// 		);

		// 		futures_timer::Delay::new(diff.as_duration()).await;

		// 		Some(Ok(()))
		// 	},
		// 	o => Some(Err(sp_inherents::Error::Application(Box::from(o)))),
		// }
	}
}

