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

pub fn list_windows() {
    for win in create_manager().list_windows().or_exit() {
        println!("{win}");
    }
}

call_window_op!(open_window);
call_window_op!(close_window);
call_window_op!(toggle_window);

call_no_args!(quit);
call_no_args!(reload);
call_no_args!(inspector);

call_with_string_arg!(run_python);
call_with_string_arg!(run_file);
