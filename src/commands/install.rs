use crate::cli::PackageManagementArgs;
use crate::utils;
use anyhow::Result;

pub fn handle_install(args: PackageManagementArgs) -> Result<()> {
    let (env_path, env_name) = utils::get_active_or_specified_env(args.env_name.as_ref())?;

    if let Some(req_file) = &args.requirements {
        println!(
            "Installing dependencies from '{}' into environment '{}'...",
            req_file, env_name
        );
        let uv_cmd_args = vec!["pip", "install", "-r", req_file.as_str()];
        utils::run_uv_command(
            &uv_cmd_args,
            None,
            vec![(utils::ACTIVE_ENV_VAR, env_path.as_path())],
        )?;
        println!("Dependencies from '{}' installed successfully in '{}'.", req_file, env_name);
    }

    if let Some(req_file) = &args.toml {
        println!(
            "Installing dependencies from '{}' into environment '{}'...",
            req_file, env_name
        );
        let toml_content = std::fs::read_to_string(req_file)?;
        let parsed_toml: toml::Value = toml::from_str(&toml_content)?;
        if let Some(deps) = parsed_toml
            .get("project")
            .and_then(|proj| proj.get("dependencies"))
            .and_then(|d| d.as_array())
        {
            let dep_list: Vec<String> = deps
                .iter()
                .filter_map(|d| d.as_str().map(|s| s.to_string()))
                .collect();
            if !dep_list.is_empty() {
                let mut uv_cmd_args = vec!["pip", "install"];
                for dep in &dep_list {
                    uv_cmd_args.push(dep);
                }
                utils::run_uv_command(
                    &uv_cmd_args,
                    Some(&env_path),
                    vec![(utils::ACTIVE_ENV_VAR, env_path.as_path())],
                )?;
            }
        }
        println!("Dependencies from pyproject.toml installed successfully in '{}'.", env_name);
    }

    if !args.packages.is_empty() {
        println!(
            "Installing package(s) [{}] into environment '{}'...",
            args.packages.join(", "),
            env_name
        );

        let mut uv_cmd_args = vec!["pip", "install"];
        for pkg in &args.packages {
            uv_cmd_args.push(pkg.as_str());
        }

        utils::run_uv_command(
            &uv_cmd_args,
            None,
            vec![(utils::ACTIVE_ENV_VAR, env_path.as_path())],
        )?;

        println!("Package(s) installed successfully in '{}'.", env_name);
    }

    if args.requirements.is_none() && args.packages.is_empty() && args.toml.is_none() {
        println!("Nothing to install. Please specify packages or --requirements or --toml.");
    }

    Ok(())
}