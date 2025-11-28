use crate::dbus_utils::{ExitOnDBusError, create_manager};

macro_rules! call_no_args {
    ($method:ident) => {
        pub fn $method() {
            create_manager().$method().or_exit();
        }
    };
}

macro_rules! call_with_string_arg {
    ($method:ident) => {
        pub fn $method(arg: &String) {
            create_manager().$method(arg).or_exit();
        }
    };
}

macro_rules! call_window_op {
    ($method:ident) => {
        pub fn $method(arg: &String) {
            let exists = create_manager().$method(arg).or_exit();
            if !exists {
                eprintln!("No such window: '{arg}'");
            }
        }
    };
}

macro_rules! call_print_list {
    ($method:ident) => {
        pub fn $method() {
            for obj in create_manager().$method().or_exit() {
                println!("{obj}");
            }
        }
    };
}

pub fn run_command(command_name: &String, command_args: &Vec<String>) {
    let (command_output, command_error) = create_manager()
        .run_command(command_name, command_args)
        .or_exit();

    println!("{command_output}");

    if command_error != "" {
        eprintln!("No such command: '{}'", command_error);
    }
}

call_print_list!(list_windows);
call_print_list!(list_commands);

call_window_op!(open_window);
call_window_op!(close_window);
call_window_op!(toggle_window);

call_no_args!(quit);
call_no_args!(reload);
call_no_args!(inspector);

call_with_string_arg!(run_python);
call_with_string_arg!(run_file);
