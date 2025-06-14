use crate::cli::EnvNameArg;
use crate::utils;
use anyhow::Result;
// std::env

pub fn handle_activate_for_shell_export(args: EnvNameArg) -> Result<()> {
    let (env_path, env_name) = utils::get_active_or_specified_env(args.name.as_ref())?;
    let env_bin_path = env_path.join("bin");

    // Important: These commands are for POSIX-like shells (bash, zsh).
    // Fish shell or Windows CMD/PowerShell would require different commands.

    // 1. Save current state if not already saved by a previous muv activation
    // We use MUV_ prefixed variables to avoid clashes.
    println!("if [ -z \"$MUV_OLD_PATH\" ]; then export MUV_OLD_PATH=\"$PATH\"; fi");
    println!("if [ -z \"$MUV_OLD_PS1\" ]; then export MUV_OLD_PS1=\"$PS1\"; fi");
    // Save original PYTHONHOME if it exists
    println!(
        "if [ -n \"${{PYTHONHOME+x}}\" ] && [ -z \"$_MUV_OLD_VIRTUAL_PYTHONHOME\" ]; then export _MUV_OLD_VIRTUAL_PYTHONHOME=\"$PYTHONHOME\"; fi"
    );

    // 2. Set new PATH
    // This simple prepend is usually what venv activate scripts do.
    println!("export PATH=\"{}:$PATH\"", env_bin_path.display());

    // 3. Set VIRTUAL_ENV
    println!("export VIRTUAL_ENV=\"{}\"", env_path.display());
    println!("export MUV_ENV_NAME=\"{}\"", env_name); // For prompt and tracking

    // 4. Update PS1 (prompt)
    // Handle case where PS1 might be unset or empty
    println!(
        "if [ -n \"${{PS1+x}}\" ]; then PS1=\"({}) $PS1\"; else PS1=\"({}) \"; fi",
        env_name, env_name
    );

    // 5. Clear PYTHONHOME (common practice for venvs to avoid conflicts)
    println!("if [ -n \"${{PYTHONHOME+x}}\" ]; then unset PYTHONHOME; fi");

    // 6. Define a deactivate function that can be called by typing 'deactivate'
    // This makes it behave more like standard virtual environments.
    println!(
        r#"
if declare -f -F deactivate > /dev/null; then
    eval "$(echo "function _muv_saved_deactivate() {{"; declare -f deactivate | tail -n +2; echo "}}")"
fi

deactivate() {{
    # Restore PS1
    if [ -n "${{MUV_OLD_PS1+x}}" ]; then
        export PS1="$MUV_OLD_PS1"
        unset MUV_OLD_PS1
    else
        unset PS1 # Or set to a default
    fi

    # Restore PATH
    if [ -n "${{MUV_OLD_PATH+x}}" ]; then
        export PATH="$MUV_OLD_PATH"
        unset MUV_OLD_PATH
    fi

    # Restore PYTHONHOME if it was saved
    if [ -n "${{_MUV_OLD_VIRTUAL_PYTHONHOME+x}}" ] ; then
        export PYTHONHOME="$_MUV_OLD_VIRTUAL_PYTHONHOME"
        unset _MUV_OLD_VIRTUAL_PYTHONHOME
    fi

    unset VIRTUAL_ENV
    unset MUV_ENV_NAME

    # Remove this deactivate function
    unset -f deactivate

    # If there was a previously saved deactivate, restore and call it
    if declare -f -F _muv_saved_deactivate > /dev/null; then
        eval "$(echo "function deactivate() {{"; declare -f _muv_saved_deactivate | tail -n +2; echo "}}")"
        unset -f _muv_saved_deactivate
        # Optionally call it: deactivate
    fi

    echo "Deactivated MUV environment (via 'deactivate' function)." >&2
}}
"#
    );

    // Indicate successful activation (to stderr, so it doesn't get `eval`ed)
    // eprintln!("GUV environment '{}' activated. To deactivate, type 'deactivate' or run 'guv deactivate'.", args.name);
    // Crucial: Ensure the last command for eval is simple or returns 0
    println!(": # MUV activation successful marker");

    Ok(())
}
