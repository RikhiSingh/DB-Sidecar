use clap::{Parser, Subcommand};
use feytra_core::{jobs, Config};
use std::io::{self, Write};
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(name = "feytra")]
#[command(about = "Feytra - AI-powered database schema intelligence", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Start interactive database assistant
    Chat,
    /// Initialize schema intelligence (run once)
    Init,
    /// Refresh schema data
    Refresh,
    /// Start API server
    Serve,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cfg = Config::from_env();

    match Cli::parse().command {
        Command::Chat => {
            run_chat_mode(&cfg).await?;
        }
        Command::Init => {
            println!("🚀 Initializing Feytra schema intelligence...");
            match jobs::execute_bootstrap(&cfg).await {
                Ok(version) => println!("✅ Initialized successfully! Version: {}", version),
                Err(e) => println!("❌ Initialization failed: {}", e),
            }
        }
        Command::Refresh => {
            println!("🔄 Refreshing schema data...");
            match jobs::execute_refresh(&cfg).await {
                Ok(result) => println!(
                    "✅ Refreshed! Version: {}, Changed: {:?}",
                    result.version, result.changed_ids
                ),
                Err(e) => println!("❌ Refresh failed: {}", e),
            }
        }
        Command::Serve => {
            let app = feytra_api::build_router(cfg.clone());
            let addr = std::net::SocketAddr::from(([0, 0, 0, 0], cfg.port));
            println!("🌐 Starting API server on {}", addr);
            axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
        }
    }
    Ok(())
}

