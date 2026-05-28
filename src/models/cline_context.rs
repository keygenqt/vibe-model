use crate::utils::parse;

/// Cline-specific prompt data.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ClineContextModel {
    /// Raw context text (everything between tags).
    pub context: Option<String>,
    /// Environment details from <environment_details>.
    pub environment_details: Option<String>,
    /// Terminal output from <terminal_output>.
    pub terminal_output: Option<String>,
    /// Workspace diagnostics from <workspace_diagnostics>.
    pub workspace_diagnostics: Option<String>,
}

#[allow(dead_code)]
impl ClineContextModel {
    /// Extract open files from environment_details.
    pub fn find_open_files(&self) -> Vec<String> {
        let section = "Visual Studio Code Open Tabs";
        self.environment_details
            .as_ref()
            .map(|s| {
                parse::extract_sections(s, &[section])
                    .get(section)
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    }

    /// Extract visible files from environment_details.
    pub fn find_visible_files(&self) -> Vec<String> {
        let section = "Visual Studio Code Visible Files";
        self.environment_details
            .as_ref()
            .map(|s| {
                parse::extract_sections(s, &[section])
                    .get(section)
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    }

    /// Extract current mode from environment_details.
    pub fn find_mode(&self) -> Option<String> {
        let section = "Current Mode";
        self.environment_details.as_ref().and_then(|s| {
            parse::extract_sections(s, &[section])
                .get(section)
                .and_then(|v| v.as_str().map(String::from))
        })
    }

    /// Extract current time from environment_details.
    pub fn find_time(&self) -> Option<String> {
        let section = "Current Time";
        self.environment_details.as_ref().and_then(|s| {
            parse::extract_sections(s, &[section])
                .get(section)
                .and_then(|v| v.as_str().map(String::from))
        })
    }
}
