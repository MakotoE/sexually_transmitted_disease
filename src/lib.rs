//! When you add this crate as a dependency, you can refer to `std` modules as `sexually_transmitted_disease`.
//!
//! For example:
//! ```rust
//! # use std as sexually_transmitted_disease;
//! fn main() {
//!     // Disease vector
//!     let diseases: sexually_transmitted_disease::vec::Vec<&str> =
//!         ["HIV/AIDS", "HPV", "herpes", "chlamydia", "hepatitis"].into();
//!
//!     let (sender, receiver) = sexually_transmitted_disease::sync::mpsc::channel();
//!     for disease in diseases {
//!         sender.send(disease);
//!     }
//!
//!     println!("{}", receiver.recv().unwrap()); // HIV/AIDS
//! }
//!
//! ```

pub use std::*;
