//! Module containing event enabler.
//!
//! This is used to create an API for the user to enable particular events for a tasklet.

use crate::api::InitError;
use crate::event::{EventId, EventSet};
use crate::event_manager::EventManager;

/// Helper struct to subscribe tasklet to particular events.
///
/// This is only created with [subscribe_tasklet_to_events](crate::api::InitApi::subscribe_tasklet_to_events)
/// and should not be moved or used anywhere outside of the system initialization.
///
/// # Safety
/// This shall be only used during system initialization. All usage after the initialization will
/// be an undefined behavior.
pub struct EventEnabler {
    /// Set to which events will be added.
    event_set: &'static EventSet,
    /// Event manager.
    event_manager: &'static EventManager,
}

impl EventEnabler {
    /// Creates new subscriber instance.
    ///
    /// # Parameters
    /// * `event_set` - Set to which events will be added.
    pub(crate) fn new(event_set: &'static EventSet, event_manager: &'static EventManager) -> Self {
        EventEnabler {
            event_set,
            event_manager,
        }
    }

    /// Enabled given event.
    ///
    /// Enabling works by adding event to an event set that is assigned to a tasklet.
    ///
    /// # Parameters
    /// * `event_id` - ID of event to enable.
    ///
    /// # Return
    /// `Self` reference if successful, `InitError` otherwise.
    pub fn enable(&self, event_id: EventId) -> Result<&Self, InitError> {
        let event = match self.event_manager.get_event(event_id) {
            Some(event) => event,
            None => return Err(InitError::EventNotFound(event_id)),
        };

        // This is safe, because this structure can be only created at system initialization.
        unsafe {
            event.add_set(self.event_set)?;
        }

        Ok(self)
    }
}
