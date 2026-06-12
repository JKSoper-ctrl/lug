use std::io::{self, Write};
use std::process::Command;

fn main() {
    if !check_command("git", &["--version"]) {
        eprintln!("Error: git is not installed or not in PATH.");
        std::process::exit(1);
    }
    if !check_command("gh", &["--version"]) {
        eprintln!("Error: gh (GitHub CLI) is not installed or not in PATH.");
        eprintln!("Please install it from https://cli.github.com/ and run 'gh auth login'.");
        std::process::exit(1);
    }

    println!("Select an option:");
    println!("1) Build and publish a new Rust project to GitHub");
    println!("2) Push changes with Conventional Commits");
    print!("Enter 1 or 2: ");
    io::stdout().flush().unwrap();
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    match choice.trim() {
        "1" => build_and_publish(),
        "2" => push_conventional_commits(),
        _ => {
            eprintln!("Invalid choice.");
            std::process::exit(1);
        }
    }
}

fn check_command(cmd: &str, args: &[&str]) -> bool {
    Command::new(cmd).args(args).output().is_ok()
}

fn build_and_publish() {
    println!("\n--- Build and publish new Rust project ---");
    let name = prompt_non_empty("Project name: ");
    let public = prompt_public_private();

    println!("Creating new Rust binary project '{}'...", name);
    let status = Command::new("cargo")
        .args(&["new", "--bin", &name])
        .status()
        .expect("Failed to run cargo new");
    if !status.success() {
        eprintln!("Error: cargo new failed.");
        std::process::exit(1);
    }

    // Change working directory to the new project
    std::env::set_current_dir(&name).unwrap_or_else(|_| {
        eprintln!("Failed to enter project directory '{}'", name);
        std::process::exit(1);
    });

    // Git is already initialised by cargo new by default, but ensure it's there
    if !std::path::Path::new(".git").exists() {
        Command::new("git").args(&["init"]).status().unwrap();
    }

    // Create GitHub repository via gh
    let visibility = if public { "--public" } else { "--private" };
    let gh_create_status = Command::new("gh")
        .args(&["repo", "create", &name, visibility, "--source=.", "--remote=origin", "--push"])
        .status()
        .expect("Failed to run gh repo create");
    if !gh_create_status.success() {
        eprintln!("Error: gh repo create failed. Are you logged in? (gh auth status)");
        std::process::exit(1);
    }

    // The initial commit (cargo new might have created a commit already, but we force a known message)
    // We'll add all and commit with "chore: initial commit"
    let add_status = Command::new("git").args(&["add", "."]).status().unwrap();
    if !add_status.success() {
        eprintln!("Warning: git add . failed");
    }
    let commit_status = Command::new("git")
        .args(&["commit", "-m", "chore: initial commit"])
        .status()
        .unwrap();
    if !commit_status.success() {
        eprintln!("Warning: git commit failed – maybe nothing to commit?");
    }
    let push_status = Command::new("git").args(&["push", "-u", "origin", "HEAD"]).status().unwrap();
    if push_status.success() {
        println!("✅ Project '{}' successfully published to GitHub as {}.", name, visibility);
    } else {
        eprintln!("Error: git push failed. You may need to push manually.");
        std::process::exit(1);
    }
}

fn push_conventional_commits() {
    println!("\n--- Push changes with Conventional Commits ---");
    println!("Types:\n  feat, fix, refactor, perf, style, test, docs, build, ops, chore");
    let commit_type = prompt_non_empty("Commit type: ");
    let scope = prompt_optional("Optional scope (leave blank to omit): ");
    let description = prompt_non_empty("Short description (required): ");
    let body = prompt_optional("Optional body (blank line to finish):\n");
    let footer = prompt_optional("Optional footer (blank line to finish):\n");

    let mut message = String::new();
    message.push_str(&commit_type);
    if !scope.is_empty() {
        message.push('(');
        message.push_str(&scope);
        message.push(')');
    }
    message.push_str(": ");
    message.push_str(&description);

    if !body.is_empty() {
        message.push_str("\n\n");
        message.push_str(&body);
    }
    if !footer.is_empty() {
        if body.is_empty() {
            message.push_str("\n\n");
        } else {
            message.push('\n');
        }
        message.push_str(&footer);
    }

    println!("\nPrepared commit message:\n---\n{}\n---", message);

    // Stage all changes (you can modify to add specific files)
    let add_status = Command::new("git").args(&["add", "-A"]).status().unwrap();
    if !add_status.success() {
        eprintln!("Error: git add -A failed.");
        std::process::exit(1);
    }

    let commit_status = Command::new("git")
        .args(&["commit", "-m", &message])
        .status()
        .unwrap();
    if !commit_status.success() {
        eprintln!("Error: git commit failed.");
        std::process::exit(1);
    }

    let push_status = Command::new("git").args(&["push"]).status().unwrap();
    if push_status.success() {
        println!("✅ Changes pushed successfully.");
    } else {
        eprintln!("Error: git push failed.");
        std::process::exit(1);
    }
}

fn prompt_non_empty(prompt: &str) -> String {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
        println!("This field cannot be empty.");
    }
}

fn prompt_optional(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim();
    if trimmed.is_empty() {
        String::new()
    } else {
        // For multiline body/footer we only read one line; a real implementation could read until blank line.
        // This simplified version accepts a single line.
        trimmed.to_string()
    }
}

fn prompt_public_private() -> bool {
    loop {
        print!("Public or private? (publ/priv): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_lowercase().as_str() {
            "publ" | "public" | "y" | "yes" => return true,
            "priv" | "private" | "n" | "no" => return false,
            _ => println!("Please answer 'public' or 'private'."),
        }
    }
}