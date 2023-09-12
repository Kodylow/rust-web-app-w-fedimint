use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::info;

const PG_DEV_POSTGRES_URL: &str =
	"postgres://postgres:postgres@localhost:5432/postgres";
const PG_DEV_APP_URL: &str = "postgres://app_user:app_pwd@localhost:5432/app_db";

// sql files
const SQL_RECREATE_DB: &str = "sql/dev_initial/00-recreate_db.sql";
const SQL_DIR: &str = "sql/dev_initial/01-dir.sql";

async fn pexec(pool: &PgPool, sql: &str) -> Result<(), sqlx::Error> {
	info!("{:<12} - pexec: {file}", "FOR-DEV-ONLY");

	let content = std::fs::read_to_string(sql)?;
	let sqls = content.split(";").collect::<Vec<&str>>();

	for sql in sqls {
		if sql.trim().is_empty() {
			continue;
		}
		sqlx::query(sql).execute(pool).await?;
	}

	Ok(())
}

async fn new_db(db_con_url: &str) -> Result<PgPool, sqlx::Error> {
	PgPoolOptions::new()
		.max_connections(1)
		.acquire_timeout(Duration::from_millis(500))
		.connect(db_con_url)
		.await
}
