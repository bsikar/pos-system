use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{fs, path::PathBuf, time::Duration};

// postgress
const PG_HOST: &str = "0.0.0.0";
const _PG_ROOT_DB: &str = "postgres";
const _PG_ROOT_USER: &str = "postgres";
const _PG_ROOT_PWD: &str = "postgres";

// app database
const PG_USER: &str = "pos_user";
const PG_PWD: &str = "pos_password";
const PG_DB: &str = "pos_db";
const PG_MAX_CON: u32 = 5;

// sql files
const SQL_DIR: &str = "sql/";
const SQL_RECREATE: &str = "sql/00-recreate-db.sql";

pub type Db = Pool<Postgres>;

pub async fn init_db() -> Result<Db, sqlx::Error> {
    // create the database with the root user (for development only)
    //{
    //    let root_db = new_db_pool(PG_HOST, _PG_ROOT_DB, _PG_ROOT_USER, _PG_ROOT_PWD, 1).await?;
    //    pexec(&root_db, SQL_RECREATE).await?;
    //}

    // run the app sql files
    let app_db = new_db_pool(PG_HOST, PG_DB, PG_USER, PG_PWD, 1).await?;
    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .into_iter()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    paths.sort();

    // execute the sql files
    for path in paths {
        if let Some(path) = path.to_str() {
            // only execute sql files and not the recreate
            if path.ends_with(".sql") && path != SQL_RECREATE {
                pexec(&app_db, path).await?;
            }
        }
    }

    // returning the app db
    new_db_pool(PG_HOST, PG_DB, PG_USER, PG_PWD, PG_MAX_CON).await
}

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    // read the file
    let content = fs::read_to_string(file).map_err(|e| {
        eprintln!("ERROR reading {} (cause: {:?})", file, e);
        e
    })?;

    let sqls = content
        .split(';')
        .filter(|s| !s.trim().is_empty())
        .collect::<Vec<_>>();

    for sql in sqls {
        match sqlx::query(sql).execute(db).await {
            Ok(_) => (),
            Err(e) => eprintln!("WARNING - pexec - sql file '{}' FAILED (cause: {})", sql, e),
        }
    }

    Ok(())
}

async fn new_db_pool(
    host: &str,
    db: &str,
    user: &str,
    pwd: &str,
    max_con: u32,
) -> Result<Db, sqlx::Error> {
    let con_str = format!("postgres://{}:{}@{}/{}", user, pwd, host, db);

    PgPoolOptions::new()
        .max_connections(max_con)
        .connect_timeout(Duration::from_millis(500))
        .connect(&con_str)
        .await
}

#[cfg(test)]
#[path = "../../../tests/model_tests/db.rs"]
mod model_tests;
