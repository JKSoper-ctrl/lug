//To Do:
//- Automatically create README.md file on project creation
//- Allow a custom starting verison (leave blank to remain 0.1.0)
//- Version control using Semantic Versioning 2.0.0
//- Allow whole chain to be entered in one
//- Add description when gh repo being created
//- Add file address to the summary before confirming a new project
//- Verify if GitHub logged in
//- Check correct gh and git versions
//- Add handling for incorrect values

use std::io;
use std::process::Command;

fn main() {
    println!("---");
    println!("Running lug - please select an option and press enter to confirm :)");
    println!("[1] Build and publish a new Rust project to GitHub");
    println!("[2] Push changes with Conventional Commits");
    println!("[0] Check dependancies");

    let mut action_selection = String::new();
    io::stdin().read_line(&mut action_selection).unwrap();

    if action_selection.trim() == "1" { //creating new project
        println!("---");
        println!("Creating new project - type project name and press enter to confirm");

        let mut project_name = String::new();
        io::stdin().read_line(&mut project_name).unwrap();
        let short_project_path = "./".to_owned() + &project_name.trim();

        println!("---");
        println!("Will this project be public or private? - type number and press enter to conifrm");
        println!("[1] Public");
        println!("[0] Private");
        
        let mut project_visability = String::new();
        io::stdin().read_line(&mut project_visability).unwrap();
        let mut project_visability_arg = "";

        if project_visability.trim() == "1" { //public
            project_visability_arg = "--public";
        } else if project_visability.trim() == "0" { //private
            project_visability_arg = "--private";
        } else {
            eprintln!("Incorrect input!");
        }
        
        println!("---");
        println!("Do you have a decription for this project? If so write below and press enter to continue (can leave blank)");

        let mut project_description = String::new();
        io::stdin().read_line(&mut project_description).unwrap();

        println!("---");
        println!("Are you happy to create and publish the following project? Enter you answer and press enter to confirm");
        println!();
        println!("Project name: {}", project_name.trim());
        println!("Repo visability: {}", project_visability_arg);
        println!("Project description: {}", project_description.trim());
        println!();
        println!("[1] Yes");
        println!("[0] No");

        let mut happy_to_confirm = String::new();
        io::stdin().read_line(&mut happy_to_confirm).unwrap();

        println!("---");

        if happy_to_confirm.trim() == "1" { //happy to confirm
            //creating new cargo project
            Command::new("cargo")
                .args(&["new", project_name.trim()])
                .status()
                .expect("Error using `cargo new` to create new Rust project in local files");
            
            println!("Created Cargo project!");

            //adding git
            Command::new("git")
                .args(&["add", "."])
                .current_dir(&short_project_path)
                .status()
                .expect("Error adding git");

            println!("Added folder to Git!");

            //creating first git commit
            Command::new("git")
                .args(&["commit", "-m", "chore: initial commit"])
                .current_dir(&short_project_path)
                .status()
                .expect("Failed to create first commit");

            println!("Created first commit!");

            //pushing to GitHub
            Command::new("gh")
                .args(&["repo", "create", project_name.trim(), project_visability_arg, "--source=.", "--remote=origin", "--push"])
                .current_dir(&short_project_path)
                .status()
                .expect("Error creating new repo");

            println!("Published onto GitHub!");
        } else {
            println!("Creation cancelled");
        }


    } else if action_selection.trim() == "2" { //pushing changes to git
        //adding git
        Command::new("git")
            .args(&["add", "."])
            .status()
            .expect("Error adding git");
        
        println!("---");
        println!("Changes that would be made:");
        println!();

        Command::new("git")
            .args(&["diff", "HEAD"])
            .status()
            .expect("couldn't see git changes");

        println!();
        println!("---");
        println!("What type of change is it?");
        println!("[1] feat - Commits that add, adjust or remove a feature");
        println!("[2] fix - Commits that fix an API or UI bug");
        println!("[3] refactor - Rewrite/restructure code without altering behavior");
        println!("[4] perf - Performance improvement (special refactor)");
        println!("[5] style - Code style changes (formatting, semicolons, etc.)");
        println!("[6] test - Add or correct tests");
        println!("[7] docs - Documentation only");
        println!("[8] build - Build tools, dependencies, project version");
        println!("[9] ops - Infrastructure, CI/CD, deployment, monitoring");
        println!("[0] chore - Initial commit, .gitignore, etc.");

        let mut change_selection = String::new();
        io::stdin().read_line(&mut change_selection).unwrap();

        let mut change_type = String::new();
        if change_selection.trim() == "1" {
            change_type = "feat: ".to_string()
        } else if change_selection.trim() == "2" {
            change_type = "fix: ".to_string()
        } else if change_selection.trim() == "3" {
            change_type = "refactor: ".to_string()
        } else if change_selection.trim() == "4" {
            change_type = "perf: ".to_string()
        } else if change_selection.trim() == "5" {
            change_type = "style: ".to_string()
        } else if change_selection.trim() == "6" {
            change_type = "test: ".to_string()
        } else if change_selection.trim() == "7" {
            change_type = "docs: ".to_string()
        } else if change_selection.trim() == "8" {
            change_type = "build: ".to_string()
        } else if change_selection.trim() == "9" {
            change_type = "ops: ".to_string()
        } else { //if change_selection == 0
            change_type = "chore: ".to_string()
        }

        println!("---");
        println!("Write a description:");
        
        let mut change_description = String::new();
        io::stdin().read_line(&mut change_description).unwrap();

        let commit_message = change_type + &change_description.trim();

        println!("---");
        println!("Are you fine to push the following change?");
        println!();
        println!("{}", commit_message.trim());
        let output = Command::new("git")
            .args(&["diff", "--shortstat", "HEAD"])
            .output()
            .expect("failed to summarise the git changes");
        print!("{}", String::from_utf8_lossy(&output.stdout));
        println!();
        println!("[1] Yes");
        println!("[0] No");

        let mut happy_to_confirm = String::new();
        io::stdin().read_line(&mut happy_to_confirm).unwrap();

        println!("---");

        if happy_to_confirm.trim() == "1" {
            Command::new("git")
                .args(&["add","."])
                .status()
                .expect("failed to add to git");

            Command::new("git")
                .args(&["commit", "-m", commit_message.trim()])
                .status()
                .expect("failed to add to git");

            Command::new("git")
                .args(&["push", "origin", "HEAD"])
                .status()
                .expect("failed to push to GitHub");
        }

    } else if action_selection.trim() == "0" { //checking dependancies
        println!("---");
        println!("Checking Cargo:");
        Command::new("cargo").arg("--version").status().expect("Failed to find Cargo");
        println!();
        println!("Checking Git:");
        Command::new("git").arg("--version").status().expect("Failed to find Git");
        println!();
        println!("Checking GitHub:");
        Command::new("gh").arg("--version").status().expect("Failed to find GitHub");
        println!();
        println!("All dependancies installed!")

    } else {
        eprintln!("Incorrect input!");
    }
    println!("---");

}