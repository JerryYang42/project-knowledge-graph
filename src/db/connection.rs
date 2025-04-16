use anyhow::{Context, Result};
use neo4rs::{Graph, GraphConfig};
use std::env;

/// Get a Neo4j connection using environment variables
pub async fn get_connection() -> Result<Graph> {
    let uri = env::var("NEO4J_URI").unwrap_or_else(|_| "bolt://localhost:7687".to_string());
    let user = env::var("NEO4J_USER").unwrap_or_else(|_| "neo4j".to_string());
    let password = env::var("NEO4J_PASSWORD").unwrap_or_else(|_| "password".to_string());

    let config = GraphConfig::new()
        .uri(&uri)
        .user(&user)
        .password(&password);

    Graph::connect(config)
        .await
        .with_context(|| format!("Failed to connect to Neo4j at {}", uri))
}

/// Verify that the Neo4j connection is working
pub async fn verify_connection() -> Result<bool> {
    let graph = get_connection().await?;
    let mut result = graph.execute("RETURN 1 as test").await?;
    
    if let Some(row) = result.next().await? {
        let value: i64 = row.get("test")?;
        Ok(value == 1)
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    
    // Note: These tests require a running Neo4j instance
    
    #[tokio::test]
    #[serial]
    async fn test_connection() {
        if let Ok(graph) = get_connection().await {
            assert!(true, "Connection successful");
            
            // Test a simple query
            let mut result = graph.execute("RETURN 1 + 1 as sum").await.unwrap();
            let row = result.next().await.unwrap().unwrap();
            let sum: i64 = row.get("sum").unwrap();
            assert_eq!(sum, 2);
        } else {
            // Skip test if no database connection is available
            println!("Skipping test_connection: No database connection available");
        }
    }
    
    #[tokio::test]
    #[serial]
    async fn test_verify_connection() {
        // Only run test if NEO4J_URI is set
        if std::env::var("NEO4J_URI").is_ok() {
            let result = verify_connection().await;
            assert!(result.is_ok(), "Connection verification should succeed");
            assert!(result.unwrap(), "Connection should be verified");
        } else {
            // Skip test if no database connection is available
            println!("Skipping test_verify_connection: No NEO4J_URI environment variable set");
        }
    }
}
