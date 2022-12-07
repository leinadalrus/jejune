pub mod shared {
    pub mod data {
        use dotenvy::dotenv;
        use sqlx::postgres::PgPoolOptions;
        use std::env;
        use std::pin::Pin;
        use std::future::Future;
        use std::task::{Context, Poll};
        use std::time::{Duration, Instant};

        pub enum AsyncFnFuture {
          State0,
          State1(tokio::time::MissedTickBehavior),
          Terminated
        }

        pub trait EstablishInterface {
            fn establish_connection() -> Result<(), sqlx::Error>;
        }

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

        impl Future for AsyncFnFuture {
          type Output = ();

          fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
              if !Instant::now() >= self.when {
                cx.waker().wake_by_ref();
                return Poll::Pending;
              }

              return Poll::Ready(());
          }
        }

        impl EstablishInterface for AccountData {
            fn establish_connection() -> Result<(), sqlx::Error> {
                dotenv().ok();

                let database_url = env::var("SUPABASE_PUBLIC_POSTGRESQL_URL")
                    .expect("Database URL must be set in \".env\"");

                let pool = PgPoolOptions::new()
                    .max_connections(1)
                    .connect(&database_url);

                return Ok(());
            }
        }
    } // pub mod data
} // pub mod shared
