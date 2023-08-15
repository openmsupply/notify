use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

use crate::PostgresSettings;

pub fn get_datasource_pool(settings: &PostgresSettings) -> Pool<ConnectionManager<PgConnection>> {
    let url = settings.connection_string();
    let manager = ConnectionManager::<PgConnection>::new(url);

    Pool::builder()
        .min_idle(Some(1))
        .build(manager)
        .expect("Could not create datasource connection pool")
}
