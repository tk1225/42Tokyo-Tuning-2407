use sqlx::mysql::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;
use std::time::Duration;
use std::env;

pub async fn create_pool() -> MySqlPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // MySqlPool::connect(&database_url)
    //     .await
    //     .expect("Failed to create pool")

    MySqlPoolOptions::new()
        .max_connections(100) // 最大接続数を増やす
        .connect_timeout(Duration::from_secs(30)) // 接続タイムアウトを設定する
        .idle_timeout(Some(Duration::from_secs(600))) // 接続のアイドル時間を設定する
        .max_lifetime(Some(Duration::from_secs(1800))) // 接続の最大ライフタイムを設定する
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}
