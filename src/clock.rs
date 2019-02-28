//! Board-specific clock configuration

#[cfg(any(feature = "board-hifive1", feature = "board-lofive"))]
use hal::{
    e310x::{PRCI, AONCLK},
    clock::{Clocks, PrciExt, AonExt},
    time::Hertz,
};

#[cfg(feature = "board-hifive1")]
/// Configures clock generation system.
///
/// For HiFive1 board external oscillators are enabled for both high-frequency and low-frequency clocks.
pub fn configure(prci: PRCI, aonclk: AONCLK, target_coreclk: Hertz) -> Clocks {
    let coreclk = prci.constrain();
    let coreclk = coreclk.use_external(Hertz(16_000_000)).coreclk(target_coreclk);

    let aonclk = aonclk.constrain();
    let aonclk = aonclk.use_external(Hertz(32_768));

    Clocks::freeze(coreclk, aonclk)
}

#[cfg(feature = "board-lofive")]
/// Configures clock generation system.
///
/// For LoFive board external oscillator is enabled for high-frequency clock.
/// For low-frequency clock internal oscillator is used.
pub fn configure(prci: PRCI, aonclk: AONCLK, target_coreclk: Hertz) -> Clocks {
    let coreclk = prci.constrain();
    let coreclk = coreclk.use_external(Hertz(16_000_000)).coreclk(target_coreclk);

    let aonclk = aonclk.constrain();
    let aonclk = aonclk.use_external(Hertz(32_768));

    Clocks::freeze(coreclk, aonclk)
}
