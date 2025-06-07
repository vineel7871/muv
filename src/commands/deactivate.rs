// src/commands/deactivate.rs
use anyhow::Result;

pub fn handle_deactivate_for_shell_export() -> Result<()> {
    // These commands are designed to be run by `eval` from the shell function
    // and assume `guv activate` (or the `deactivate` function it defines) has set things up.

    // 1. Restore PS1
    println!("if [ -n \"${{GUV_OLD_PS1+x}}\" ]; then");
    println!("    export PS1=\"$GUV_OLD_PS1\"");
    println!("    unset GUV_OLD_PS1");
    println!("else");
    println!("    unset PS1"); // Or set to a default if GUV_OLD_PS1 was never set
    println!("fi");

    // 2. Restore PATH
    println!("if [ -n \"${{GUV_OLD_PATH+x}}\" ]; then");
    println!("    export PATH=\"$GUV_OLD_PATH\"");
    println!("    unset GUV_OLD_PATH");
    println!("fi");

    // 3. Restore PYTHONHOME if it was saved by our activate
    println!("if [ -n \"${{_GUV_OLD_VIRTUAL_PYTHONHOME+x}}\" ] ; then");
    println!("    export PYTHONHOME=\"$_GUV_OLD_VIRTUAL_PYTHONHOME\"");
    println!("    unset _GUV_OLD_VIRTUAL_PYTHONHOME");
    println!("fi");

    // 4. Unset environment variables
    println!("unset VIRTUAL_ENV");
    println!("unset GUV_ENV_NAME");

    // 5. Undefine the 'deactivate' function if it exists and was ours
    // Check for GUV_OLD_PS1 as a proxy for whether our activate was run
    println!("if [ -n \"${{GUV_OLD_PS1+x}}\" ] && declare -f -F deactivate > /dev/null; then unset -f deactivate; fi");
    println!("if declare -f -F _guv_saved_deactivate > /dev/null; then unset -f _guv_saved_deactivate; fi");


    eprintln!("GUV environment deactivated.");
    // Crucial: Ensure the last command for eval is simple or returns 0
    println!(": # GUV deactivation successful marker");
    Ok(())
}