//! Provides interrupt features for `UserButton` on PA0 for the board
use cortex_m::peripheral::NVIC;
use stm32f3xx_hal::pac::exti::{FTSR1, IMR1, RTSR1};
use stm32f3xx_hal::pac::syscfg::EXTICR1;
use stm32f3xx_hal::pac::{Interrupt, EXTI, SYSCFG};

/// Used to clear the external interrupt pending register for the user button without moving the EXTI peripheral into global static state.
///
/// # Note
/// This does modify hardware register EXTI_PR1.PR0 and should probably only be called from `EXTI0` interrupt context
///
/// # Example
///
/// ```
/// #[interrupt]
/// fn EXTI0() {
///     // If we don't clear the interrupt to signal it's been serviced, it will continue to fire.
///     button::interrupt::clear();
/// }
/// ```
pub fn clear() {
    unsafe {
        let exti = &(*stm32f3xx_hal::pac::EXTI::ptr());
        exti.pr1.write(|w| w.pr0().set_bit())
    }
}

pub enum TriggerMode {
    Rising,
    Falling,
    Both,
}

/// Configures and enables interrupt for the `UserButton` on PA0.
///
/// # Example
///
/// ```
/// let device_periphs = pac::Peripherals::take().unwrap();
/// button::interrupt::enable(&device_periphs.EXTI, &device_periphs.SYSCFG, TriggerMode::Rising);
/// ```
pub fn enable(external_interrupts: &EXTI, sysconfig: &SYSCFG, mode: TriggerMode) {
    // See chapter 14 of the reference manual
    // https://www.st.com/content/ccc/resource/technical/document/reference_manual/4a/19/6e/18/9d/92/43/32/DM00043574.pdf/files/DM00043574.pdf/jcr:content/translations/en.DM00043574.pdf

    configure_exti0(&external_interrupts.imr1);
    map_exti0_to_pa0(&sysconfig.exticr1);

    match mode {
        TriggerMode::Rising => configure_rising_edge_trigger(&external_interrupts.rtsr1),
        TriggerMode::Falling => configure_falling_edge_trigger(&external_interrupts.ftsr1),
        TriggerMode::Both => {
            configure_rising_edge_trigger(&external_interrupts.rtsr1);
            configure_falling_edge_trigger(&external_interrupts.ftsr1);
        }
    }

    enable_exti0();
}

fn configure_exti0(interrupt_mask: &IMR1) {
    interrupt_mask.modify(|_, w| w.mr0().set_bit())
}

fn map_exti0_to_pa0(external_interrupt_config: &EXTICR1) {
    const PORT_A_CONFIG: u8 = 0x000;
    external_interrupt_config.modify(|_, w| unsafe { w.exti0().bits(PORT_A_CONFIG) });
}

fn configure_rising_edge_trigger(rising_trigger_select: &RTSR1) {
    rising_trigger_select.modify(|_, w| w.tr0().set_bit())
}

fn configure_falling_edge_trigger(falling_trigger_select: &FTSR1) {
    falling_trigger_select.modify(|_, w| w.tr0().set_bit())
}

fn enable_exti0() {
    unsafe {
        NVIC::unmask(Interrupt::EXTI0);
    }
}
