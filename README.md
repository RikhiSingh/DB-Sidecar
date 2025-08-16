# Feytra: Cursor for Databases

> **"Cursor but for databases"** - An intelligent CLI assistant that sits next to your database and provides natural language Q&A about your schema, relationships, and data structure.

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 🎯 What is Feytra?

Feytra is an AI-powered database schema intelligence tool that runs as a sidecar next to your database. Think of it as "Cursor but for databases" - you can ask natural language questions about your schema and get intelligent, contextual answers.

### Key Features

- 🤖 **AI-Powered Q&A**: Ask questions like "What tables do I have?" or "Show me relationships between users and orders"
- 🔄 **Automatic Schema Detection**: Automatically detects schema changes via DDL hashing
- ⚡ **Zero Configuration**: Works with just your `DATABASE_URL`
- 🚀 **Sidecar Deployment**: Runs alongside your database, not as a proxy
- 🔍 **Vector Search**: Uses OpenAI embeddings for semantic schema understanding

## 🚀 Quick Start

### 1. Install Feytra

```bash
# Clone the repository
git clone https://github.com/your-username/feytra.git
cd feytra

# Build the project
cargo build --release
```

### 2. Configure Your Environment

Create a `.env` file in your project root:

```env
# Database Configuration
DATABASE_URL=postgresql://username:password@localhost:5432/your_database

# AI Configuration (Optional - for enhanced responses)
OPENAI_API_KEY=your_openai_api_key
LLM_PROVIDER=openai
OPENAI_MODEL=gpt-4o-mini
OPENAI_EMBED_MODEL=text-embedding-3-small

# Feytra Configuration
SIDECAR_DB=./data/sidecar.sqlite
EMBED_PROVIDER=openai
EMBED_DIM=768
VECTOR_BACKEND=hnsw
RUST_LOG=info
```

### 3. Start Chatting

```bash
# Start interactive chat mode
cargo run -p sidecar -- chat
```

Feytra will automatically:

- Connect to your database
- Detect if schema intelligence exists
- Initialize embeddings if needed
- Start the interactive Q&A session

### 4. Ask Questions

```
💭 What tables do I have?
🤖 You have 5 tables: users, products, categories, orders, order_items...

💭 Show me the relationships between users and orders
🤖 Users and orders are related through a foreign key relationship...

💭 Find all authentication-related tables
🤖 Based on your schema, I found these authentication-related objects...
```

## 📋 CLI Commands

### Primary Commands

- `feytra chat` - Interactive Q&A mode (main usage)
- `feytra init` - Manual schema initialization
- `feytra refresh` - Force schema refresh
- `feytra serve` - Start API server (optional)

### Chat Commands

- **Natural language questions** about your schema
- `help` - Show available commands
- `tables` - List all tables
- `schema` - Show schema overview
- `refresh` - Update schema data
- `exit` - End session

## 🔄 Automatic Schema Management

Feytra automatically manages your schema intelligence:

1. **Startup Check**: Compares current DDL hashes with stored hashes
2. **Change Detection**: If hashes don't match, automatically re-vectorizes
3. **Smart Updates**: Only processes changed schema objects
4. **Persistent Storage**: Maintains intelligence across restarts

## 🏗️ Architecture

### Core Components

- **Schema Intelligence Engine**: Automatic DDL detection and vectorization
- **AI-Powered Q&A**: OpenAI integration for natural language responses
- **Sidecar Deployment**: Non-intrusive, runs alongside your database

### Technology Stack

- **Rust**: High-performance, memory-safe language
- **PostgreSQL**: Primary database support
- **OpenAI**: Embedding generation and LLM responses
- **HNSW**: Fast vector similarity search
- **SQLite**: Local metadata storage

## 🎯 Use Cases

### For Developers

- **Schema Exploration**: Quickly understand unfamiliar databases
- **Relationship Discovery**: Find connections between tables
- **Documentation**: Get natural language explanations of schema objects

### For DevOps

- **Schema Monitoring**: Track changes across environments
- **Deployment Validation**: Ensure schema consistency
- **Troubleshooting**: Understand database structure quickly

### For Data Analysts

- **Data Discovery**: Explore database structure
- **Query Planning**: Understand available data sources
- **Report Generation**: Identify relevant tables and relationships

## 🔧 Configuration

### Environment Variables

| Variable             | Description                             | Default                |
| -------------------- | --------------------------------------- | ---------------------- |
| `DATABASE_URL`       | PostgreSQL connection string            | Required               |
| `OPENAI_API_KEY`     | OpenAI API key for AI features          | Optional               |
| `LLM_PROVIDER`       | LLM provider (openai)                   | None                   |
| `OPENAI_MODEL`       | OpenAI model for responses              | gpt-4o-mini            |
| `OPENAI_EMBED_MODEL` | OpenAI embedding model                  | text-embedding-3-small |
| `SIDECAR_DB`         | SQLite metadata database path           | ./data/sidecar.sqlite  |
| `EMBED_PROVIDER`     | Embedding provider (openai, local_stub) | local_stub             |
| `EMBED_DIM`          | Embedding dimensions                    | 768                    |
| `VECTOR_BACKEND`     | Vector search backend                   | hnsw                   |
| `RUST_LOG`           | Logging level                           | info                   |

## 🚀 Deployment

### VM/Server Deployment (Recommended)

1. **Install on the same machine as your database**
2. **Configure environment variables**
3. **Run as a service**:

```bash
# Create systemd service
sudo cp feytra.service /etc/systemd/system/
sudo systemctl enable feytra
sudo systemctl start feytra
```

### Docker Deployment

```bash
# Build and run
docker build -t feytra .
docker run -d --name feytra \
  -e DATABASE_URL=postgresql://user:pass@host:5432/db \
  -v ./data:/app/data \
  feytra
```

### Local Development

```bash
# Run directly
cargo run -p sidecar -- chat

# Or build and run
cargo build --release
./target/release/sidecar chat
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone and setup
git clone https://github.com/your-username/feytra.git
cd feytra

# Install dependencies
cargo build

# Run tests
cargo test

# Start development
cargo run -p sidecar -- chat
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🗺️ Roadmap

See [ROADMAP.md](ROADMAP.md) for our development roadmap and future plans.

## 💡 Why "Cursor for Databases"?

Just like Cursor revolutionized code editing with AI, Feytra aims to revolutionize database schema understanding with AI. The goal is to make databases as easy to understand and work with as code is with Cursor.

## 🆘 Support

- **Issues**: [GitHub Issues](https://github.com/RikhiSingh/feytra/issues)

---

**Made with ❤️ for developers who love databases**
