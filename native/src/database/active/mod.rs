pub(crate) mod dl_post;

use crate::database::connect_db;
use once_cell::sync::OnceCell;
use sea_orm::DatabaseConnection;
use tokio::sync::Mutex;

pub(crate) static ACTIVE_DATABASE: OnceCell<Mutex<DatabaseConnection>> = OnceCell::new();

pub(crate) async fn init() {
    let db = connect_db("active.db").await;
    ACTIVE_DATABASE.set(Mutex::new(db)).unwrap();
    dl_post::init().await;
}
