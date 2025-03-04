//! These prelude re-exports are a set of exports that are commonly used from within the library.
//!
//! These are not publicly re-exported to the end user, and must stay as a private module.

pub use std::result::Result as StdResult;

pub use aformat::{aformat, aformat_into, ArrayString, CapStr};
pub use extract_map::{ExtractKey, ExtractMap, LendingIterator};
pub use serde_json::Value;
pub use small_fixed_array::{FixedArray, FixedString, TruncatingInto};
pub use to_arraystring::ToArrayString;

pub(crate) use super::utils::join_to_string;
pub use crate::error::{Error, Result};

pub type JsonMap = serde_json::Map<String, Value>;
