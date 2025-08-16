use crate::embed::EmbeddingProvider;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub struct OpenAiEmbeddingProvider {
    pub api_key: String,
    pub model: String,
    pub dim: usize,
}

pub struct OpenAiLlmProvider {
    pub api_key: String,
    pub model: String,
}

#[derive(Serialize)]
struct EmbeddingsRequestBody<'a> {
    input: &'a [String],
    model: &'a str,
}

#[derive(Deserialize)]
struct EmbeddingsResponse {
    data: Vec<EmbeddingItem>,
}

#[derive(Deserialize)]
struct EmbeddingItem {
    embedding: Vec<f32>,
}

#[derive(Serialize)]
struct ChatRequestBody<'a> {
    model: &'a str,
    messages: &'a [ChatMessage],
    temperature: f32,
}

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[async_trait]
impl EmbeddingProvider for OpenAiEmbeddingProvider {
    async fn embed_batch(&self, inputs: &[String]) -> anyhow::Result<Vec<Vec<f32>>> {
        if inputs.is_empty() {
            return Ok(Vec::new());
        }
        let client = reqwest::Client::new();
        let body = EmbeddingsRequestBody {
            input: inputs,
            model: &self.model,
        };
        let res = client
            .post("https://api.openai.com/v1/embeddings")
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;
        if !res.status().is_success() {
            let text = res.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(format!(
                "openai embeddings error: {}",
                text
            )));
        }
        let parsed: EmbeddingsResponse = res.json().await?;
        let mut out: Vec<Vec<f32>> = Vec::with_capacity(parsed.data.len());
        for item in parsed.data {
            let mut vec = item.embedding;
            if self.dim > 0 && vec.len() != self.dim {
                // If dimension differs, pad or truncate to configured dim for consistency
                if vec.len() < self.dim {
                    vec.resize(self.dim, 0.0);
                } else {
                    vec.truncate(self.dim);
                }
            }
            out.push(vec);
        }
        Ok(out)
    }
}

impl OpenAiLlmProvider {
    pub async fn generate_response(&self, question: &str, context: &str) -> anyhow::Result<String> {
        let client = reqwest::Client::new();

        let system_prompt = format!(
            "You are an AI database assistant that helps users understand their database schema. 
            You have access to the following database schema information:
            
            {}
            
            Answer the user's question about their database in a helpful, clear, and concise way. 
            If the question is about tables, relationships, indexes, or schema structure, use the provided context to give accurate answers.
            If you don't have enough information to answer the question, say so and suggest what additional information might be needed.",
            context
        );

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt,
            },
            ChatMessage {
                role: "user".to_string(),
                content: question.to_string(),
            },
        ];

        let body = ChatRequestBody {
            model: &self.model,
            messages: &messages,
            temperature: 0.3, // Lower temperature for more consistent responses
        };

        let res = client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;

        if !res.status().is_success() {
            let text = res.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(format!("openai chat error: {}", text)));
        }

        let parsed: ChatResponse = res.json().await?;
        if parsed.choices.is_empty() {
            return Err(anyhow::anyhow!("No response from OpenAI"));
        }

        Ok(parsed.choices[0].message.content.clone())
    }
}
