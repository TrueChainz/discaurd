use mysql::PooledConn;
use redis::Connection;
use std::env;

pub fn sql_connect() -> PooledConn {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let builder = mysql::OptsBuilder::from_opts(mysql::Opts::from_url(&url).unwrap());
    let pool = mysql::Pool::new(builder.ssl_opts(mysql::SslOpts::default())).unwrap();
    let conn = pool
        .get_conn()
        .unwrap_or_else(|_| panic!("Failed to connect database: {}", url));
    return conn;
}

pub fn redis_connect() -> Connection {
    let url = env::var("REDIS_URL").expect("REDIS_URL not found");
    let client = redis::Client::open(url.as_str()).unwrap();
    let conn = client
        .get_connection()
        .unwrap_or_else(|_| panic!("Failed to connect database: {}", url));
    return conn;
}
