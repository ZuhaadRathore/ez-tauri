//! Stronghold integration for secure data storage.
//!
//! Provides a wrapper around Tauri's Stronghold plugin for managing
//! encrypted storage of sensitive application data.

use tauri_plugin_stronghold::stronghold::Stronghold;
use thiserror::Error;

/// Errors that can occur during Stronghold operations.
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Tauri(#[from] tauri::Error),
    #[error(transparent)]
    Stronghold(#[from] tauri_plugin_stronghold::stronghold::Error),
}

/// Wrapper around Stronghold for managing encrypted storage operations.
pub struct StrongholdManager(Stronghold);

impl StrongholdManager {
    /// Creates a new StrongholdManager with the given Stronghold instance.
    pub fn new(stronghold: Stronghold) -> Self {
        Self(stronghold)
    }

    /// Returns a reference to the underlying Stronghold instance.
    pub fn stronghold(&self) -> &Stronghold {
        &self.0
    }

    /// Returns a mutable reference to the underlying Stronghold instance.
    pub fn stronghold_mut(&mut self) -> &mut Stronghold {
        &mut self.0
    }
}