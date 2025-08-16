# Feytra: Cursor for Databases

## What We're Building

**Feytra** - an intelligent CLI assistant that sits next to your database and provides natural language Q&A about your schema, relationships, and data structure.

## Core Vision

Users will:
1. **Install Feytra on their VMs/servers** next to their databases
2. **Run the app** - it automatically detects schema changes via DDL hashing
3. **Ask questions naturally** - "What tables do I have?", "Show me relationships between users and orders", "Find all authentication-related tables"

## Core Architecture (3 Pillars)

### 1. Schema Intelligence Engine
- **Automatic DDL Detection**: Compares current schema hashes with stored hashes
- **Smart Vectorization**: Converts DDL to embeddings using OpenAI's text-embedding-3-small
- **HNSW Vector Search**: Fast similarity search for relevant schema objects

### 2. AI-Powered Q&A
- **OpenAI Integration**: Uses GPT-4o-mini for intelligent natural language responses
- **Context-Aware**: Provides relevant schema objects as context to LLM
- **Semantic Understanding**: Understands intent beyond exact keyword matching

### 3. Sidecar Deployment
- **Non-Intrusive**: Runs alongside your database, not as a proxy
- **Zero Configuration**: Auto-detects database connection from environment
- **Hash-Based Updates**: Only re-vectorizes when schema actually changes

## v0.1 Goals ✅ COMPLETED
- [x] CLI interface with natural language Q&A
- [x] Automatic schema detection and vectorization
- [x] OpenAI embedding and LLM integration
- [x] Hash-based schema freshness checking
- [x] Interactive chat mode
- [x] PostgreSQL catalog provider
- [x] HNSW vector search
- [x] SQLite metadata storage

## Non-goals
- ❌ REST API endpoints
- ❌ Swagger/OpenAPI documentation
- ❌ Web UI
- ❌ Database proxy functionality
- ❌ Data querying (only schema Q&A)

## Architecture (crates)

### `feytra-core` (Main Engine)
- **catalog/**: Database schema snapshotting (PostgreSQL)
- **fingerprint/**: DDL canonicalization and hashing
- **embed/**: OpenAI embedding generation
- **vectors/**: HNSW vector search index
- **repo/**: SQLite metadata persistence
- **jobs/**: Bootstrap and refresh operations

### `feytra-api` (Optional)
- REST API for web integration (future)

### `feytra-bin/sidecar` (CLI)
- Interactive chat interface
- Command-line operations

## DDL Vectorization Process

1. **Schema Snapshot**: `PgCatalogProvider` captures all tables, views, functions, indexes
2. **DDL Generation**: Converts schema objects to canonical DDL strings
3. **Hash Computation**: Generates SHA256 hashes for change detection
4. **Embedding Creation**: Uses OpenAI text-embedding-3-small to vectorize DDL
5. **Vector Storage**: Stores in HNSW index for fast similarity search
6. **Metadata Persistence**: Saves in SQLite for persistence across restarts

## AI Q&A Flow

1. **User Question**: "What tables do I have?"
2. **Question Embedding**: Convert question to vector using OpenAI
3. **Vector Search**: Find top 5 most relevant schema objects
4. **Context Building**: Gather detailed DDL for relevant objects
5. **LLM Query**: Send question + context to GPT-4o-mini
6. **Response Generation**: AI provides natural language answer
7. **Result Display**: Show AI response + relevant schema objects

## CLI Interface

### Primary Commands
- `feytra chat` - Interactive Q&A mode (main usage)
- `feytra init` - Manual initialization
- `feytra refresh` - Force schema refresh
- `feytra serve` - API server (optional)

### Chat Commands
- Natural language questions about schema
- `help` - Show available commands
- `tables` - List all tables
- `schema` - Show schema overview
- `refresh` - Update schema data
- `exit` - End session

## Example Usage

```bash
# Install and run
cargo install feytra
feytra chat

# Interactive session
💭 What tables do I have?
�� You have 5 tables: users, products, categories, orders, order_items...

💭 Show me the relationships between users and orders
🤖 Users and orders are related through a foreign key relationship...

💭 Find all authentication-related tables
🤖 Based on your schema, I found these authentication-related objects...
```

## Deployment Options

### 1. VM/Server Deployment (Primary)
- Install on same machine as database
- Configure DATABASE_URL to point to local DB
- Run as systemd service or Docker container
- Zero network latency, full schema access

### 2. Docker Deployment
- Sidecar container alongside database
- Shared network namespace
- Easy scaling and management

### 3. Local Development
- Direct installation for development
- Real-time schema changes
- Immediate feedback loop

## Key Design Choices

### Why CLI-First?
- **Developer-Friendly**: Developers love CLI tools
- **Automation-Ready**: Easy to script and integrate
- **Low Overhead**: No web server, no browser dependencies
- **Cursor Parallel**: Like Cursor's CLI-first approach

### Why Sidecar Architecture?
- **Non-Intrusive**: Doesn't interfere with database operations
- **Zero Risk**: Can't break existing applications
- **Easy Deployment**: Just run alongside existing infrastructure
- **Scalable**: Can run multiple instances if needed

### Why Hash-Based Updates?
- **Efficient**: Only re-process when schema actually changes
- **Fast Startup**: No unnecessary re-vectorization
- **Reliable**: Deterministic change detection
- **Incremental**: Can handle large schema changes efficiently

## Success Criteria v0.1

- [x] **Zero-Config Setup**: Works with just DATABASE_URL
- [x] **Automatic Detection**: Finds schema changes without manual intervention
- [x] **Intelligent Q&A**: Provides helpful answers to schema questions
- [x] **Fast Response**: Sub-second response times for most queries
- [x] **Developer Experience**: Intuitive CLI interface

## Roadmap

### v0.2: Enhanced Intelligence
- Multi-database support (MySQL, SQLite)
- Relationship visualization
- Query suggestion based on schema
- Performance optimization

### v0.3: Advanced Features
- Schema change notifications
- Migration impact analysis
- Data lineage tracking
- Custom embedding models

### v0.4: Enterprise Features
- Team collaboration
- Schema documentation generation
- Compliance reporting
- Integration APIs

## Target Users

### Primary: Database Developers
- Need to understand complex schemas quickly
- Want natural language interface to database metadata
- Value developer productivity tools

### Secondary: DevOps Engineers
- Manage multiple database environments
- Need schema change monitoring
- Want automation-friendly tools

### Tertiary: Data Analysts
- Explore unfamiliar databases
- Understand data relationships
- Generate reports based on schema

## Value Proposition

### For Developers
- **10x Faster Schema Understanding**: Natural language Q&A vs manual exploration
- **Zero Learning Curve**: Ask questions in plain English
- **Always Up-to-Date**: Automatic schema change detection
- **Cursor-Like Experience**: Familiar CLI-first approach

### For Teams
- **Knowledge Sharing**: Consistent schema understanding across team
- **Onboarding Acceleration**: New developers can understand schemas instantly
- **Documentation**: Automatic schema documentation generation
- **Compliance**: Track schema changes and their impact

### For Organizations
- **Developer Productivity**: Reduce time spent on schema exploration
- **Risk Reduction**: Better understanding of data relationships
- **Cost Savings**: Fewer schema-related bugs and issues
- **Scalability**: Handle complex, evolving schemas efficiently
