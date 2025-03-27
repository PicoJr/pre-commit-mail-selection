use std::{collections::HashSet, path::Path, vec};

use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use git2::{Config, Repository};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct GitEmails {
    emails: Vec<String>,
}

#[derive(Default)]
struct GitEmailsConfig {
    local_email: Option<String>,  // from current repo, .git/config
    global_email: Option<String>, // from e.g. ~/.gitconfig
    local_emails: Vec<String>,    // from .git-emails.toml if any
}

/** List available emails
* * from local config if any
* * from global config if any
* * from local .git-emails.toml if any
*/
fn get_config_emails() -> anyhow::Result<GitEmailsConfig> {
    let mut git_emails_config = GitEmailsConfig::default();
    // 0 try reading email from local config
    if let Ok(repo) = Repository::open(".") {
        if let Ok(local_config) = repo.config() {
            if let Ok(user_mail) = local_config.get_string("user.email") {
                git_emails_config.local_email = Some(user_mail);
            }
        }
    }
    // 1 try reading emails from global config
    if let Ok(global_config) = Config::open_default() {
        if let Ok(user_mail) = global_config.get_string("user.email") {
            git_emails_config.global_email = Some(user_mail);
        }
    }
    // 2 try reading emails from local .git-emails.toml file
    let email_config_path = Path::new(".git-emails.toml");
    if email_config_path.exists() {
        let content: String = std::fs::read_to_string(email_config_path)?;
        let git_emails: GitEmails = toml::from_str(content.as_str())?;
        git_emails_config.local_emails = git_emails.emails;
    }
    Ok(git_emails_config)
}

/** Returns an ordered list of emails with no duplicates
*
* 0. email as declared locally in current repository in .git/config if any
* 1. email as configured globally if any
* 2. emails declared in .git-emails.toml if any
*/
fn get_unique_emails_ordered(emails: GitEmailsConfig) -> Vec<String> {
    let mut all_emails: Vec<String> = vec![];
    let mut unique_emails: HashSet<String> = HashSet::new();
    match (emails.local_email, emails.global_email) {
        (Some(email), None) | (None, Some(email)) => {
            all_emails.push(email.clone());
            unique_emails.insert(email.clone());
        }
        (Some(local_email), Some(global_email)) => {
            all_emails.push(local_email.clone());
            unique_emails.insert(local_email.clone());
            if local_email != global_email {
                all_emails.push(global_email.clone());
                unique_emails.insert(global_email.clone());
            }
        }
        _ => {}
    }
    for email in emails.local_emails.iter() {
        if !unique_emails.contains(email) {
            all_emails.push(email.clone());
            unique_emails.insert(email.clone());
        }
    }
    all_emails
}

fn main() -> anyhow::Result<()> {
    let emails = get_config_emails()?;
    let unique_emails_ordered = get_unique_emails_ordered(emails);

    // do not bother if we do not have at least 2 emails to choose from
    if unique_emails_ordered.len() > 1 {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Please select an email for commit")
            .items(&unique_emails_ordered)
            .default(0) // default to the email configured in .git/config
            .interact()?;
        let selected_email = unique_emails_ordered
            .get(selection)
            .expect("selection should be valid");
        // try writing selected email to local config
        let repo = Repository::open(".")?;
        let mut local_config = repo.config()?;
        local_config.set_str("user.email", selected_email)?;
    }
    Ok(())
}
