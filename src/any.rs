pub use sqlx_core::any::*;

pub use sqlx_core::any::driver::install_drivers;

pub fn install_default_drivers() {
    install_drivers(&[
        #[cfg(feature = "mysql")]
        sqlx_mysql::any::DRIVER,
    ])
}
