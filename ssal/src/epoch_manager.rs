use ssal_core::tokio::{
    spawn,
    time::{sleep as async_sleep, Duration},
};
use ssal_database::{Database, Lock};

pub fn registration_epoch_manager(database: Database) {
    spawn(async move { async_sleep(Duration::from_secs(5)) });
}
