use db::conn::get_pool;

#[tokio::main]
fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let config = db::config::Config::new();
    let pool = get_pool(&config).expect("Failed to create pool");

    // just some example code to show how to use the connection pool
    let max_connections = 10;
    let mut connections = Vec::new();
    for _ in 0..max_connections {
        let pool = pool.clone();
        let connection = tokio::spawn(async move {
            let connection = pool.get().expect("Failed to get connection");
            connection
        });
    }
}
