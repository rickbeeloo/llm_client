use crate::providers::llama_cpp::LlamaLlm;
use crate::text_utils;
use crate::LlmDefinition;
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;

pub async fn generate_logit_bias_from_chars(
    llm_definition: &LlmDefinition,
    allowed_chars: Option<Vec<String>>,
    removed_chars: Option<Vec<String>>,
) -> Result<HashMap<String, serde_json::Value>, Box<dyn Error>> {
    let mut logit_bias = HashMap::new();
    match llm_definition {
        LlmDefinition::OpenAiLlm(_) => {
            if let Some(allowed_chars) = allowed_chars {
                let allowed_tokens = text_utils::get_char_tokens(&allowed_chars);
                for token in allowed_tokens {
                    logit_bias.insert(token.to_string(), json!(100));
                }
            }
            if let Some(removed_chars) = removed_chars {
                let removed_tokens = text_utils::get_char_tokens(&removed_chars);
                for token in removed_tokens {
                    logit_bias.insert(token.to_string(), json!(-100));
                }
            }
        }
        LlmDefinition::LlamaLlm(_) => {
            let provider = LlamaLlm::new();
            if let Some(allowed_chars) = allowed_chars {
                let allowed_tokens = provider.tokenize_chars(&allowed_chars).await?;
                for token in allowed_tokens {
                    logit_bias.insert(token.to_string(), json!(100.0));
                }
            }
            if let Some(removed_chars) = removed_chars {
                let removed_tokens = provider.tokenize_chars(&removed_chars).await?;
                for token in removed_tokens {
                    logit_bias.insert(token.to_string(), json!(-100.0));
                }
            }
        }
    }
    Ok(logit_bias)
}

pub fn generate_punctuation() -> Vec<String> {
    (['.', ',', ';', ':', '!', '?', '\'', '"'].iter())
        .map(|&c| c.to_string())
        .collect()
}

pub fn generate_bad_split_chars() -> Vec<String> {
    [
        "[]", "[", "]", "()", "(", ")", "•", "entry", "Entry", "story", "Story", "Feature",
        "feature",
    ]
    .iter()
    .map(|&c| c.to_string())
    .collect()

    // bad_chars.extend(('1'..='9').map(|c| c.to_string()));
    // bad_chars.extend((1..=9).map(|n| format!("{:02}", n)));
}

pub fn generate_whitespace_chars() -> Vec<String> {
    ([r"\t", r"\r", r"\v", r"\f"].iter())
        .map(|&c| c.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::llama_cpp::models;
    use crate::providers::llama_cpp::models::LlamaLlmModel;
    use crate::providers::llama_cpp::server::start_server;
    use crate::providers::llm_openai::models::OpenAiLlmModels;

    #[tokio::test]
    async fn test_llm_openai() {
        let allowed_chars: Vec<String> = vec![
            "hello".to_string(),
            "there".to_string(),
            "general".to_string(),
        ];
        let removed_chars: Vec<String> = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        let response = generate_logit_bias_from_chars(
            &LlmDefinition::OpenAiLlm(OpenAiLlmModels::Gpt35Turbo),
            Some(allowed_chars),
            Some(removed_chars),
        )
        .await;
        eprintln!("{:?}", response.unwrap());
    }

    #[tokio::test]
    async fn test_llama_cpp() {
        let allowed_chars: Vec<String> = vec![
            "hello".to_string(),
            "there".to_string(),
            "general".to_string(),
        ];
        let removed_chars: Vec<String> = vec!["1".to_string(), "2".to_string()];
        let zephyr_7b_chat = LlamaLlmModel::new(
            "https://huggingface.co/TheBloke/zephyr-7B-alpha-GGUF/blob/main/zephyr-7b-alpha.Q5_K_M.gguf",
            models::LlamaPromptFormat::Mistral7BChat,
            Some(9001),
        );
        let _ = start_server(
            &zephyr_7b_chat.model_id,
            &zephyr_7b_chat.model_filename,
            None,
            None,
            Some(zephyr_7b_chat.max_tokens_for_model),
            None,
        )
        .await;
        let llm_definition = LlmDefinition::LlamaLlm(zephyr_7b_chat);
        let response = generate_logit_bias_from_chars(
            &llm_definition,
            Some(allowed_chars),
            Some(removed_chars),
        )
        .await;
        eprintln!("{:?}", response.unwrap());
    }
}
