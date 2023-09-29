//! Module representing user-accessible peripherals.

use samv71_hal::{pac, pmc::PMC};

/// Peripherals structure.
/// These peripherals can be used to create HAL drivers in user code.
///
/// Core peripherals (like PMC) are stored already in form of HAL drivers, instead of
/// PAC instances, as they are core components that most applications will have
/// to create instances of, and use.
pub struct UserPeripherals {
    /// Chip ID.
    pub chip_id: Option<pac::CHIPID>,
    /// NVIC
    pub nvic: Option<pac::NVIC>,
    /// I/O Port A.
    pub pio_a: Option<pac::PIOA>,
    /// I/O Port B.
    pub pio_b: Option<pac::PIOB>,
    /// I/O Port C.
    pub pio_c: Option<pac::PIOC>,
    /// I/O Port D.
    pub pio_d: Option<pac::PIOD>,
    /// I/O Port E.
    pub pio_e: Option<pac::PIOE>,
    /// Clocks controller.
    /// This is HAL driver instance that provides abstraction over PMC.
    pub pmc: Option<PMC>,
    /// Timer Counter 1.
    pub timer_counter1: Option<pac::TC1>,
    /// Timer Counter 2.
    pub timer_counter2: Option<pac::TC2>,
    /// Timer Counter 3.
    pub timer_counter3: Option<pac::TC3>,
}
