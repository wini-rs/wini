use {
    err::InitError,
    inquire::{
        ui::{RenderConfig, Styled},
        Confirm,
        Select,
        Text,
    },
    std::{collections::HashMap, fmt::Display, sync::LazyLock},
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

struct Answer {
    yes: &'static str,
    no: &'static str,
    default: bool,
}

const OFFICIAL_REPOSITORY_QUESTIONS: &[(&str, Answer)] = &[
    (
        "Do you want to do static site generation (SSG) ?",
        Answer {
            yes: "ssg",
            no: "ssr",
            default: false,
        },
    ),
    (
        "Do you want to use Nushell as the shell ?",
        Answer {
            yes: "nushell",
            no: "posix-sh",
            default: false,
        },
    ),
];

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

pub fn prompt_yes_no(question: &'static str, default: bool) -> Result<bool, InitError> {
    Confirm::new(question)
        .with_default(default)
        .prompt()
        .map_err(InitError::PromptError)
}


/// Port of `scripts/branches.json`
pub static OPTIONS_TO_BRANCH: LazyLock<HashMap<Vec<&'static str>, &'static str>> =
    LazyLock::new(|| {
        HashMap::from_iter([
            (Vec::from_iter(["ssr", "posix-sh"]), "default"),
            (Vec::from_iter(["ssg", "posix-sh"]), "ssg"),
            (Vec::from_iter(["ssr", "nushell"]), "nushell"),
            (Vec::from_iter(["ssg", "nushell"]), "ssg-nushell"),
        ])
    });
