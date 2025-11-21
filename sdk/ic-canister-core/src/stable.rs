//! Stable storage helpers for persisting state across canister upgrades.
//!
//! Provides simple serialization/deserialization for config stored in stable memory.

use candid::{decode_one, encode_one, CandidType};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::DefaultMemoryImpl;
use serde::Deserialize;
use std::cell::RefCell;

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

/// Save a value to stable memory using Candid serialization.
///
/// Should be called from `#[ic_cdk::pre_upgrade]`.
///
/// ## Example
///
/// ```rust,no_run
/// use ic_canister_core::stable::stable_save;
///
/// #[derive(candid::CandidType)]
/// struct Config {
///     value: String,
/// }
///
/// #[ic_cdk::pre_upgrade]
/// fn pre_upgrade() {
///     let config = Config { value: "data".to_string() };
///     stable_save(&config).expect("Failed to save config");
/// }
/// ```
pub fn stable_save<T: CandidType>(value: &T) -> Result<(), String> {
    let bytes = encode_one(value).map_err(|e| format!("Candid encoding failed: {}", e))?;

    ic_cdk::api::stable::stable_write(0, &bytes);

    Ok(())
}

/// Load a value from stable memory using Candid deserialization.
///
/// Should be called from `#[ic_cdk::post_upgrade]`.
///
/// ## Example
///
/// ```rust,no_run
/// use ic_canister_core::stable::stable_restore;
///
/// #[derive(candid::CandidType, candid::Deserialize)]
/// struct Config {
///     value: String,
/// }
///
/// #[ic_cdk::post_upgrade]
/// fn post_upgrade() {
///     let config: Config = stable_restore().expect("Failed to restore config");
/// }
/// ```
pub fn stable_restore<T>() -> Result<T, String>
where
    T: CandidType + for<'de> Deserialize<'de>,
{
    let size = ic_cdk::api::stable::stable_size();
    if size == 0 {
        return Err("Stable memory is empty".to_string());
    }

    // Allocate buffer and read stable memory into it
    let byte_count = size as usize * 65536;
    let mut bytes = vec![0u8; byte_count];
    ic_cdk::api::stable::stable_read(0, &mut bytes);

    decode_one(&bytes).map_err(|e| format!("Candid decoding failed: {}", e))
}

/// Helper for managing multiple stable memories.
///
/// Use when you need separate stable memories for different data structures.
pub struct StableMemoryManager {
    inner: RefCell<MemoryManager<DefaultMemoryImpl>>,
}

impl StableMemoryManager {
    pub fn new() -> Self {
        Self {
            inner: RefCell::new(MemoryManager::init(DefaultMemoryImpl::default())),
        }
    }

    pub fn get(&self, id: MemoryId) -> Memory {
        self.inner.borrow().get(id)
    }
}

impl Default for StableMemoryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(CandidType, Deserialize, Debug, PartialEq)]
    struct TestData {
        value: u32,
        text: String,
    }

    // Note: These tests would need to run in a canister environment
    // with stable memory available. They're here for documentation.

    #[test]
    fn test_encode_decode() {
        let data = TestData {
            value: 42,
            text: "test".to_string(),
        };

        let bytes = encode_one(&data).unwrap();
        let decoded: TestData = decode_one(&bytes).unwrap();

        assert_eq!(decoded, data);
    }
}
