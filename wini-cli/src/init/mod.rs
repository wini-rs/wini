use {
    err::InitError,
    inquire::{
        ui::{RenderConfig, Styled},
        Select,
        Text,
    },
    std::{fmt::Display, sync::LazyLock},
};

pub mod ask;
pub mod err;
pub mod git;
pub mod rename;

pub const SEP: &str = "\x1B[90m│\x1B[0m";

pub fn sep() {
    println!("{SEP}");
}

/// The official URL of the repo
const WINI_REPO: &str = "https://codeberg.org/wini/wini-template";

/// The render config used by inquire for the prompts
static RENDER_CONFIG: LazyLock<RenderConfig> = LazyLock::new(|| {
    RenderConfig::default_colored()
        .with_prompt_prefix(Styled::new("◆").with_fg(inquire::ui::Color::LightCyan))
        .with_answered_prompt_prefix(Styled::new("◇").with_fg(inquire::ui::Color::DarkCyan))
        .with_highlighted_option_prefix(Styled::new("►").with_fg(inquire::ui::Color::DarkCyan))
});

const HEADER: &str = "\
┌───────────────────────────────────┐
│ \x1b[36mWelcome to your new Wini project!\x1B[0m │
\x1b[36m◆\x1b[0m ──────────────────────────────────┘";


const OFFICIAL_REPOSITORY_OPTIONS: &[&str] = &[
    "Basic",
    "Basic - Workspace",
    "Meta",
    "Meta - Workspace",
];
const OFFICIAL_REPOSITORY_BRANCHES: &[&str] = &["main", "workspaces", "meta", "meta-workspaces"];

pub struct RepoSummary {
    dir: String,
    remote_url: Option<String>,
    branch: String,
    last_commit_hash: String,
}

/// Creates a select prompt
pub fn select<T>(title: &str, options: Vec<T>) -> Result<usize, InitError>
where
    T: Display,
{
    match Select::new(title, options).with_vim_mode(true).raw_prompt() {
        Ok(r) => Ok(r.index),
        Err(_) => Err(InitError::ManualExit),
    }
}

/// Creates an input prompt
pub fn input(prompt: &str) -> Result<String, InitError> {
    Text::new(prompt)
        .prompt()
        .map_err(|_| InitError::ManualExit)
}
