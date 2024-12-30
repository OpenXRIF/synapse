use std::collections::HashMap;

use crate::constants::RAG_ARG_PREFIX;
use crate::models::prompt_format_models::{PromptFormat, PromptFormatArgType};
use crate::prompting::rag::fill::RagFiller;

/// Builds a prompt string from a `PromptFormat` and a set of arguments.
pub fn build_prompt(
    format: PromptFormat,
    args: HashMap<String, PromptFormatArgType>,
    rag_filler: &RagFiller,
) -> String {
    let mut prompt: String = format.prompt.clone();

    for (key, value) in args {
        // TODO: Document how to format prompts with rag arguments and auto-fills
        // Example prompt arg: {{ rag:get_waypoints_from_waypoint_set:e7-2025 }}
        if key.starts_with(RAG_ARG_PREFIX) {
            let rag_result: String = rag_filler.fill(&key[RAG_ARG_PREFIX.len()..]);

            prompt = prompt.replace(&format!("{{{{ {} }}}}", key), &rag_result);
        } else {
            let arg_value: String = match value {
                PromptFormatArgType::String(value) => format!("{}", value),
                PromptFormatArgType::Int(value) => format!("{}", value),
                PromptFormatArgType::Bool(value) => format!("{}", value),
                PromptFormatArgType::List(value) => format!("{:?}", value),
            };

            prompt = prompt.replace(&format!("{{{{ {} }}}}", key), &arg_value);
        }
    }

    prompt
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::prompt_format_models::PromptFormatArgType;

    fn setup_test() -> RagFiller<'static> {
        let mut functions: HashMap<&str, fn(&str) -> String> = HashMap::new();
        functions.insert("test_rag_arg", |_| "test_rag_value".to_string());
        RagFiller::new_test(functions)
    }

    #[test]
    fn test_build_prompt() {
        let rag_filler: RagFiller<'_> = setup_test();
        let format: PromptFormat = PromptFormat {
            format_name: "test".to_string(),
            prompt: "This is a test prompt with a {{ test_arg }}.".to_string(),
            prompt_args: HashMap::new(),
            metadata: HashMap::new(),
        };

        let mut args: HashMap<String, PromptFormatArgType> = HashMap::new();
        args.insert(
            "test_arg".to_string(),
            PromptFormatArgType::String("test_arg_value".to_string().into()),
        );

        let prompt = build_prompt(format, args, &rag_filler);

        assert_eq!(prompt, "This is a test prompt with a test_arg_value.");
    }

    #[test]
    fn test_build_prompt_with_rag_arg() {
        let rag_filler: RagFiller<'_> = setup_test();

        let format: PromptFormat = PromptFormat {
            format_name: "test".to_string(),
            prompt: "This is a test prompt with a {{ rag:test_rag_arg }}.".to_string(),
            prompt_args: HashMap::new(),
            metadata: HashMap::new(),
        };

        let mut args: HashMap<String, PromptFormatArgType> = HashMap::new();
        args.insert(
            "rag:test_rag_arg".to_string(),
            PromptFormatArgType::String(String::new()),
        );

        let prompt: String = build_prompt(format, args, &rag_filler);

        assert_eq!(prompt, "This is a test prompt with a test_rag_value.");
    }
}
