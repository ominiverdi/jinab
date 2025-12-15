use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::ExitCode;

const ENV_VAR_NAME: &str = "JINA_API_KEY";
const CONFIG_DIR_NAME: &str = "jinab";
const CONFIG_FILE_NAME: &str = "config";

#[derive(Parser)]
#[command(name = "jinab")]
#[command(about = "Read and search the web using Jina AI's Reader API")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Store the Jina API key to ~/.config/jinab/config
    Key {
        /// The Jina API key to store
        api_key: String,
    },
    /// Read a webpage using Jina's Reader API
    Read {
        /// The URL to read
        url: String,
        /// Output JSON instead of markdown
        #[arg(long)]
        json: bool,
    },
    /// Search the web using Jina's Search API
    Search {
        /// The search query
        query: String,
        /// Output JSON instead of markdown
        #[arg(long)]
        json: bool,
    },
    /// Generate shell completions
    Completions {
        /// The shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

fn get_config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join(CONFIG_DIR_NAME).join(CONFIG_FILE_NAME))
}

fn save_api_key(api_key: &str) -> Result<(), String> {
    let config_path = get_config_path().ok_or("Could not determine config directory")?;

    let config_dir = config_path
        .parent()
        .ok_or("Could not determine config directory parent")?;

    fs::create_dir_all(config_dir)
        .map_err(|e| format!("Failed to create config directory: {}", e))?;

    fs::write(&config_path, api_key.trim())
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    Ok(())
}

fn load_api_key() -> Option<String> {
    // Environment variable takes precedence
    if let Ok(key) = std::env::var(ENV_VAR_NAME) {
        let key = key.trim().to_string();
        if !key.is_empty() {
            return Some(key);
        }
    }

    // Fall back to config file
    if let Some(config_path) = get_config_path() {
        if let Ok(key) = fs::read_to_string(config_path) {
            let key = key.trim().to_string();
            if !key.is_empty() {
                return Some(key);
            }
        }
    }

    None
}

fn require_api_key() -> Result<String, String> {
    load_api_key().ok_or_else(|| {
        format!(
            "No API key found. Set {} environment variable or run: jinab key <api-key>",
            ENV_VAR_NAME
        )
    })
}

fn jina_request(endpoint: &str, path: &str, api_key: &str, accept_json: bool) -> Result<String, String> {
    let url = format!("{}/{}", endpoint, path);

    let client = reqwest::blocking::Client::new();
    let mut request = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key));

    if accept_json {
        request = request.header("Accept", "application/json");
    }

    let response = request
        .send()
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "API request failed with status {}: {}",
            response.status(),
            response.text().unwrap_or_default()
        ));
    }

    response.text().map_err(|e| format!("Failed to read response: {}", e))
}

fn cmd_key(api_key: &str) -> Result<(), String> {
    save_api_key(api_key)?;
    eprintln!("API key saved to {:?}", get_config_path().unwrap());
    Ok(())
}

fn cmd_read(url: &str, json: bool) -> Result<(), String> {
    let api_key = require_api_key()?;
    let content = jina_request("https://r.jina.ai", url, &api_key, json)?;
    print!("{}", content);
    Ok(())
}

fn cmd_search(query: &str, json: bool) -> Result<(), String> {
    let api_key = require_api_key()?;
    let content = jina_request("https://s.jina.ai", query, &api_key, json)?;
    print!("{}", content);
    Ok(())
}

fn cmd_completions(shell: Shell) {
    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "jinab", &mut io::stdout());
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Key { api_key } => cmd_key(&api_key),
        Commands::Read { url, json } => cmd_read(&url, json),
        Commands::Search { query, json } => cmd_search(&query, json),
        Commands::Completions { shell } => {
            cmd_completions(shell);
            return ExitCode::SUCCESS;
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}
