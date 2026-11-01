use gloo_net::http::Request;
use serde::Deserialize;
use std::sync::atomic::{AtomicU32, Ordering};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::app::TodoItemData;

static NEXT_ID: AtomicU32 = AtomicU32::new(1);

const GITHUB_API: &str = "https://api.github.com";

#[derive(Debug, Deserialize)]
struct GithubIssue {
    title: String,
    #[serde(default)]
    body: Option<String>,
    state: String,
    html_url: Option<String>,
}

#[derive(Debug, PartialEq)]
struct ParsedUrl {
    owner: String,
    repo: String,
    number: u32,
}

/// Parse a GitHub issue or pull request URL.
///
/// Accepts:
///   - `https://github.com/{owner}/{repo}/issues/{n}`
///   - `https://github.com/{owner}/{repo}/pull/{n}`
///   - The same with `http://`, no scheme, `www.`, query strings, or fragments.
fn parse_github_url(raw: &str) -> Result<ParsedUrl, String> {
    let url = raw.trim();

    let with_scheme = if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else if url.starts_with("github.com/") || url.starts_with("www.github.com/") {
        format!("https://{}", url.trim_start_matches("www."))
    } else {
        return Err("URL must be a github.com issue or pull request".into());
    };

    let rest = with_scheme
        .strip_prefix("http://")
        .or_else(|| with_scheme.strip_prefix("https://"))
        .ok_or_else(|| "URL must start with http(s)://github.com".to_string())?;

    // Strip any trailing query string or fragment before splitting on '/'.
    let host_path = rest.split(['?', '#']).next().unwrap_or(rest);
    let host_path = host_path.trim_start_matches("www.");

    let parts: Vec<&str> = host_path
        .trim_end_matches('/')
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();

    if parts.first() != Some(&"github.com") {
        return Err("URL must be a github.com URL".into());
    }

    // github.com / owner / repo / issues|pull / number
    if parts.len() < 5 {
        return Err("URL must include owner, repo, and issue/PR number".into());
    }

    let owner = parts[1].to_string();
    let repo = parts[2].to_string();
    let kind = parts[3];

    if kind != "issues" && kind != "pull" {
        return Err("URL must point to an issue or pull request".into());
    }

    let number = parts[4]
        .parse::<u32>()
        .map_err(|_| "Invalid issue/PR number".to_string())?;

    Ok(ParsedUrl {
        owner,
        repo,
        number,
    })
}

/// Fetch the metadata for an issue or pull request and turn it into a task.
///
/// The task text is the issue title plus the first non-empty line of the
/// body (when one exists), the completion state mirrors the issue state,
/// and the original link is kept in `github_url`.
async fn fetch_issue(parsed: &ParsedUrl, original_url: &str) -> Result<TodoItemData, String> {
    let api_url = format!(
        "{}/repos/{}/{}/issues/{}",
        GITHUB_API, parsed.owner, parsed.repo, parsed.number
    );

    let response = Request::get(&api_url)
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .map_err(|_| "Could not reach GitHub".to_string())?;

    let status = response.status();
    if status == 404 {
        return Err("Issue or pull request not found".into());
    }
    if !(200..300).contains(&status) {
        return Err(format!("GitHub returned status {}", status));
    }

    let issue: GithubIssue = response
        .json()
        .await
        .map_err(|_| "Could not read issue metadata".to_string())?;

    let description = issue
        .body
        .as_deref()
        .and_then(|body| {
            body.lines()
                .map(str::trim)
                .find(|line| !line.is_empty())
                .map(|line| format!("{} — {}", issue.title, line))
        })
        .unwrap_or_else(|| issue.title.clone());

    Ok(TodoItemData {
        id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
        text: description,
        completed: issue.state == "closed",
        github_url: Some(
            issue
                .html_url
                .unwrap_or_else(|| original_url.to_string()),
        ),
    })
}

#[derive(Properties, PartialEq)]
pub struct GithubLinkInputProps {
    pub on_create: Callback<TodoItemData>,
}

#[function_component(GithubLinkInput)]
pub fn github_link_input(props: &GithubLinkInputProps) -> Html {
    let url = use_state(|| String::new());
    let loading = use_state(|| false);
    let error = use_state(|| Option::<String>::None);

    let submit = {
        let url = url.clone();
        let loading = loading.clone();
        let error = error.clone();
        let on_create = props.on_create.clone();
        Callback::from(move |_: ()| {
            let raw = (*url).trim().to_string();

            if raw.is_empty() {
                error.set(Some("Paste a GitHub issue or pull request URL first".into()));
                return;
            }

            let parsed = match parse_github_url(&raw) {
                Ok(parsed) => parsed,
                Err(message) => {
                    error.set(Some(message));
                    return;
                }
            };

            error.set(None);
            loading.set(true);

            let loading = loading.clone();
            let error = error.clone();
            let on_create = on_create.clone();

            spawn_local(async move {
                match fetch_issue(&parsed, &raw).await {
                    Ok(item) => {
                        on_create.emit(item);
                        error.set(None);
                    }
                    Err(message) => error.set(Some(message)),
                }
                loading.set(false);
            });
        })
    };

    let on_input = {
        let url = url.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target().unwrap().unchecked_into();
            url.set(input.value());
        })
    };

    let on_keydown = {
        let submit = submit.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default();
                submit.emit(());
            }
        })
    };

    let on_add = {
        let submit = submit.clone();
        Callback::from(move |_: MouseEvent| submit.emit(()))
    };

    html! {
        <div class="github-link-input">
            <div class="composer-row">
                <div class="input-wrapper">
                    <span class="input-icon" aria-hidden="true">
                        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path>
                            <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path>
                        </svg>
                    </span>
                    <input
                        class="composer-input"
                        type="text"
                        placeholder="https://github.com/owner/repo/issues/123"
                        value={(*url).clone()}
                        oninput={on_input}
                        onkeydown={on_keydown}
                        disabled={*loading}
                        aria-label="GitHub issue or pull request URL"
                    />
                </div>
                <button class="btn btn-primary" onclick={on_add} disabled={*loading}>
                    { if *loading { "Fetching…" } else { "Add task" } }
                </button>
            </div>
            if let Some(message) = &*error {
                <p class="github-link-error" role="alert">{ message.clone() }</p>
            }
        </div>
    }
}
