//! Runtime configuration management for canisters.
//!
//! Provides the `EnvState` type for storing and managing configuration
//! passed via init args at deployment time.

use candid::{CandidType, Deserialize};

/// Trait that configuration types must implement.
///
/// This ensures the config can be serialized/deserialized via Candid
/// and stored in stable memory.
pub trait Config: CandidType + for<'de> Deserialize<'de> + Clone + 'static {}

/// Runtime environment state container.
///
/// Stores configuration passed during canister initialization and provides
/// methods for accessing and updating it.
///
/// ## Example
///
/// ```rust,no_run
/// use ic_canister_core::config::{Config, EnvState};
/// use candid::{CandidType, Deserialize};
///
/// #[derive(Clone, CandidType, Deserialize)]
/// pub struct MyConfig {
///     pub api_url: String,
///     pub log_level: String,
/// }
///
/// impl Config for MyConfig {}
///
/// let mut state = EnvState::<MyConfig>::new();
/// let config = MyConfig {
///     api_url: "https://api.example.com".to_string(),
///     log_level: "info".to_string(),
/// };
///
/// state.init(config.clone());
/// assert_eq!(state.get().api_url, "https://api.example.com");
/// ```
#[derive(Clone)]
pub struct EnvState<T: Config> {
    inner: Option<T>,
}

impl<T: Config> EnvState<T> {
    /// Create a new uninitialized state.
    pub fn new() -> Self {
        Self { inner: None }
    }

    /// Initialize the state with configuration.
    ///
    /// Should be called from `#[ic_cdk::init]`.
    ///
    /// ## Panics
    ///
    /// Panics if called when state is already initialized.
    pub fn init(&mut self, config: T) {
        if self.inner.is_some() {
            ic_cdk::trap("State already initialized");
        }
        self.inner = Some(config);
    }

    /// Get a reference to the configuration.
    ///
    /// ## Panics
    ///
    /// Panics if state is not initialized.
    pub fn get(&self) -> &T {
        self.inner
            .as_ref()
            .expect("State not initialized - call init() first")
    }

    /// Get a mutable reference to the configuration.
    ///
    /// ## Panics
    ///
    /// Panics if state is not initialized.
    pub fn get_mut(&mut self) -> &mut T {
        self.inner
            .as_mut()
            .expect("State not initialized - call init() first")
    }

    /// Update the configuration.
    ///
    /// Useful for runtime config updates via canister methods.
    ///
    /// ## Panics
    ///
    /// Panics if state is not initialized.
    pub fn update(&mut self, config: T) {
        if self.inner.is_none() {
            ic_cdk::trap("Cannot update uninitialized state");
        }
        self.inner = Some(config);
    }

    /// Check if state is initialized.
    pub fn is_initialized(&self) -> bool {
        self.inner.is_some()
    }

    /// Take the configuration out of the state.
    ///
    /// Leaves the state uninitialized.
    pub fn take(&mut self) -> Option<T> {
        self.inner.take()
    }
}

impl<T: Config> Default for EnvState<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, CandidType, Deserialize, Debug, PartialEq)]
    struct TestConfig {
        value: String,
    }

    impl Config for TestConfig {}

    #[test]
    fn test_new_state_is_uninitialized() {
        let state = EnvState::<TestConfig>::new();
        assert!(!state.is_initialized());
    }

    #[test]
    fn test_init_and_get() {
        let mut state = EnvState::new();
        let config = TestConfig {
            value: "test".to_string(),
        };

        state.init(config.clone());
        assert!(state.is_initialized());
        assert_eq!(state.get().value, "test");
    }

    #[test]
    fn test_update() {
        let mut state = EnvState::new();
        state.init(TestConfig {
            value: "initial".to_string(),
        });

        state.update(TestConfig {
            value: "updated".to_string(),
        });

        assert_eq!(state.get().value, "updated");
    }

    #[test]
    #[should_panic(expected = "State already initialized")]
    fn test_double_init_panics() {
        let mut state = EnvState::new();
        state.init(TestConfig {
            value: "first".to_string(),
        });
        state.init(TestConfig {
            value: "second".to_string(),
        });
    }

    #[test]
    #[should_panic(expected = "State not initialized")]
    fn test_get_uninitialized_panics() {
        let state = EnvState::<TestConfig>::new();
        let _ = state.get();
    }

    #[test]
    fn test_take() {
        let mut state = EnvState::new();
        state.init(TestConfig {
            value: "test".to_string(),
        });

        let config = state.take();
        assert!(config.is_some());
        assert!(!state.is_initialized());
    }
}
