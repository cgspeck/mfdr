use clap::Parser;
use diesel::prelude::*;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    sql_query,
    sql_types::Text,
};
use std::time::Duration;
use tokio::runtime::Runtime;

#[derive(QueryableByName)]
struct Subscription {
    #[diesel(sql_type = Text)]
    ref_url: String,
}

#[derive(Parser)]
#[command(name = "mfdr", about = "MyGPO Feed Downloader Runner", long_about = None, version)]
struct Cli {
    #[arg(short, long)]
    daemon: bool,
    #[arg(short, long, value_name = "MINUTES", default_value_t = 480)]
    sleep_interval: u64,
}

fn main() {
    dotenv::dotenv().ok();

    let args = Cli::parse();

    let runtime = Runtime::new().unwrap();

    // Set up a connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool");

    let sleep_duration = Duration::from_secs(args.sleep_interval * 60);

    loop {
        // Run the query and process the results
        let results = runtime.block_on(async {
            let mut conn = pool.get().expect("Failed to get connection from pool");

            let query_result: Vec<Subscription> = sql_query(
                "SELECT DISTINCT ref_url FROM subscriptions_subscription ORDER BY ref_url ASC",
            )
            .load(&mut conn)
            .expect("Error executing query");

            query_result
        });

        // Process each result and run the shell command
        for subscription in results {
            let command = format!("python manage.py feed-downloader {}", subscription.ref_url);
            std::process::Command::new("sh")
                .arg("-c")
                .arg(&command)
                .status()
                .expect("Failed to execute command");
        }

        if args.daemon {
            println!("Sleeping for {:#?}", sleep_duration);
            std::thread::sleep(sleep_duration);
        } else {
            break;
        }
    }
}
