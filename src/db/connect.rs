use std::{env, sync::Arc};

use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub async fn run() -> Arc<Mutex<Client>> {
    let db_uri = env::var("DB_URI").expect("DB_URI not set in .env");

    let builder = SslConnector::builder(SslMethod::tls()).expect("SSL connector failed to build");
    let connector = MakeTlsConnector::new(builder.build());

    let (client, connection) = tokio_postgres::connect(&db_uri, connector)
        .await
        .expect("Failed to connect to postgress");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    Arc::new(Mutex::new(client))
}
