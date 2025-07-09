use clap::{Parser, Subcommand};

mod calls;
mod dbus_utils;
mod misc_utils;

#[derive(Parser)]
#[command(version)]
#[command(about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

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

    /// List names of all windows.
    ListWindows {},

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
        Some(Commands::Quit {}) => calls::quit(),
        Some(Commands::Reload {}) => calls::reload(),
        Some(Commands::Inspector {}) => calls::inspector(),
        Some(Commands::OpenWindow { window_name }) => calls::open_window(window_name),
        Some(Commands::CloseWindow { window_name }) => calls::close_window(window_name),
        Some(Commands::ToggleWindow { window_name }) => calls::toggle_window(window_name),
        Some(Commands::RunPython { code }) => calls::run_python(code),
        Some(Commands::RunFile { path }) => calls::run_file(&misc_utils::resolve_path(path)),
        None => {}
    }
}
