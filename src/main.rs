use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};
use colored::Colorize;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use teloxide::prelude::*;
use teloxide::types::ChatId;
use tracing_subscriber::EnvFilter;

const APP_NAME: &str = "telegram-notifier";
const SYSTEM_CONFIG_PATH: &str = "/etc/telegram-notifier/config.toml";

#[derive(Debug, Parser)]
#[command(
    name = "telegram-notifier",
    version,
    about = "Send Telegram notifications or listen for incoming messages."
)]
struct Cli {
    /// Bot token (overrides config)
    #[arg(long)]
    token: Option<String>,

    /// Target chat id (overrides config; required for send)
    #[arg(long)]
    chat_id: Option<i64>,

    /// Use custom config path instead of defaults
    #[arg(long)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Send a text message to the configured chat
    Send {
        /// Message text to send
        message: String,
    },
    /// Listen for incoming messages and print chat_id/user/text
    Listen,
}

#[derive(Debug, Deserialize, Default, Clone)]
struct FileConfig {
    bot_token: Option<String>,
    chat_id: Option<i64>,
}

#[derive(Debug)]
struct ResolvedConfig {
    bot_token: String,
    chat_id: Option<i64>,
}

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    let cli = Cli::parse();
    let config = resolve_config(&cli).context("failed to resolve configuration")?;

    match cli.command {
        Commands::Send { message } => send_message(&config, message).await?,
        Commands::Listen => listen_messages(&config).await?,
    }

    Ok(())
}

fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(env_filter).init();
}

async fn send_message(config: &ResolvedConfig, message: String) -> Result<()> {
    let chat_id = config.chat_id.ok_or_else(|| {
        anyhow!(
            "chat_id is required for send mode; run `listen` to get it or pass via --chat-id/config"
        )
    })?;

    let bot = Bot::new(&config.bot_token);
    bot.send_message(ChatId(chat_id), message).await?;

    println!("Message sent to chat_id {}", chat_id);
    Ok(())
}

async fn listen_messages(config: &ResolvedConfig) -> Result<()> {
    let bot = Bot::new(&config.bot_token);

    let handler = Update::filter_message().endpoint(|_bot: Bot, msg: Message| async move {
        print_message(&msg);
        Ok::<(), teloxide::RequestError>(())
    });

    let mut dispatcher = Dispatcher::builder(bot, handler).build();

    let dispatch_handle = tokio::spawn(async move {
        dispatcher.dispatch().await;
    });

    let mut dispatch_handle_opt = Some(dispatch_handle);

    tokio::select! {
        res = async {
            if let Some(handle) = dispatch_handle_opt.take() {
                handle.await.context("dispatcher task failed")
            } else {
                Ok(())
            }
        } => { res?; }
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Ctrl+C received, aborting dispatcher");
            if let Some(handle) = dispatch_handle_opt.take() {
                handle.abort();
            }
        }
    };

    Ok(())
}

fn print_message(msg: &Message) {
    let ts_local: DateTime<Local> = msg.date.with_timezone(&Local);

    let chat_id = msg.chat.id.0;
    let chat_colored = format!("{chat_id}").red();

    let user_display = msg
        .from()
        .map(format_user)
        .unwrap_or_else(|| "<unknown user>".to_string());

    let content = msg
        .text()
        .map(|s| s.to_string())
        .or_else(|| msg.caption().map(|s| s.to_string()))
        .unwrap_or_else(|| format!("<{:?}>", msg.kind));

    println!(
        "[{}] chat_id={} user={} text={}",
        ts_local.format("%Y-%m-%d %H:%M:%S"),
        chat_colored,
        user_display,
        content
    );
}

fn format_user(user: &teloxide::types::User) -> String {
    if let Some(username) = &user.username {
        format!("@{username}")
    } else {
        let mut name = user.first_name.clone();
        if let Some(last) = &user.last_name {
            if !last.is_empty() {
                name.push(' ');
                name.push_str(last);
            }
        }
        if name.trim().is_empty() {
            "<no-name>".to_string()
        } else {
            name
        }
    }
}

fn resolve_config(cli: &Cli) -> Result<ResolvedConfig> {
    let mut file_config = FileConfig::default();

    // Load from /etc (lowest priority)
    if let Some(cfg) = load_config_file(Path::new(SYSTEM_CONFIG_PATH))? {
        file_config = merge(file_config, cfg);
    }

    // Load from user config (higher priority)
    if let Some(user_path) = cli.config.clone().or_else(default_user_config_path) {
        if let Some(cfg) = load_config_file(&user_path)? {
            file_config = merge(file_config, cfg);
        }
    }

    let bot_token = cli
        .token
        .clone()
        .or(file_config.bot_token)
        .ok_or_else(|| anyhow!("bot token is required (use --token or config file)"))?;

    let chat_id = cli.chat_id.or(file_config.chat_id);

    Ok(ResolvedConfig { bot_token, chat_id })
}

fn merge(base: FileConfig, overlay: FileConfig) -> FileConfig {
    FileConfig {
        bot_token: overlay.bot_token.or(base.bot_token),
        chat_id: overlay.chat_id.or(base.chat_id),
    }
}

fn load_config_file(path: &Path) -> Result<Option<FileConfig>> {
    if !path.exists() {
        return Ok(None);
    }
    let data = fs::read_to_string(path)
        .with_context(|| format!("failed to read config file {}", path.display()))?;
    let cfg: FileConfig = toml::from_str(&data)
        .with_context(|| format!("failed to parse config file {}", path.display()))?;
    Ok(Some(cfg))
}

fn default_user_config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join(APP_NAME).join("config.toml"))
}
