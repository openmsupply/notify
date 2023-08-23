use super::StorageConnection;
use crate::repository_error::RepositoryError;
use diesel::{
    prelude::*,
    query_builder::InsertStatement,
    query_dsl::methods::ExecuteDsl,
    r2d2::{ConnectionManager, PooledConnection},
};

pub struct RowRepository<'a, T>
where
    T: Table + 'static + Copy,
{
    connection: &'a StorageConnection,
    table: T,
}

impl<'a, 'b, T> RowRepository<'a, T>
where
    T: Table + 'static + Copy,
{
    pub fn new(connection: &'a StorageConnection, table: T) -> Self {
        RowRepository { connection, table }
    }

    pub fn insert_one<R>(&self, row: R) -> Result<(), RepositoryError>
    where
        R: Insertable<T>,
        InsertStatement<T, R::Values>:
            ExecuteDsl<PooledConnection<ConnectionManager<SqliteConnection>>>,
    {
        diesel::insert_into(self.table)
            .values(row)
            .execute(&self.connection.connection)?;
        Ok(())
    }
}
