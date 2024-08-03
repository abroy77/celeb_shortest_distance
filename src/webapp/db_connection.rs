use crate::data::Actor;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, sqlite::SqlitePool};

#[derive(Debug, Deserialize, Serialize, FromRow)]
struct DbActor {
    id: i64,
    full_name: String,
    birth_year: Option<u32>,
}

impl From<DbActor> for Actor {
    fn from(db_actor: DbActor) -> Self {
        Actor {
            id: db_actor.id as usize,
            full_name: db_actor.full_name,
            birth_year: db_actor.birth_year,
        }
    }
}

pub async fn query_actor(sqlite_pool: &SqlitePool, name: &str) -> Vec<Actor> {
    let query =
        sqlx::query_as::<_, DbActor>(r#"SELECT full_name FROM actors WHERE full_name = $1;"#)
            .bind(name);

    query
        .fetch_all(sqlite_pool)
        .await
        .unwrap()
        .into_iter()
        .map(Actor::from)
        .collect()
}

pub async fn query_similar_actor(sqlite_pool: &SqlitePool, name: &str) -> Vec<Actor> {
    let query = sqlx::query_as::<_, DbActor>(
        r#"SELECT id, full_name, birth_year FROM actors WHERE full_name LIKE '%' || $1 || '%' LIMIT 5;"#,
    )
    .bind(name);

    query
        .fetch_all(sqlite_pool)
        .await
        .unwrap()
        .into_iter()
        .map(Actor::from)
        .collect()
}

pub async fn prefix_query_actors(sqlite_pool: &SqlitePool, name: &str) -> Vec<Actor> {
    let query = sqlx::query_as::<_, DbActor>(
        r#"SELECT id, full_name, birth_year FROM actors WHERE full_name MATCH '^' || $1 ||  ' *' LIMIT 5;"#,
    )
    .bind(name);

    query
        .fetch_all(sqlite_pool)
        .await
        .unwrap()
        .into_iter()
        .map(Actor::from)
        .collect()
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{sqlite::SqliteConnectOptions, sqlite::SqlitePoolOptions};

    fn setup_actor_db() -> SqlitePool {
        let cnnection_options = SqliteConnectOptions::new().filename("actors.db");

        SqlitePoolOptions::new().connect_lazy_with(cnnection_options)
    }
    fn srk() -> Actor {
        Actor {
            full_name: "Shah Rukh Khan".to_string(),
            id: 451321,
            birth_year: Some(1965),
        }
    }

    #[tokio::test]
    async fn test_get_actor_by_substring() {
        let srk = srk();
        let pool = setup_actor_db();
        let response = query_similar_actor(&pool, "shah rukh").await;
        assert!(response.contains(&srk));
    }

    #[tokio::test]
    async fn test_get_actor_by_prefix() {
        let srk = srk();
        let pool = setup_actor_db();
        let response = prefix_query_actors(&pool, "shah rukh").await;
        assert!(response.contains(&srk));
    }
}
