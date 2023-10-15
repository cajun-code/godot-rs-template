let project_name = variable::get("project-name")

file::rename("project.gdextension", `${project_name}.gdextension`)