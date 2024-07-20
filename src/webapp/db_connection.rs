use sqlx::{sqlite::SqlitePool, Row};



pub async fn query_actor(sqlite_pool: &SqlitePool, name: &str) -> Vec<String> {
    let query = sqlx::query(
        r#"SELECT full_name FROM actors WHERE full_name = $1;"#
    )
    .bind(name);

    query
    .fetch_all(sqlite_pool)
    .await
    .unwrap()
    .into_iter()
    .map(|row| row.get("full_name"))
    .collect()

}


pub async fn query_similar_actor(sqlite_pool: &SqlitePool, name: &str) -> Vec<String> {
    let query = sqlx::query(
        r#"SELECT full_name FROM actors WHERE full_name LIKE '%' || $1 || '%' LIMIT 5;"#,
    )
    .bind(name);

    query
    .fetch_all(sqlite_pool)
    .await
    .unwrap()
    .into_iter()
    .map(|row| row.get("full_name"))
    .collect()
}

pub async fn prefix_query_actors(sqlite_pool: &SqlitePool, name: &str) -> Vec<String> {

    let query = sqlx::query(
        r#"SELECT full_name from actors WHERE full_name MATCH '^' || $1 ||  ' *' limit 5;"#,
        ).bind(name);

    query
    .fetch_all(sqlite_pool)
    .await
    .unwrap()
    .into_iter()
    .map(|row| row.get("full_name"))
    .collect()
    }

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{sqlite::SqlitePoolOptions, sqlite::SqliteConnectOptions};

    fn setup_actor_db() -> SqlitePool {
        let cnnection_options = SqliteConnectOptions::new()
            .filename("actors.db");

        let pool = SqlitePoolOptions::new()
            .connect_lazy_with(cnnection_options);
        pool
    }

    #[tokio::test]
    async fn test_get_actor_by_substring() {
        let to_find = "shah rukh khan".to_owned();
        let pool = setup_actor_db();
        let response = query_similar_actor(&pool, "shah rukh").await;
        dbg!(response.clone());
        assert!(response.contains(&to_find));
    }


    #[tokio::test]
    async fn test_get_actor_by_prefix() {
        let to_find = "shah rukh khan".to_owned();
        let pool = setup_actor_db();
        let response = prefix_query_actors(&pool, "shah rukh").await;
        dbg!(response.clone());
        assert!(response.contains(&to_find));
    }
}