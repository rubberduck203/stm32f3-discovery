#[cfg(feature = "revD")]
pub mod rev_d;
#[cfg(feature = "revD")]
pub use self::rev_d::Compass;

#[cfg(feature = "revE")]
pub mod rev_e;
#[cfg(feature = "revE")]
pub use self::rev_e::Compass;