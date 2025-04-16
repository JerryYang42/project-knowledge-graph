use anyhow::Result;
use neo4rs::Graph;
use tracing::{info, warn};

/// Initialize the database with required constraints and indexes
pub async fn initialize_database(graph: &Graph) -> Result<()> {
    info!("Initializing database schema");
    
    // Create constraints for nodes to ensure uniqueness
    let constraints = vec![
        // Project constraint
        "CREATE CONSTRAINT project_name_unique IF NOT EXISTS FOR (p:Project) REQUIRE p.name IS UNIQUE",
        
        // Repository constraint
        "CREATE CONSTRAINT repository_url_unique IF NOT EXISTS FOR (r:Repository) REQUIRE r.url IS UNIQUE",
        
        // Branch constraint (unique per repository)
        "CREATE CONSTRAINT branch_name_repo_unique IF NOT EXISTS FOR (b:Branch) REQUIRE (b.name, b.repository_id) IS UNIQUE",
        
        // Ticket constraint
        "CREATE CONSTRAINT ticket_id_unique IF NOT EXISTS FOR (t:Ticket) REQUIRE t.ticket_id IS UNIQUE",
        
        // Dashboard constraint
        "CREATE CONSTRAINT dashboard_url_unique IF NOT EXISTS FOR (d:Dashboard) REQUIRE d.url IS UNIQUE",
        
        // Notebook constraint
        "CREATE CONSTRAINT notebook_url_unique IF NOT EXISTS FOR (n:Notebook) REQUIRE n.url IS UNIQUE",
        
        // Pipeline constraint
        "CREATE CONSTRAINT pipeline_name_unique IF NOT EXISTS FOR (c:CIPipeline) REQUIRE (c.name, c.url) IS UNIQUE",
    ];
    
    // Create indexes for better query performance
    let indexes = vec![
        // Project indexes
        "CREATE INDEX project_created_at IF NOT EXISTS FOR (p:Project) ON (p.created_at)",
        
        // Repository indexes
        "CREATE INDEX repository_local_path IF NOT EXISTS FOR (r:Repository) ON (r.local_path)",
        
        // Branch indexes
        "CREATE INDEX branch_is_working IF NOT EXISTS FOR (b:Branch) ON (b.is_working_branch)",
        
        // PR indexes
        "CREATE INDEX pr_status IF NOT EXISTS FOR (pr:PullRequest) ON (pr.status)",
        "CREATE INDEX pr_number IF NOT EXISTS FOR (pr:PullRequest) ON (pr.number)",
        
        // Ticket indexes
        "CREATE INDEX ticket_status IF NOT EXISTS FOR (t:Ticket) ON (t.status)",
        "CREATE INDEX ticket_type IF NOT EXISTS FOR (t:Ticket) ON (t.ticket_type)",
    ];
    
    // Execute constraints
    for constraint in constraints {
        match graph.execute(constraint).await {
            Ok(_) => info!("Applied constraint: {}", constraint),
            Err(e) => warn!("Failed to apply constraint: {} - Error: {:?}", constraint, e),
        }
    }
    
    // Execute indexes
    for index in indexes {
        match graph.execute(index).await {
            Ok(_) => info!("Created index: {}", index),
            Err(e) => warn!("Failed to create index: {} - Error: {:?}", index, e),
        }
    }
    
    info!("Database schema initialization completed");
    Ok(())
}

/// Drop all constraints