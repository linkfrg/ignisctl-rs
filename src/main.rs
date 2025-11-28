use clap::{Parser, Subcommand};

mod calls;
mod dbus_utils;
mod misc_utils;

#[derive(Parser)]
#[command(version)]
#[command(about, long_about = None)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
enum Commands {
    /// Open a window.
    OpenWindow {
        /// The name of the window.
        window_name: String,
    },

    /// Close a window.
    CloseWindow {
        /// The name of the window.
        window_name: String,
    },

    /// Toggle a window.
    ToggleWindow {
        /// The name of the window.
        window_name: String,
    },

    /// Run a command by its name.
    RunCommand {
        /// The name of the command
        command_name: String,

        /// Arguments to pass to the command
        command_args: Vec<String>,
    },

    /// List names of all windows.
    ListWindows {},

    /// List names of all commands.
    ListCommands {},

    /// Execute a Python code inside the running Ignis process.
    RunPython {
        /// The Python code to run.
        code: String,
    },

    /// Execute a Python file inside the running Ignis process.
    RunFile {
        /// A path to the Python file to run.
        path: String,
    },

    /// Open GTK Inspector.
    Inspector {},

    /// Reload Ignis.
    Reload {},

    /// Quit Ignis.
    Quit {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::ListWindows {}) => calls::list_windows(),
        Some(Commands::ListCommands {}) => calls::list_commands(),
        Some(Commands::Quit {}) => calls::quit(),
        Some(Commands::Reload {}) => calls::reload(),
        Some(Commands::Inspector {}) => calls::inspector(),
        Some(Commands::OpenWindow { window_name }) => calls::open_window(window_name),
        Some(Commands::CloseWindow { window_name }) => calls::close_window(window_name),
        Some(Commands::ToggleWindow { window_name }) => calls::toggle_window(window_name),
        Some(Commands::RunCommand {
            command_name,
            command_args,
        }) => calls::run_command(command_name, command_args),
        Some(Commands::RunPython { code }) => calls::run_python(code),
        Some(Commands::RunFile { path }) => calls::run_file(&misc_utils::resolve_path(path)),
        _ => {}
    }
}
