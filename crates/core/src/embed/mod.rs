use async_trait::async_trait;

pub mod providers {
    pub mod local_stub;
    pub mod openai;
}

#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    async fn embed_batch(&self, inputs: &[String]) -> anyhow::Result<Vec<Vec<f32>>>;
}

pub fn build_input_text(object_name: &str, canonical_ddl: &str) -> String {
    format!("OBJECT {}\nDDL:\n{}", object_name, canonical_ddl)
}

pub fn build_embedding_provider_from_config(
    cfg: &crate::Config,
) -> anyhow::Result<Box<dyn EmbeddingProvider>> {
    match cfg.embed_provider.as_str() {
        "openai" => {
            let api_key = cfg
                .openai_api_key
                .clone()
                .ok_or_else(|| anyhow::anyhow!("OPENAI_API_KEY not set"))?;
            let model = cfg
                .openai_embed_model
                .clone()
                .unwrap_or_else(|| "text-embedding-3-small".to_string());
            Ok(Box::new(providers::openai::OpenAiEmbeddingProvider {
                api_key,
                model,
                dim: cfg.embed_dim,
            }))
        }
        _ => Ok(Box::new(
            providers::local_stub::LocalStubEmbeddingProvider { dim: cfg.embed_dim },
        )),
    }
}

pub fn build_llm_provider_from_config(
    cfg: &crate::Config,
) -> anyhow::Result<Option<providers::openai::OpenAiLlmProvider>> {
    if cfg.llm_provider == "openai" {
        let api_key = cfg
            .openai_api_key
            .clone()
            .ok_or_else(|| anyhow::anyhow!("OPENAI_API_KEY not set"))?;
        let model = cfg
            .openai_model
            .clone()
            .unwrap_or_else(|| "gpt-4o-mini".to_string());
        Ok(Some(providers::openai::OpenAiLlmProvider {
            api_key,
            model,
        }))
    } else {
        Ok(None)
    }
}
