use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

use crate::PostgresSettings;

pub struct DatasourcePool {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl Clone for DatasourcePool {
    fn clone(&self) -> Self {
        DatasourcePool {
            pool: self.pool.clone(),
        }
    }
}

pub fn get_datasource_pool(settings: &PostgresSettings) -> DatasourcePool {
    let url = settings.connection_string();
    let manager = ConnectionManager::<PgConnection>::new(url);

    let pool = Pool::builder()
        .min_idle(Some(1))
        .build(manager)
        .expect("Could not create datasource connection pool");

    DatasourcePool { pool }
}
