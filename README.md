# Project Knowledge Graph

A local graph database for storing interconnected project information using Neo4j and Rust.

## Overview

Project Knowledge Graph is a tool for developers and project managers to maintain a local knowledge base of project-related information and their relationships. It allows you to store and query connections between:

- Projects
- GitHub repositories
- Local file paths
- Working branches
- Pull requests
- Jenkins CI/CD pipelines
- GitHub Actions
- Jira tickets
- Kibana dashboards
- Databricks notebooks
- And more!

## Features

- **Relationship-focused**: Store and query complex relationships between project components
- **Local-first**: Keep your knowledge base on your local machine
- **Fast and efficient**: Built with Rust for performance and reliability
- **CLI interface**: Easily interact with your knowledge graph from the command line
- **Extensible**: Add new node types and relationships as needed
- **Docker support**: Run Neo4j in a container for easy setup

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (1.70.0+)
- [Docker](https://docs.docker.com/get-docker/) and Docker Compose (for running Neo4j)
- [Neo4j](https://neo4j.com/download/) (4.4+) - optional if using Docker

## Quick Start

### 1. Clone the repository

```bash
git clone https://github.com/yourusername/project-knowledge-graph.git
cd project-knowledge-graph
```

### 2. Start Neo4j using Docker

```bash
docker-compose up -d
```

This will start Neo4j on `localhost:7687` (Bolt protocol) and `localhost:7474` (HTTP for Neo4j Browser).

### 3. Build the CLI tool

```bash
cargo build --release
```

### 4. Set up environment (optional)

```bash
export NEO4J_URI=bolt://localhost:7687
export NEO4J_USER=neo4j
export NEO4J_PASSWORD=password
```

### 5. Initialize the database

```bash
./target/release/pkg init
```

### 6. Use the CLI

Add a project:
```bash
./target/release/pkg project add "DataPipeline" --desc "Data processing pipeline"
```

Add a repository:
```bash
./target/release/pkg repo add "https://github.com/company/data-pipeline" --path "/home/user/projects/data-pipeline" --project "DataPipeline"
```

View project info:
```bash
./target/release/pkg project info "DataPipeline"
```

## CLI Reference

### Projects

```bash
# Add a project
pkg project add "ProjectName" --desc "Project description"

# List all projects
pkg project list

# Get project info
pkg project info "ProjectName"
```

### Repositories

```bash
# Add a repository
pkg repo add "https://github.com/org/repo" --path "/local/path" --project "ProjectName"

# List repositories
pkg repo list
```

### Branches

```bash
# Add a branch
pkg branch add "https://github.com/org/repo" "branch-name" --working

# List branches for a repository
pkg branch list "https://github.com/org/repo"
```

### Tickets

```bash
# Add a ticket
pkg ticket add "JIRA-123" "Implement feature X" --status "in-progress" --project "ProjectName"

# List tickets
pkg ticket list
```

See `pkg --help` for more commands and options.

## Docker Setup

The included Docker Compose file sets up:

- Neo4j database with proper configuration
- Persistent volume for Neo4j data
- Exposed ports for Neo4j Bolt and Browser interfaces

## Development

### Running tests

```bash
cargo test
```

### Building for development

```bash
cargo build
```

### Linting

```bash
cargo clippy
```

### Generating documentation

```bash
cargo doc --open
```

## Project Structure

- `src/` - Rust source code
  - `main.rs` - Application entry point
  - `db/` - Neo4j database interaction
  - `models/` - Domain models
  - `cli/` - Command-line interface
  - `utils/` - Utility functions
- `tests/` - Integration tests
- `docker/` - Docker configuration
- `migrations/` - Database migrations and setup scripts

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
