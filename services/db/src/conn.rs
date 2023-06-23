// This is a module for managing connections to the database
// It is used by the other modules in the services/db/src directory
// to create connections to the database. It uses the diesel crate
// to manage the connections. The diesel crate uses the connection
// pool pattern to manage connections to the database.

use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection, PoolError};

use crate::config::Config;

/// This function creates a connection pool to the database
/// It uses the configuration information to create the connection
/// pool. The connection pool is created using the connection manager
/// which is created using the connection string.
pub fn get_pool(config: &Config) -> Result<Pool<ConnectionManager<PgConnection>>, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(config.database_url());
    Pool::builder().build(manager)
}

/// This function returns a connection from the connection pool
/// It is used to get a connection from the connection pool
/// so that it can be used to query the database.
pub fn get_connection(
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, PoolError> {
    pool.get()
}