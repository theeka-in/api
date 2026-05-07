pub fn get_connection_url() -> String {
    let host = std::env::var("DB_HOST").unwrap_or("localhost".to_string());
    let port = std::env::var("DB_PORT").unwrap_or("5432".to_string());
    let user = std::env::var("DB_USER").expect("DB_USER must be set");
    let password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let name = std::env::var("DB_NAME").expect("DB_NAME must be set");

    format!("postgres://{user}:{password}@{host}:{port}/{name}")
}
