//! API tests. These are all private because why would they need to be public. sorry
//! All tests must be run in debug mode so they don't write any actual data to the user data
// Make sure tests are run in debug mode so user data is unaffected
#[cfg(not(debug_assertions))]
const DEBUG: _ = compile_error!("Tests must be run in debug mode");
mod login;
mod new_user;
mod session;
