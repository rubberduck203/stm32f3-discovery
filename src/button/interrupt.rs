/// Provides interrupt features for `UserButton` on PA0 for the board
use cortex_m::peripheral::NVIC;
use stm32f3xx_hal::stm32::{Interrupt, EXTI, SYSCFG};

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
/// // If we don't clear the interrupt to signal it's been serviced, it will continue to fire.
/// button::interrupt::clear();
/// ```
pub fn clear() {
    const EXTI_PR1: usize = 0x40010414;
    const PR0: usize = (1 << 0);
    unsafe {
        core::ptr::write_volatile(EXTI_PR1 as *mut usize, PR0);
    }
}

/// Configures and enables rising edge interrupt for the `UserButton` on PA0.
///
/// # Example
///
/// ```
/// let device_periphs = stm32::Peripherals::take().unwrap();
/// button::interrupt::enable(&device_periphs.EXTI, &device_periphs.SYSCFG);
/// ```
pub fn enable(external_interrupts: &EXTI, sysconfig: &SYSCFG) {
    // See chapter 14 of the reference manual
    // https://www.st.com/content/ccc/resource/technical/document/reference_manual/4a/19/6e/18/9d/92/43/32/DM00043574.pdf/files/DM00043574.pdf/jcr:content/translations/en.DM00043574.pdf

    // enable exti0
    let interrupt_mask = &external_interrupts.imr1;
    interrupt_mask.modify(|_, w| w.mr0().set_bit());

    //TODO: take enum to specify trigger mode {rising, falling, both}
    // trigger on rising edge
    let rising_trigger_select = &external_interrupts.rtsr1;
    rising_trigger_select.modify(|_, w| w.tr0().set_bit());

    // map line EXTI0 to PA0
    let external_interrupt_config = &sysconfig.exticr1;
    const PORT_A_CONFIG: u8 = 0x000;
    external_interrupt_config.modify(|_, w| unsafe { w.exti0().bits(PORT_A_CONFIG) });

    //enable interrupts on EXTI0
    unsafe {
        NVIC::unmask(Interrupt::EXTI0);
    }
}
