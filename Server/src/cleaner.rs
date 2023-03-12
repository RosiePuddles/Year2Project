use deadpool_postgres::{GenericClient, Pool};

use crate::{logger::Logger};

macro_rules! log_wrap {
	($e:expr) => {
		if let Err(e) = $e {
			println!("{}:{} Error writing to log {:?}", file!(), line!(), e)
		}
	};
}

const WAIT_SECS: u64 = 300;

pub async fn main(pool: Pool, logger: Logger<'_>) {
	loop {
		// db clean
		match pool.get().await {
			Ok(client) => {
				let stmt = client.prepare(include_str!("../sql/clean.sql")).await.unwrap();
				if let Err(e) = client.query(&stmt, &[]).await {
					log_wrap!(logger.clean(format!("{}:{} Statement query error {:?}", file!(), line!(), e)))
				} else {
					log_wrap!(logger.clean(format!("{}:{} DB clean successful", file!(), line!())));
				}
			}
			Err(e) => {
				log_wrap!(logger.clean(format!("{}:{} Open client error {:?}", file!(), line!(), e)));
			}
		};
		log_wrap!(logger.clean(format!("{}:{} Clean wait for {} secs", file!(), line!(), WAIT_SECS)));
		std::thread::sleep(std::time::Duration::new(WAIT_SECS, 0));
	}
}
