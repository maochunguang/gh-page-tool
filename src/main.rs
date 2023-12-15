use clap::{arg, Command};
mod publish_lib;
fn main() {
    let matches = Command::new("gh_page")
        .version("1.0")
        .author("tommy")
        .about("Implements gh-pages like functionality")
        .subcommand(
            Command::new("publish")
                .about("Publishes a directory to a gh-pages branch")
                .arg(arg!(-d --directory <DIRECTORY> "Sets the directory to publish"))
                .arg(arg!(-b --branch <BRANCH> "Sets the branch to publish to"))
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("publish") {
        let default_dir = String::from("book");
        let default_branch = String::from("gh-pages");
        let directory = matches.get_one::<String>("directory").unwrap_or(&default_dir);
        let branch = matches.get_one::<String>("branch").unwrap_or(&default_branch);
        publish_lib::publish(directory, branch);
    }
}