//! Cline prompt parser.
//! Extracts task, file content, and feedback from Cline prompts.

use crate::{
    clients::{cline::parser::models::PromptTagsModel, parser::ClientParser},
    models::{
        cline_context::ClineContextModel,
        prompt::{PromptFile, PromptMode, PromptModel},
    },
    utils::parse,
};

/// Parses Cline-specific prompt format.
pub struct ClineParser;

impl ClientParser for ClineParser {
    /// Parse raw prompt into structured PromptModel.
    fn parse(&self, prompt: &str) -> PromptModel {
        // @todo
        // println!("1 ==============");
        // println!("{}", prompt);

        let known_tags = &[
            "task",
            "feedback",
            "file_content",
            "environment_details",
            "terminal_output",
            "final_file_content",
            "workspace_diagnostics",
        ];

        // 1. Extract tags as JSON
        let json = parse::extract_tags(prompt, known_tags);

        // 2. Parse JSON to model
        let tags: PromptTagsModel = match serde_json::from_value(json) {
            Ok(data) => data,
            Err(e) => {
                tracing::error!("JSON parse error: {:?}", e);
                return PromptModel::new("Cline prompt parsing failed.");
            }
        };

        // 3. Strip known tags to get raw context
        let context = parse::strip_tags(prompt, known_tags);

        // 4. Get prompt
        let prompt = tags.get_user_prompt(&context);

        // 5. Get files
        let files: Vec<PromptFile> = tags
            .file_content
            .into_iter()
            .map(|f| PromptFile {
                path: f.path,
                content: f.content,
            })
            .collect();

        // 6. Determine prompt mode
        let mode = if tags.task.is_some() || tags.feedback.is_some() {
            PromptMode::Task
        } else if let Some(ctx) = &context {
            if ctx.contains("[write_to_file")
                || ctx.contains("[attempt_completion]")
                || ctx.contains("[ERROR]")
            {
                PromptMode::Service
            } else {
                PromptMode::Task
            }
        } else {
            PromptMode::Unknown
        };

        // 7. Build Cline context
        let cline_context = ClineContextModel {
            context,
            environment_details: tags.environment_details,
            terminal_output: tags.terminal_output,
            workspace_diagnostics: tags.workspace_diagnostics,
        };

        PromptModel {
            prompt,
            mode,
            files,
            cline_context: Some(cline_context),
        }
    }
}