async fn run_chat_mode(cfg: &Config) -> anyhow::Result<()> {
    println!("🤖 Welcome to Feytra - Your AI Database Assistant!");
    println!("==================================================");
    println!("Database: {}", cfg.database_url);
    println!(
        "Intelligence: {} embeddings, {} vectors",
        cfg.embed_provider, cfg.vector_backend
    );
    println!();

    // Check if we need to initialize
    let repo = feytra_core::repo::sqlite::SqliteRepo::open(&cfg.sidecar_db).await?;
    repo.migrate().await?;

    let embeddings = repo.fetch_all_embeddings().await.unwrap_or_default();
    if embeddings.is_empty() {
        println!("⚠️  No schema intelligence found. Running initialization...");
        match jobs::execute_bootstrap(cfg).await {
            Ok(version) => println!("✅ Initialized! Version: {}", version),
            Err(e) => {
                println!(
                    "❌ Initialization failed: {}. You can run 'feytra init' manually.",
                    e
                );
                println!("Continuing with limited functionality...");
            }
        }
    } else {
        println!(
            "✅ Schema intelligence ready! Found {} objects",
            embeddings.len()
        );
    }

    println!("\n💬 Ask me anything about your database schema!");
    println!("Examples:");
    println!("  • \"What tables do I have?\"");
    println!("  • \"Show me the structure of the users table\"");
    println!("  • \"What are the relationships between tables?\"");
    println!("  • \"Find tables related to authentication\"");
    println!("  • \"refresh\" - Update schema data");
    println!("  • \"exit\" - End session");
    println!();

    // Chat loop
    loop {
        print!("💭 ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let question = input.trim();

        if question.is_empty() {
            continue;
        }

        if question.eq_ignore_ascii_case("exit") || question.eq_ignore_ascii_case("quit") {
            println!("👋 Goodbye! Happy coding!");
            break;
        }

        if question.eq_ignore_ascii_case("refresh") {
            println!("🔄 Refreshing schema data...");
            match jobs::execute_refresh(cfg).await {
                Ok(result) => println!(
                    "✅ Refreshed! Version: {}, Changed: {:?}",
                    result.version, result.changed_ids
                ),
                Err(e) => println!("❌ Refresh failed: {}", e),
            }
            println!();
            continue;
        }

        // Process the question
        print!("🤔 ");
        io::stdout().flush()?;

        match process_question(cfg, question).await {
            Ok(response) => println!("{}", response),
            Err(e) => println!("❌ Error: {}", e),
        }
        println!();
    }

    Ok(())
}

async fn process_question(cfg: &Config, question: &str) -> anyhow::Result<String> {
    let question_lower = question.to_lowercase();

    // Handle specific commands
    match question_lower.as_str() {
        "help" | "?" => {
            return Ok(HELP_TEXT.to_string());
        }
        "tables" | "list tables" | "show tables" => {
            return list_tables(cfg).await;
        }
        "schema" | "schema info" => {
            return get_schema_info(cfg).await;
        }
        _ => {}
    }

    // Try vector-based search for semantic understanding
    match answer_with_vectors(cfg, question).await {
        Ok(answer) => Ok(answer),
        Err(_) => {
            // Fallback to basic responses
            if question_lower.contains("table") {
                Ok("I can help you explore your tables! Try asking:\n• \"What tables do I have?\"\n• \"Show me the structure of [table_name]\"\n• \"Find tables related to [topic]\"".to_string())
            } else if question_lower.contains("relationship")
                || question_lower.contains("foreign key")
            {
                Ok("I can help you understand table relationships! Try asking:\n• \"What are the relationships between tables?\"\n• \"Show me foreign keys in [table_name]\"\n• \"How are [table1] and [table2] connected?\"".to_string())
            } else if question_lower.contains("index") {
                Ok("I can help you with indexes! Try asking:\n• \"What indexes do I have?\"\n• \"Show me indexes on [table_name]\"\n• \"Are there any missing indexes?\"".to_string())
            } else {
                Ok(format!("I understand you're asking about: \"{}\"\n\nTry asking me about:\n• Tables and their structure\n• Database relationships and foreign keys\n• Indexes and performance\n• Schema information\n\nOr ask me to \"refresh\" to update the schema data.", question))
            }
        }
    }
}

async fn list_tables(cfg: &Config) -> anyhow::Result<String> {
    use feytra_core::catalog::{pg::PgCatalogProvider, CatalogProvider};
    use sqlx::PgPool;

    let pool = PgPool::connect(&cfg.database_url).await?;
    let provider = PgCatalogProvider::new(pool);
    let objects = provider.snapshot().await?;

    let tables: Vec<_> = objects
        .iter()
        .filter(|o| matches!(o.kind, feytra_core::catalog::ObjectKind::Table))
        .map(|o| o.qualified_name.clone())
        .collect();

    if tables.is_empty() {
        Ok("📋 No tables found in the database.".to_string())
    } else {
        let mut response = format!("📋 Found {} tables:\n\n", tables.len());
        for (i, table) in tables.iter().enumerate() {
            response.push_str(&format!("{}. {}\n", i + 1, table));
        }
        Ok(response)
    }
}

async fn get_schema_info(cfg: &Config) -> anyhow::Result<String> {
    use feytra_core::catalog::{pg::PgCatalogProvider, CatalogProvider};
    use sqlx::PgPool;

    let pool = PgPool::connect(&cfg.database_url).await?;
    let provider = PgCatalogProvider::new(pool);
    let objects = provider.snapshot().await?;

    let mut info = Vec::new();
    info.push(format!("📊 Database Schema Overview"));
    info.push(format!("Total objects: {}", objects.len()));

    let mut kind_counts = std::collections::HashMap::new();
    for obj in &objects {
        *kind_counts.entry(format!("{:?}", obj.kind)).or_insert(0) += 1;
    }

    info.push("\n📈 Object Breakdown:".to_string());
    for (kind, count) in kind_counts {
        info.push(format!("  • {}: {}", kind, count));
    }

    Ok(info.join("\n"))
}

async fn answer_with_vectors(cfg: &Config, question: &str) -> anyhow::Result<String> {
    use feytra_core::{
        embed::build_embedding_provider_from_config,
        repo::sqlite::SqliteRepo,
        vectors::{hnsw::HnswIndex, VectorIndex},
    };

    // Get embeddings for the question
    let provider = build_embedding_provider_from_config(cfg)?;
    let question_embedding = provider.embed_batch(&[question.to_string()]).await?;

    if question_embedding.is_empty() {
        return Ok("Sorry, I couldn't process your question.".to_string());
    }

    // Get repository and search for similar objects
    let repo = SqliteRepo::open(&cfg.sidecar_db).await?;
    let embeddings = repo.fetch_all_embeddings().await?;

    if embeddings.is_empty() {
        return Ok("No schema information available. Try running 'feytra init' first.".to_string());
    }

    // Build HNSW index and search
    let index = HnswIndex::new(cfg.embed_dim);

    for (object_id, embedding) in &embeddings {
        index.upsert(object_id.clone(), embedding.clone()).await?;
    }

    let results = index.search(question_embedding[0].clone(), 5).await?;

    if results.is_empty() {
        return Ok("I couldn't find relevant schema information for your question.".to_string());
    }

    // Get object details for the top results
    let mut answer = "🔍 Here are the most relevant schema objects:\n\n".to_string();

    for (object_id, score) in results {
        if let Ok(Some((kind, qualified_name, canonical_ddl))) =
            repo.fetch_object_by_id(&object_id).await
        {
            answer.push_str(&format!(
                "📋 {} (relevance: {:.1}%)\n",
                qualified_name,
                score * 100.0
            ));
            answer.push_str(&format!("   Type: {}\n", kind));

            if !canonical_ddl.is_empty() {
                let ddl_preview = if canonical_ddl.len() > 150 {
                    format!("{}...", &canonical_ddl[..150])
                } else {
                    canonical_ddl.clone()
                };
                answer.push_str(&format!("   Structure: {}\n", ddl_preview));
            }
            answer.push_str("\n");
        }
    }

    answer
        .push_str("💡 Tip: Ask me to \"refresh\" to update the schema data if it seems outdated.");
    Ok(answer)
}

const HELP_TEXT: &str = r#"🎯 Feytra Database Assistant Help

📋 Available Commands:
• "tables" or "list tables" - Show all tables
• "schema" or "schema info" - Database overview
• "refresh" - Update schema data
• "exit" or "quit" - End session

💡 Example Questions:
• "What tables do I have?"
• "Show me the structure of the users table"
• "What are the relationships between tables?"
• "Find tables related to authentication"
• "Are there any missing indexes?"
• "How is the orders table structured?"

🔧 Technical Commands:
• feytra init - Initialize schema intelligence
• feytra refresh - Refresh schema data
• feytra serve - Start API server

💭 Just ask me anything about your database schema!
"#;
