use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub trait EstablishInterface {
    fn establish_connection() -> PgConnection;
}

#[derive(Queryable)]
pub struct AccountData {
    uuid: String,
    first_name: String,
    last_name: String,
    cheque: f32,
    is_transacted: bool,
}

impl AccountData {
    fn new() {
        return Self;
    }
}

impl Default for AccountData {
    fn default() -> Self {
        return Self::new();
    }
}

impl EstablishInterface for AccountData {
    fn establish_connection() -> PgConnection {
        dotenv().ok();

        let database_url = env::var("SUPABASE_PUBLIC_POSTGRESQL_URL")
            .expect("Database URL must be set in \".env\"");

        return PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error on connection establishment with {}", database_url));
    }
}
