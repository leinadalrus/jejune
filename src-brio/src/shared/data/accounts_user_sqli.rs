pub mod shared {
    pub mod data {
        use diesel::pg::PgConnection;
        use diesel::prelude::*;
        use dotenvy::dotenv;
        use std::env;

        pub trait EstablishInterface {
            fn establish_connection() -> PgConnection;
            fn format_display_schema();
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

            fn format_display_schema() {
                let connection = &mut establish_connection();
                let results = accounts
                    .filter(is_transacted.eq(true))
                    .limit(1)
                    .load::<AccountData>(connection)
                    .expect("Error loading posts");

                println!("Displaying {} accounts", results.len());
                for account in results {
                    println!("{}", account.uuid);
                    println!("-----------\n");
                    println!("{}", account.first_name);
                }
            }
        }
    } // pub mod data
} // pub mod shared