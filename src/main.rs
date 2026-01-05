use clap::Parser;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// Default remote URL for pages (can be overridden with env var IDK_REMOTE)
const DEFAULT_REMOTE: &str = "https://raw.githubusercontent.com/anthropics/idk/main/pages";

/// idk - man pages for agents
///
/// Like tldr, but with semantic search and agent-optimized output.
#[derive(Parser)]
#[command(name = "idk")]
#[command(about = "man pages for agents - semantic search for commands")]
struct Cli {
    /// Command name or search query
    query: Vec<String>,

    /// Output as JSON (machine-readable)
    #[arg(short, long)]
    json: bool,

    /// List all available pages
    #[arg(short, long)]
    list: bool,

    /// Force search mode (even for single words)
    #[arg(short, long)]
    search: bool,

    /// Sync pages from remote
    #[arg(long)]
    sync: bool,

    /// Clear local cache
    #[arg(long)]
    clear_cache: bool,

    /// Fetch from remote even if cached locally
    #[arg(short, long)]
    update: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Page {
    name: String,
    summary: String,
    see_also: Vec<String>,
    keywords: Vec<String>,
    examples: Vec<Example>,
    flags: Vec<Flag>,
    exit_codes: Vec<ExitCode>,
    errors: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Example {
    description: String,
    command: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Flag {
    flag: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExitCode {
    code: i32,
    meaning: String,
}

#[derive(Debug, Serialize)]
struct SearchResult {
    name: String,
    summary: String,
    score: i32,
}

/// Index file that lists all available pages in the remote
#[derive(Debug, Serialize, Deserialize, Default)]
struct PageIndex {
    pages: Vec<String>,
}

fn get_cache_dir() -> PathBuf {
    if let Some(cache_dir) = dirs::cache_dir() {
        cache_dir.join("idk").join("pages")
    } else {
        PathBuf::from(".cache/idk/pages")
    }
}

fn get_pages_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    // Priority 1: Pages next to binary
    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            let pages = exe_dir.join("pages");
            if pages.exists() {
                dirs.push(pages);
            }
        }
    }

    // Priority 2: Pages in current directory
    if let Ok(cwd) = std::env::current_dir() {
        let pages = cwd.join("pages");
        if pages.exists() {
            dirs.push(pages);
        }
    }

    // Priority 3: Cache directory
    let cache = get_cache_dir();
    if cache.exists() {
        dirs.push(cache);
    }

    dirs
}

fn get_remote_url() -> String {
    std::env::var("IDK_REMOTE").unwrap_or_else(|_| DEFAULT_REMOTE.to_string())
}

fn fetch_remote_page(command: &str) -> Option<String> {
    let url = format!("{}/{}.md", get_remote_url(), command);

    match ureq::get(&url).call() {
        Ok(response) => response.into_string().ok(),
        Err(_) => None,
    }
}

fn fetch_page_index() -> Option<PageIndex> {
    let url = format!("{}/index.json", get_remote_url());

    match ureq::get(&url).call() {
        Ok(response) => {
            let body = response.into_string().ok()?;
            serde_json::from_str(&body).ok()
        }
        Err(_) => None,
    }
}

fn cache_page(command: &str, content: &str) -> std::io::Result<()> {
    let cache_dir = get_cache_dir();
    fs::create_dir_all(&cache_dir)?;
    let path = cache_dir.join(format!("{}.md", command));
    fs::write(path, content)
}

fn parse_page(content: &str) -> Page {
    let mut page = Page::default();
    let mut current_section: Option<&str> = None;
    let mut current_example: Option<Example> = None;

    for line in content.lines() {
        // Command name
        if let Some(name) = line.strip_prefix("# ") {
            page.name = name.trim().to_string();
            continue;
        }

        // Summary/metadata lines
        if let Some(text) = line.strip_prefix("> ") {
            let text = text.trim();
            if let Some(rest) = text.strip_prefix("See also:") {
                page.see_also = rest.split(',')
                    .map(|s| s.trim().trim_matches('`').to_string())
                    .collect();
            } else if let Some(rest) = text.strip_prefix("Keywords:") {
                page.keywords = rest.split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
            } else {
                if !page.summary.is_empty() {
                    page.summary.push(' ');
                }
                page.summary.push_str(text);
            }
            continue;
        }

        // Section headers
        if let Some(section) = line.strip_prefix("## ") {
            current_section = Some(match section.trim().to_lowercase().as_str() {
                "flags" => "flags",
                "exit codes" => "exit_codes",
                "common errors" => "errors",
                _ => "examples",
            });
            continue;
        }

        // Example description
        if line.starts_with("- ") && !matches!(current_section, Some("flags") | Some("exit_codes") | Some("errors")) {
            // Save previous example if any
            if let Some(ex) = current_example.take() {
                page.examples.push(ex);
            }
            current_example = Some(Example {
                description: line[2..].trim().to_string(),
                command: String::new(),
            });
            continue;
        }

        // Example command
        if line.starts_with('`') && line.ends_with('`') && line.len() > 2 {
            if let Some(ref mut ex) = current_example {
                ex.command = line[1..line.len()-1].to_string();
                page.examples.push(current_example.take().unwrap());
            }
            continue;
        }

        // Flag line
        if current_section == Some("flags") && line.starts_with("- ") {
            if let Some(rest) = line.strip_prefix("- `") {
                if let Some(idx) = rest.find("`:") {
                    let flag = rest[..idx].to_string();
                    let desc = rest[idx+2..].trim().to_string();
                    page.flags.push(Flag { flag, description: desc });
                }
            }
            continue;
        }

        // Exit code line
        if current_section == Some("exit_codes") && line.starts_with("- ") {
            if let Some(rest) = line.strip_prefix("- `") {
                if let Some(idx) = rest.find("`:") {
                    if let Ok(code) = rest[..idx].parse() {
                        let meaning = rest[idx+2..].trim().to_string();
                        page.exit_codes.push(ExitCode { code, meaning });
                    }
                }
            }
            continue;
        }

        // Error line
        if current_section == Some("errors") && line.starts_with("- ") {
            page.errors.push(line[2..].trim().to_string());
        }
    }

    // Don't forget last example
    if let Some(ex) = current_example {
        page.examples.push(ex);
    }

    page
}

fn load_page(command: &str, fetch_if_missing: bool) -> Option<Page> {
    // Check all local directories first
    for dir in get_pages_dirs() {
        let path = dir.join(format!("{}.md", command));
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                return Some(parse_page(&content));
            }
        }
    }

    // Try fetching from remote
    if fetch_if_missing {
        if let Some(content) = fetch_remote_page(command) {
            // Cache it locally
            let _ = cache_page(command, &content);
            return Some(parse_page(&content));
        }
    }

    None
}

