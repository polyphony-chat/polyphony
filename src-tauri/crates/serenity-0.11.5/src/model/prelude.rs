//! The model prelude re-exports all types in the model sub-modules.
//!
//! This allows for quick and easy access to all of the model types.
//!
//! # Examples
//!
//! Import all model types into scope:
//!
//! ```rust,no_run
//! use serenity::model::prelude::*;
//! ```

pub use super::application::*;
pub use super::channel::*;
pub use super::connection::*;
pub use super::event::*;
pub use super::gateway::*;
pub use super::guild::audit_log::*;
pub use super::guild::*;
pub use super::id::*;
#[allow(deprecated)]
pub use super::interactions::*;
pub use super::invite::*;
pub use super::mention::*;
pub use super::misc::*;
#[allow(deprecated)]
pub use super::oauth2::*;
pub use super::permissions::*;
pub use super::sticker::*;
pub use super::user::*;
pub use super::voice::*;
pub use super::webhook::*;
pub use super::*;