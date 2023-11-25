use std::env; 
use std::fs;
use std::path::Path;
use url::{Url, ParseError};
use git2::Repository;
use git2::{Cred, Error, RemoteCallbacks};

struct Stagit {
    url: String,
    repo: Repository,
    repo_url: String,
    repo_head: String,
    repo_description: String,
    repo_license: String,
    repo_readme: String,
    //repo_submodules: String,
}

pub fn git_credentials_callback(
    _user: &str,
    _user_from_url: Option<&str>,
    _cred: git2::CredentialType,
) -> Result<git2::Cred, git2::Error> {
    let user = _user_from_url.unwrap_or("git");

    if _cred.contains(git2::CredentialType::USERNAME) {
        return git2::Cred::username(user);
    }
    let private_ssh_path = Path::new("~/.ssh/id_rsa");
    let public_ssh_path = Path::new("~/.ssh/id_rsa.pub");
    git2::Cred::ssh_key(user, Some(Path::new(public_ssh_path)), Path::new(private_ssh_path), None)
}

fn clone_repo(url: String) -> Result<git2::Repository, git2::Error> {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
            None,
        )
    });

    // Prepare fetch options.
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    // Prepare builder.
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);
    
    builder.clone(
        "git@github.com:mskorczak/portfolio.git",
        Path::new("./repos/portfolio.git"))
}

fn stagit_read_file(path: String) -> String {
    fs::read_to_string(path).unwrap()
}

fn generate(repo: Repository, url: String) {
    let stagit = Stagit {
        url: url,
        repo: repo,
        repo_url: stagit_read_file("./repos/porfolio.git/.git/url".to_string()),
        repo_head: stagit_read_file("./repos/portfolio.git/.git/HEAD".to_string()),
        repo_description: stagit_read_file("./repos/portfolio.git/description".to_string()),
        repo_license: "NOLICENSE".to_string(),
        repo_readme: stagit_read_file("./repos/portfolio.git/README.md".to_string()),
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let repos = args.split_first().unwrap().0;

    let url = "git@github.com:mskorczak/portfolio.git";
    match clone_repo(url.to_string()) {
        Ok(repo) => generate(repo, url.to_string()),
        Err(e) => panic!("Download of {} failed.", e),
    };

}
