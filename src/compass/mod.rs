#[cfg(not(feature = "revE"))]
pub mod rev_d;
#[cfg(not(feature = "revE"))]
pub use self::rev_d::Compass;

#[cfg(feature = "revE")]
pub mod rev_e;
#[cfg(feature = "revE")]
pub use self::rev_e::Compass;