fn search_pages(query: &str) -> Vec<(Page, i32)> {
    let query_lower = query.to_lowercase();
    let query_words: Vec<&str> = query_lower.split_whitespace().collect();
    let mut results: Vec<(Page, i32)> = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    for dir in get_pages_dirs() {
        let entries = match fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => continue,
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "md") {
                if let Ok(content) = fs::read_to_string(&path) {
                    let page = parse_page(&content);

                    // Skip duplicates
                    if seen.contains(&page.name) {
                        continue;
                    }
                    seen.insert(page.name.clone());

                    let mut score = 0;

                    // Exact command name match
                    let name_lower = page.name.to_lowercase();
                    if query_words.iter().any(|&w| w == name_lower) {
                        score += 100;
                    }

                    // Keyword matches
                    let page_keywords: Vec<String> = page.keywords.iter()
                        .map(|k| k.to_lowercase())
                        .collect();
                    for word in &query_words {
                        if page_keywords.iter().any(|k| k == *word || k.contains(*word)) {
                            score += 20;
                        }
                    }

                    // Summary contains query words
                    let summary_lower = page.summary.to_lowercase();
                    for word in &query_words {
                        if summary_lower.contains(*word) {
                            score += 10;
                        }
                    }

                    // Example descriptions contain query words
                    for example in &page.examples {
                        let desc_lower = example.description.to_lowercase();
                        for word in &query_words {
                            if desc_lower.contains(*word) {
                                score += 5;
                            }
                        }
                    }

                    if score > 0 {
                        results.push((page, score));
                    }
                }
            }
        }
    }

    results.sort_by(|a, b| b.1.cmp(&a.1));
    results
}

