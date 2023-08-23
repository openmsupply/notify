use super::StorageConnection;
use crate::repository_error::RepositoryError;
use diesel::{
    associations::HasTable,
    prelude::*,
    query_builder::{
        AsChangeset, DeleteStatement, InsertStatement, IntoUpdateTarget, Query, UpdateStatement,
    },
    query_dsl::methods::{ExecuteDsl, FilterDsl},
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

    pub fn update_one<R>(&self, row: R) -> Result<(), RepositoryError>
    where
        R: AsChangeset<Target = R::Table> + IntoUpdateTarget + Copy,
        UpdateStatement<R::Table, R::WhereClause, R::Changeset>:
            ExecuteDsl<PooledConnection<ConnectionManager<SqliteConnection>>>,
    {
        diesel::update(row)
            .set(row)
            .execute(&self.connection.connection)?;
        Ok(())
    }

    pub fn delete<P>(&self, predicate: P) -> Result<(), RepositoryError>
    where
        T: FilterDsl<P>,
        T::Output: HasTable + IntoUpdateTarget,
        DeleteStatement<<T::Output as HasTable>::Table, P>:
            ExecuteDsl<PooledConnection<ConnectionManager<SqliteConnection>>>,
    {
        diesel::delete(self.table.filter(predicate)).execute(&self.connection.connection)?;
        Ok(())
    }
}
