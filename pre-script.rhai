let project_name = variable::get("project-name");

file::rename("extension.toml", `${project_name}.gdextension`);