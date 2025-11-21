//! Authorization helpers for checking caller permissions.
//!
//! Provides utilities for verifying canister controllers and managing admin lists.

use candid::Principal;

/// Check if the caller is a controller of this canister.
///
/// ## Example
///
/// ```rust,no_run
/// use ic_canister_core::auth::is_controller;
///
/// #[ic_cdk::update]
/// fn admin_only_method() {
///     let caller = ic_cdk::caller();
///     if !is_controller(caller) {
///         ic_cdk::trap("Unauthorized: caller is not a controller");
///     }
///     // ... admin logic ...
/// }
/// ```
pub fn is_controller(principal: Principal) -> bool {
    ic_cdk::api::is_controller(&principal)
}

/// Check if the caller is in the provided admin list.
///
/// ## Example
///
/// ```rust,no_run
/// use ic_canister_core::auth::is_admin;
/// use candid::Principal;
///
/// let admins = vec![
///     Principal::from_text("aaaaa-aa").unwrap(),
/// ];
///
/// let caller = ic_cdk::caller();
/// if !is_admin(caller, &admins) {
///     ic_cdk::trap("Unauthorized: caller is not an admin");
/// }
/// ```
pub fn is_admin(principal: Principal, admin_list: &[Principal]) -> bool {
    admin_list.contains(&principal)
}

/// Require that caller is a controller, otherwise trap.
///
/// ## Example
///
/// ```rust,no_run
/// use ic_canister_core::auth::require_controller;
///
/// #[ic_cdk::update]
/// fn sensitive_operation() {
///     require_controller();
///     // ... operation only controllers can perform ...
/// }
/// ```
pub fn require_controller() {
    let caller = ic_cdk::caller();
    if !is_controller(caller) {
        ic_cdk::trap("Unauthorized: caller must be a controller");
    }
}

/// Require that caller is in the admin list, otherwise trap.
///
/// ## Example
///
/// ```rust,no_run
/// use ic_canister_core::auth::require_admin;
/// use candid::Principal;
///
/// let admins = vec![Principal::from_text("aaaaa-aa").unwrap()];
///
/// #[ic_cdk::update]
/// fn admin_operation() {
///     require_admin(&admins);
///     // ... operation only admins can perform ...
/// }
/// ```
pub fn require_admin(admin_list: &[Principal]) {
    let caller = ic_cdk::caller();
    if !is_admin(caller, admin_list) {
        ic_cdk::trap("Unauthorized: caller must be an admin");
    }
}

/// Require that caller is either a controller or in the admin list.
///
/// ## Example
///
/// ```rust,no_run
/// use ic_canister_core::auth::require_controller_or_admin;
/// use candid::Principal;
///
/// let admins = vec![Principal::from_text("aaaaa-aa").unwrap()];
///
/// #[ic_cdk::update]
/// fn privileged_operation() {
///     require_controller_or_admin(&admins);
///     // ... operation for controllers or admins ...
/// }
/// ```
pub fn require_controller_or_admin(admin_list: &[Principal]) {
    let caller = ic_cdk::caller();
    if !is_controller(caller) && !is_admin(caller, admin_list) {
        ic_cdk::trap("Unauthorized: caller must be a controller or admin");
    }
}

/// Get the caller's principal.
///
/// Convenience wrapper around `ic_cdk::caller()`.
pub fn get_caller() -> Principal {
    ic_cdk::caller()
}

/// Check if caller is anonymous.
pub fn is_anonymous(principal: Principal) -> bool {
    principal == Principal::anonymous()
}

/// Require that caller is NOT anonymous.
pub fn require_authenticated() {
    let caller = ic_cdk::caller();
    if is_anonymous(caller) {
        ic_cdk::trap("Unauthorized: anonymous caller not allowed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anonymous() {
        assert!(is_anonymous(Principal::anonymous()));
        assert!(!is_anonymous(
            Principal::from_text("aaaaa-aa").unwrap()
        ));
    }

    #[test]
    fn test_is_admin() {
        let admin = Principal::from_text("aaaaa-aa").unwrap();
        let non_admin = Principal::from_text("bbbbb-bb").unwrap();
        let admin_list = vec![admin];

        assert!(is_admin(admin, &admin_list));
        assert!(!is_admin(non_admin, &admin_list));
    }
}
