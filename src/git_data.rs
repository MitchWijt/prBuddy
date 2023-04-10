use shutil::pipe;

#[derive(PartialEq)]
#[derive(Debug)]
struct Command<'a>(Vec<Vec<&'a str>>);

impl Command<'_> {
    pub fn call(self) -> Result<String, &'static str> {
        let output = pipe(self.0);
        match output {
            Ok(output_str) => Ok(output_str.replace("\n", "")),
            Err(_e) => Err("Failed to execute command")
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct GitData {
    pub branch: String,
    pub main_branch: String,
    pub repo_name: String,
    pub owner: String,
}

impl GitData {
    pub fn build() -> Result<Self, &'static str> {
        let git_command_args = GitCommandArgs::build();
        Self::from_command_args(git_command_args)
    }

    fn from_command_args(command_args: GitCommandArgs) -> Result<Self, &'static str> {
        let branch_name = command_args.branch.call()?;
        let main_branch_name = command_args.main_branch.call()?;
        let remote_url = command_args.remote_url.call()?;

        let url_data = GitUrlData::build(remote_url);

        return Ok(GitData {
            branch: branch_name,
            main_branch: main_branch_name,
            repo_name: url_data.repo_name,
            owner: url_data.owner
        })
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
struct GitCommandArgs<'a> {
    branch: Command<'a>,
    main_branch: Command<'a>,
    remote_url: Command<'a>,
}

impl GitCommandArgs<'_> {
    pub fn build<'a>() -> GitCommandArgs<'a> {
        return GitCommandArgs {
            branch: Command(vec![vec!["git", "rev-parse", "--abbrev-ref", "HEAD"]]),
            main_branch: Command(vec![vec!["git", "remote", "show", "origin"], vec!["awk", r#"/HEAD branch/ {print $NF}"#]]),
            remote_url: Command(vec![vec!["git", "config", "--get", "remote.origin.url"]]),
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
struct GitUrlData {
    repo_name: String,
    owner: String,
}

impl GitUrlData {
    pub fn build(url: String) -> Self {
        return if url.contains("git@") {
            Self::from_ssh(url)
        } else {
            Self::from_https(url)
        }
    }

    fn from_https(url: String) -> Self {
        let split = url.split("/");
        let vector: Vec<&str> = split.collect();

        let owner = *vector.get(3).unwrap();
        let repo_name = *vector.get(4).unwrap();

        return GitUrlData {
            owner: String::from(owner),
            repo_name: String::from(repo_name.replace(".git", ""))
        }
    }

    fn from_ssh(url: String) -> Self {
        let split = url.split("/");
        let vector: Vec<&str> = split.collect();

        let owner = *vector.get(0).unwrap();
        let repo_name = *vector.get(1).unwrap();

        return GitUrlData {
            owner: String::from(owner.replace("git@github.com:", "")),
            repo_name: String::from(repo_name.replace(".git", ""))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn git_data_from_command_args_returns_data() {
        //given
        let git_command_args = GitCommandArgs::build();

        //when
        let git_data = GitData::from_command_args(git_command_args).unwrap();

        //then
        let expected = GitData {
            branch: String::from("feature-refactor-code"),
            main_branch: String::from("main"),
            repo_name: String::from("prBuddy"),
            owner: String::from("MitchWijt"),
        };

        assert_eq!(git_data.main_branch, expected.main_branch);
        assert_eq!(git_data.repo_name, expected.repo_name);
        assert_eq!(git_data.owner, expected.owner);
    }

    #[test]
    fn git_command_args_build_returns_correct_args() {
        //given
        let command_args = GitCommandArgs::build();

        let expected: GitCommandArgs = GitCommandArgs {
            branch: Command(vec![vec!["git", "rev-parse", "--abbrev-ref", "HEAD"]]),
            main_branch: Command(vec![vec!["git", "remote", "show", "origin"], vec!["awk", r#"/HEAD branch/ {print $NF}"#]]),
            remote_url: Command(vec![vec!["git", "config", "--get", "remote.origin.url"]]),
        };

        //then
        assert_eq!(command_args, expected);
    }

    #[test]
    fn git_url_data_from_ssh_returns_correct_git_data() {
        //given
        let remote_url_ssh = String::from("git@github.com:MitchWijt/prBuddy.git");

        //when
        let url_data = GitUrlData::build(remote_url_ssh);

        //then
        let expected: GitUrlData = GitUrlData {
            owner: String::from("MitchWijt"),
            repo_name: String::from("prBuddy"),
        };

        assert_eq!(url_data, expected);
    }

    #[test]
    fn git_url_data_from_https_returns_correct_git_data() {
        //given
        let remote_url_ssh = String::from("https://github.com/MitchWijt/prBuddy.git");

        //when
        let url_data = GitUrlData::build(remote_url_ssh);

        //then
        let expected: GitUrlData = GitUrlData {
            owner: String::from("MitchWijt"),
            repo_name: String::from("prBuddy"),
        };

        assert_eq!(url_data, expected);
    }
}