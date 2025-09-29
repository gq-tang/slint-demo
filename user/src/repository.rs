use std::path::Path;

use crate::model::User;
use regex::Regex;
use sqlx::{QueryBuilder, migrate::Migrator, sqlite::SqlitePoolOptions};
use std::fs;

pub fn init_dsn(path: &str) -> Result<String, anyhow::Error> {
    let re = Regex::new("(^sqlite://)?(.*)")?;

    let caps = re.captures(path).unwrap();
    if caps.len() != 3 {
        return Err(anyhow::anyhow!("invalid dsn"));
    }
    let pre = match caps.get(1) {
        Some(v) => v.as_str(),
        None => "sqlite://",
    };
    let path = match caps.get(2) {
        Some(v) => v.as_str(),
        None => return Err(anyhow::anyhow!("invalid dsn")),
    };

    if let Some(parent) = Path::new(path).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    if !Path::new(path).exists() {
        fs::File::create(path)?;
    }

    Ok(format!("{}{}", pre, path))
}

pub async fn sqlite_connect(path: &str) -> Result<sqlx::Pool<sqlx::Sqlite>, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(path)
        .await?;

    Ok(pool)
}

static MIGRATOR: Migrator = sqlx::migrate!();
pub async fn migrate_db(pool: &sqlx::Pool<sqlx::Sqlite>) -> Result<(), sqlx::Error> {
    MIGRATOR.run(pool).await?;

    Ok(())
}

pub async fn save_user(pool: &sqlx::Pool<sqlx::Sqlite>, user: User) -> Result<(), sqlx::Error> {
    sqlx::query("insert into users(username,age,gender,email) values(?,?,?,?)")
        .bind(user.username)
        .bind(user.age)
        .bind(user.gender)
        .bind(user.email)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn list_user(
    pool: &sqlx::Pool<sqlx::Sqlite>,
    search: &str,
) -> Result<Vec<User>, sqlx::Error> {
    let mut builder =
        QueryBuilder::new("select id,username,age,gender,email,created_at from users where 1=1");
    if search != "" {
        builder
            .push(" and username like ")
            .push_bind(format!("%{}%", search));
        builder
            .push(" or email like ")
            .push_bind(format!("%{}%", search));
    }
    let query = builder.build_query_as::<User>();
    let users: Vec<User> = query.fetch_all(pool).await?;

    Ok(users)
}