fn list_pages() -> Vec<String> {
    let mut pages = std::collections::HashSet::new();

    for dir in get_pages_dirs() {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |e| e == "md") {
                    if let Some(stem) = path.file_stem() {
                        pages.insert(stem.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    let mut pages: Vec<_> = pages.into_iter().collect();
    pages.sort();
    pages
}

fn sync_pages() {
    println!("{}", "Syncing pages from remote...".cyan());

    let Some(index) = fetch_page_index() else {
        println!("{}", "Failed to fetch page index. Make sure the remote is available.".red());
        println!("Remote URL: {}", get_remote_url().dimmed());
        return;
    };

    let cache_dir = get_cache_dir();
    if let Err(e) = fs::create_dir_all(&cache_dir) {
        println!("{}: {}", "Failed to create cache directory".red(), e);
        return;
    }

    let total = index.pages.len();
    let mut synced = 0;
    let mut failed = 0;

    for page_name in &index.pages {
        if let Some(content) = fetch_remote_page(page_name) {
            if cache_page(page_name, &content).is_ok() {
                synced += 1;
                println!("  {} {}", "✓".green(), page_name);
            } else {
                failed += 1;
                println!("  {} {} (cache write failed)", "✗".red(), page_name);
            }
        } else {
            failed += 1;
            println!("  {} {} (fetch failed)", "✗".red(), page_name);
        }
    }

    println!();
    println!("Synced {}/{} pages", synced.to_string().green(), total);
    if failed > 0 {
        println!("Failed: {}", failed.to_string().red());
    }
    println!("Cache: {}", cache_dir.display());
}

fn clear_cache() {
    let cache_dir = get_cache_dir();
    if cache_dir.exists() {
        if let Err(e) = fs::remove_dir_all(&cache_dir) {
            println!("{}: {}", "Failed to clear cache".red(), e);
        } else {
            println!("{} {}", "Cleared cache:".green(), cache_dir.display());
        }
    } else {
        println!("Cache directory doesn't exist: {}", cache_dir.display());
    }
}

fn format_page(page: &Page, use_json: bool) -> String {
    if use_json {
        return serde_json::to_string_pretty(page).unwrap_or_default();
    }

    let mut output = Vec::new();

    // Header
    output.push(page.name.bold().cyan().to_string());
    output.push(page.summary.dimmed().to_string());

    if !page.see_also.is_empty() {
        output.push(format!("See also: {}", page.see_also.join(", ")).dimmed().to_string());
    }

    output.push(String::new());

    // Examples
    for ex in &page.examples {
        output.push(format!("  {}", ex.description.yellow()));
        output.push(format!("  {}", ex.command.green()));
        output.push(String::new());
    }

    // Flags
    if !page.flags.is_empty() {
        output.push("Common Flags:".bold().to_string());
        for flag in &page.flags {
            output.push(format!("  {}  {}", flag.flag.cyan(), flag.description));
        }
        output.push(String::new());
    }

    // Exit codes
    if !page.exit_codes.is_empty() {
        output.push("Exit Codes:".bold().to_string());
        for ec in &page.exit_codes {
            output.push(format!("  {}  {}", ec.code.to_string().cyan(), ec.meaning));
        }
        output.push(String::new());
    }

    // Errors
    if !page.errors.is_empty() {
        output.push("Common Errors:".bold().to_string());
        for err in &page.errors {
            output.push(format!("  {} {}", "•".red(), err));
        }
        output.push(String::new());
    }

    output.join("\n")
}

fn format_search_results(results: &[(Page, i32)], use_json: bool) -> String {
    if use_json {
        let json_results: Vec<SearchResult> = results.iter()
            .take(10)
            .map(|(page, score)| SearchResult {
                name: page.name.clone(),
                summary: page.summary.clone(),
                score: *score,
            })
            .collect();
        return serde_json::to_string_pretty(&json_results).unwrap_or_default();
    }

    if results.is_empty() {
        return format!(
            "{}\nTry different keywords or check available pages with: idk --list",
            "No matching commands found.".yellow()
        );
    }

    let mut output = vec!["Matching commands:".bold().to_string(), String::new()];

    for (page, _score) in results.iter().take(10) {
        output.push(format!("  {}", page.name.bold().cyan()));
        let summary_preview: String = page.summary.chars().take(80).collect();
        let suffix = if page.summary.len() > 80 { "..." } else { "" };
        output.push(format!("  {}{}", summary_preview.dimmed(), suffix.dimmed()));
        output.push(String::new());
    }

    output.join("\n")
}

fn main() {
    let cli = Cli::parse();

    if cli.clear_cache {
        clear_cache();
        return;
    }

    if cli.sync {
        sync_pages();
        return;
    }

    if cli.list {
        let pages = list_pages();
        if pages.is_empty() {
            println!("No pages available yet.");
            println!();
            println!("Try:");
            println!("  idk --sync     Sync pages from remote");
            println!();
            println!("Or set IDK_REMOTE to your pages URL");
        } else {
            println!("Available commands:");
            for page in pages {
                println!("  {}", page);
            }
        }
        return;
    }

    if cli.query.is_empty() {
        println!("{}", "idk - man pages for agents".bold());
        println!();
        println!("Usage:");
        println!("  idk <command>      Show help for a command");
        println!("  idk <query...>     Search for commands");
        println!("  idk --list         List all pages");
        println!("  idk --json <cmd>   Output as JSON");
        println!("  idk --sync         Sync pages from remote");
        println!("  idk --update <cmd> Force fetch from remote");
        return;
    }

    let query = cli.query.join(" ");

    // If single word and not forced search, try direct lookup first
    if cli.query.len() == 1 && !cli.search {
        // If update flag, fetch from remote first
        if cli.update {
            if let Some(content) = fetch_remote_page(&cli.query[0]) {
                let _ = cache_page(&cli.query[0], &content);
                println!("{}", format_page(&parse_page(&content), cli.json));
                return;
            }
        }

        // Try loading (with remote fetch fallback)
        if let Some(page) = load_page(&cli.query[0], true) {
            println!("{}", format_page(&page, cli.json));
            return;
        }
    }

    // Search mode
    let results = search_pages(&query);

    // If search found exact match, show it directly
    if !results.is_empty() && results[0].1 >= 100 {
        println!("{}", format_page(&results[0].0, cli.json));
    } else {
        println!("{}", format_search_results(&results, cli.json));
    }
}
