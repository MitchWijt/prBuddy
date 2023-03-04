use shutil::pipe;

pub struct GitData {
    pub branch: String,
    pub main_branch: String,
    pub repo_name: String,
    pub owner: String,
}

struct GitCommandArgs<'a> {
    branch: Vec<Vec<&'a str>>,
    main_branch: Vec<Vec<&'a str>>,
    remote_url: Vec<Vec<&'a str>>,
}

struct GitUrlData {
    repo_name: String,
    owner: String,
}

pub fn get_git_data() -> Result<GitData, &'static str> {
    let git_command_args = get_git_command_args();

    let branch_name = get_command_output(git_command_args.branch)?;
    let remote_url = get_command_output(git_command_args.remote_url)?;
    let main_branch_name = get_command_output(git_command_args.main_branch)?;

    let url_data = get_data_from_url(remote_url);

    return Ok(GitData {
        branch: branch_name,
        main_branch: main_branch_name,
        repo_name: url_data.repo_name,
        owner: url_data.owner
    })
}

fn get_git_command_args<'a>() -> GitCommandArgs<'a> {
    return GitCommandArgs {
        branch: vec![vec!["git", "rev-parse", "--abbrev-ref", "HEAD"]],
        main_branch: vec![vec!["git", "remote", "show", "origin"], vec!["awk", r#"/HEAD branch/ {print $NF}"#]],
        remote_url: vec![vec!["git", "config", "--get", "remote.origin.url"]],
    }
}

fn get_command_output(args: Vec<Vec<&str>>) -> Result<String, &'static str> {
    let output = pipe(args);
    match output {
        Ok(output_str) => Ok(output_str.replace("\n", "")),
        Err(_e) => Err("Failed to execute command")
    }
}

fn get_data_from_url(url: String) -> GitUrlData {
    return if url.contains("git@") {
        get_repo_info_ssh(&url)
    } else {
        get_repo_info_https(&url)
    }
}

fn get_repo_info_ssh(url: &String) -> GitUrlData {
    let split = url.split("/");
    let vector: Vec<&str> = split.collect();

    let owner = *vector.get(0).unwrap();
    let repo_name = *vector.get(1).unwrap();

    return GitUrlData {
        owner: String::from(owner.replace("git@github.com:", "")),
        repo_name: String::from(repo_name.replace(".git", ""))
    }
}

fn get_repo_info_https(url: &String) -> GitUrlData {
    let split = url.split("/");
    let vector: Vec<&str> = split.collect();

    let owner = *vector.get(3).unwrap();
    let repo_name = *vector.get(4).unwrap();

    return GitUrlData {
        owner: String::from(owner),
        repo_name: String::from(repo_name.replace(".git", ""))
    }
}