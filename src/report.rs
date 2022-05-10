use colored::{ColoredString, Colorize};
use std::error::Error;
use tabled::{Style, Table, Tabled};

use crate::error::ConfigurationError;
use crate::Configuration;

#[derive(Tabled)]
struct Report {
    ok: ColoredString,
    desc: String,
    reason: ColoredString,
}

impl Report {
    pub fn new(ok: &'static str, desc: &'static str) -> Self {
        Report {
            ok: ok.green(),
            desc: String::from(desc),
            reason: ColoredString::default(),
        }
    }

    // TODO(young): e should be string or ConfigurationError?
    pub fn update_reason(mut self, e: ConfigurationError) -> Self {
        let mut vec = vec![format!("{}", e)];
        if let Some(s) = e.source() {
            vec.push(s.to_string());
        }

        self.reason = vec.join("\n").red();

        self
    }
}

pub fn generate_configuration_report(
    config: &Configuration,
    err: Option<ConfigurationError>,
) -> String {
    let config_err_message = match err {
        Some(e) => Report::new("X", "no config err").update_reason(e),
        None => Report::new("O", "no config err"),
    };

    let slack_channel_message = match config.get_slack_channel() {
        // TODO(young): Reason?
        Some(_) => Report::new("O", "slack_channel"),
        None => {
            Report::new("X", "slack_channel").update_reason(ConfigurationError::SlackConfigNotFound)
        }
    };

    let slack_token_message = match config.get_slack_channel() {
        // TODO(young): Reason?
        Some(_) => Report::new("O", "slack_token"),
        None => {
            Report::new("X", "slack_token").update_reason(ConfigurationError::SlackConfigNotFound)
        }
    };

    let discord_webhook_url_message = match config.get_discord_webhook_url() {
        // TODO(young): Reason?
        Some(_) => Report::new("O", "discord_webhook_url"),
        None => Report::new("X", "discord_webhook_url")
            .update_reason(ConfigurationError::DiscordConfigNotFound),
    };

    Table::new(vec![
        config_err_message,
        slack_channel_message,
        slack_token_message,
        discord_webhook_url_message,
    ])
    .with(Style::modern())
    .to_string()
}