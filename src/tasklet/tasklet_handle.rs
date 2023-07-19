//! Handle to a tasklet.
//!
//! This module contains tasklet handle implementation, which is used to reference a tasklet in the
//! system.

use crate::task::Task;
use crate::tasklet::Tasklet;

/// Tasklet handle.
///
/// Tasklet handle is available to the user of the system to reference and interact with the
/// tasklet via exposed interface. All system API functions shall use handles when a reference to
/// tasklet is required.
///
/// # Generic Parameters
/// * `T` - Type that is processed by the tasklet.
/// * `C` - Type of tasklet context data.
pub struct TaskletHandle<T: 'static, C: 'static> {
    /// Reference to the tasklet.
    tasklet: &'static Tasklet<T, C>,
}

impl<T, C> TaskletHandle<T, C> {
    /// Creates new tasklet handle.
    ///
    /// # Parameters
    /// * `tasklet` - Pointer to the tasklet.
    pub(crate) fn new(tasklet: &'static Tasklet<T, C>) -> Self {
        TaskletHandle { tasklet }
    }

    /// Returns name of this tasklet.
    #[inline(always)]
    pub fn get_name(&self) -> &'static str {
        self.tasklet.get_name()
    }

    /// Returns reference to the tasklet.
    pub(crate) fn tasklet(&self) -> &'static Tasklet<T, C> {
        self.tasklet
    }
}
