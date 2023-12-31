// loosely based on https://github.com/xoac/opinionated-rust-template/blob/main/template/pre-get-repository.rhai
if !variable::is_set("project-description") {
    let project_description = variable::prompt("Short description of the project");
    variable::set("project-description", project_description);
}

let use_gitserver = if variable::is_set("use_gitserver") {
    variable::get("use_gitserver")
} else {
    variable::prompt("Would you like to set a Git repository URL?", "true", ["true", "false"])
};

if use_gitserver == "true" || use_gitserver == true {
    let gs_username = variable::prompt("Username");
    variable::set("gs_username", gs_username);
    let project_name = variable::get("project-name");
    let origin_repository = variable::prompt("Repository URL", "https://github.com/" + {gs_username} + "/" + {project_name});
    origin_repository.trim();
    let repository = origin_repository; // make copy for parse

    // remove `http://` `https://` or `git@`
    let schemas = ["http://", "https://", "git@"];
    for schema in schemas {
        let schema_index = repository.index_of(schema);
        if schema_index == 0 {
            let splitted_repo = repository.split(schema);
            repository = splitted_repo[1];
            break;
        }
    }

    // split by `/` assume there is `git server`, `organization`,  `project-name`
    let splitted_repo = repository.split(`/`);

    if splitted_repo.len() != 3 {
        abort("could not parse repository url. Expected format <gitserver>/username/project-name ")
    }

    let git_server = splitted_repo[0];
    let git_user = splitted_repo[1];
    let git_project_name = splitted_repo[2];

    if git_project_name != project_name {
        // FIXME: this should be only warning
        let warn_msg = "!!! => project-name(" + {project_name} + ") has different name than used in git-url(" + {git_project_name} + ")!";
        print(warn_msg);

        project_name = variable::prompt("The project name in link and used in prompt are different. Which `project-name` to use?", {project_name}, [{project_name}, {git_project_name}]);
    }

    variable::set("repository", "https://" + {repository});
    variable::set("project-name", project_name);
} else {
    print("!!! repository in Cargo.toml will not be set");
    variable::set("repository", "");
}

if !variable::is_set("crossterm_io") {
    let crossterm_io = variable::prompt("Use stdout or stderr for Crossterm IO?", "stdout", ["stdout", "stderr"]);
    variable::set("crossterm_io", crossterm_io);
} else {
    if variable::get("crossterm_io").to_lower() == "stdout" {
        variable::set("crossterm_io", "stdout");
    } else if variable::get("crossterm_io").to_lower() == "stderr" {
        variable::set("crossterm_io", "stderr");
    } else {
        print("!!! Unknown value for `crossterm_io`: " + variable::get("crossterm_io") + ". Using `stdout`.");
        variable::set("crossterm_io", "stdout");
    }
};

let use_rustfmt = if variable::is_set("use_rustfmt") {
    variable::get("use_rustfmt")
} else {
    variable::prompt("Use an opinionated rustfmt.toml file?", "false", ["true", "false"])
};

if use_rustfmt == "false" || use_rustfmt == false {
    file::delete("./.rustfmt.toml");
} else {
    print("!!! opinionated rustfmt.toml requires nightly.");
    variable::set("msrv", "nightly");
}

let msrv = if variable::is_set("msrv") {
    variable::get("msrv")
} else {
    variable::prompt("What is your Minimum Supported Rust Version", "stable")
};
variable::set("msrv", msrv);

let project_name = variable::get("project-name");
let crate_name = project_name;
crate_name.replace("-", "_");
crate_name.replace(" ", "_");
variable::set("crate_name", crate_name);

file::delete("./hooks");
