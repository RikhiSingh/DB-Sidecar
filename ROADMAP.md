# Feytra Roadmap

## Vision

Feytra is an AI-powered database schema intelligence sidecar that works like "Cursor for databases" - providing natural language understanding of database schemas through vectorized DDL and semantic search.

## v0.1 (Current) - Foundation & CLI

### Core Infrastructure ✅

- [x] PostgreSQL sidecar deployment model
- [x] DDL capture and canonicalization
- [x] Vector embeddings generation (local_stub provider)
- [x] HNSW vector search for semantic similarity
- [x] SQLite metadata storage with migrations
- [x] Change detection via polling
- [x] Basic schema statistics and sampling

### CLI Interface ✅

- [x] Interactive chat mode (`feytra chat`)
- [x] Schema initialization (`feytra init`)
- [x] Schema refresh (`feytra refresh`)
- [x] Natural language Q&A capabilities
- [x] Vector-based semantic search
- [x] Contextual responses with relevance scoring

### Architecture ✅

- [x] Modular crate structure (core, sidecar)
- [x] Pluggable embedding providers
- [x] Pluggable vector backends
- [x] Environment-based configuration
- [x] Docker deployment support

## v0.2 - Enhanced Intelligence

### Improved Q&A Capabilities

- [ ] Enhanced semantic understanding of complex queries
- [ ] Better relationship detection and explanation
- [ ] Index analysis and recommendations
- [ ] Constraint and foreign key visualization
- [ ] Schema change impact analysis

### Advanced DDL Processing

- [ ] Expand canonicalization coverage (triggers, functions, procedures)
- [ ] Better constraint and index representation
- [ ] View and materialized view support
- [ ] Schema dependency graph visualization

### CLI Enhancements

- [ ] Interactive schema exploration mode
- [ ] Export capabilities (JSON, SQL, diagrams)
- [ ] Batch question processing
- [ ] Custom query templates

## v0.3 - AI Integration

### OpenAI Integration

- [ ] OpenAI embedding provider for better semantic understanding
- [ ] GPT-powered natural language to SQL generation
- [ ] Intelligent schema recommendations
- [ ] Advanced query explanation and optimization suggestions

### Enhanced Vector Search

- [ ] Qdrant as production-ready vector backend
- [ ] Multi-dimensional similarity search
- [ ] Context-aware query understanding
- [ ] Learning from user interactions

### Performance & Scalability

- [ ] Optimized vector search performance
- [ ] Large schema support (1000+ objects)
- [ ] Incremental embedding updates
- [ ] Memory-efficient processing

## v0.4 - Multi-Database Support

### MySQL Connector

- [ ] MySQL catalog provider implementation
- [ ] MySQL-specific DDL canonicalization
- [ ] Cross-database schema comparison
- [ ] Unified CLI experience

### Database Agnostic Features

- [ ] Generic catalog provider trait
- [ ] Database-specific optimizations
- [ ] Schema migration analysis
- [ ] Cross-platform deployment

## v0.5 - Advanced Analytics

### Schema Intelligence

- [ ] Unused index detection
- [ ] Performance bottleneck identification
- [ ] Schema optimization recommendations
- [ ] Data flow analysis
- [ ] Security vulnerability detection

### Visualization & Reporting

- [ ] Interactive schema diagrams
- [ ] Relationship graphs
- [ ] Performance dashboards
- [ ] Change history visualization
- [ ] Export to various formats (PDF, HTML, etc.)

## v0.6 - Enterprise Features

### Team Collaboration

- [ ] Shared schema knowledge base
- [ ] Team annotations and comments
- [ ] Schema documentation generation
- [ ] Change approval workflows

### Integration & Plugins

- [ ] IDE plugins (VS Code, IntelliJ)
- [ ] CI/CD integration
- [ ] Webhook notifications for schema changes
- [ ] Programmatic access via library

### Security & Compliance

- [ ] Role-based access control
- [ ] Audit logging
- [ ] Data encryption
- [ ] Compliance reporting

## v0.7 - AI-Powered Development

### Natural Language to SQL

- [ ] Advanced NL-to-SQL generation
- [ ] Query optimization suggestions
- [ ] SQL code completion
- [ ] Query explanation and documentation

### Intelligent Recommendations

- [ ] Schema design suggestions
- [ ] Performance optimization advice
- [ ] Best practice recommendations
- [ ] Migration path suggestions

### Learning & Adaptation

- [ ] User behavior learning
- [ ] Personalized recommendations
- [ ] Context-aware suggestions
- [ ] Continuous improvement

## Future Horizons

### Advanced AI Capabilities

- [ ] Multi-modal understanding (diagrams + text)
- [ ] Predictive schema evolution
- [ ] Automated schema optimization
- [ ] Natural language schema design

### Ecosystem Integration

- [ ] ORM integration (Prisma, SQLAlchemy, etc.)
- [ ] Cloud database support (RDS, Cloud SQL, etc.)
- [ ] Kubernetes sidecar deployment
- [ ] Serverless deployment options

### Community & Ecosystem

- [ ] Plugin system for custom providers
- [ ] Community schema templates
- [ ] Marketplace for specialized connectors
- [ ] Open source contributions

## Development Principles

### Technical Excellence

- Maintain high code quality and test coverage
- Prioritize correctness of canonicalization and hashing
- Keep deployment and usage simple
- Focus on performance and reliability

### User Experience

- CLI-first approach for immediate value
- Intuitive natural language interface
- Fast, responsive interactions
- Helpful error messages and guidance

### Architecture

- Modular, pluggable design
- Local-first processing for privacy
- Sidecar deployment model
- Zero external dependencies by default

## Success Metrics

### v0.1 Success Criteria

- [x] Deploy sidecar on VM with PostgreSQL
- [x] Initialize and vectorize schema DDL
- [x] Ask natural language questions via CLI
- [x] Get relevant, contextual answers
- [x] Handle schema changes gracefully
- [x] Zero external dependencies (local_stub embeddings)

### Future Success Metrics

- User adoption and engagement
- Query response accuracy and relevance
- Performance and scalability benchmarks
- Community contributions and feedback
- Enterprise customer adoption

---

**Feytra** - Making databases as easy to understand as your code! 🤖💾
