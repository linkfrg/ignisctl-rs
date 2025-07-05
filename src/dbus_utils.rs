use zbus::{Error as ZBusError, Result as ZbusResult, blocking::Connection, proxy};

#[proxy(
    default_service = "com.github.linkfrg.ignis",
    interface = "com.github.linkfrg.ignis",
    default_path = "/com/github/linkfrg/ignis"
)]
pub trait IgnisIpc {
    fn list_windows(&self) -> ZbusResult<Vec<String>>;
    fn inspector(&self) -> ZbusResult<()>;
    fn reload(&self) -> ZbusResult<()>;
    fn quit(&self) -> ZbusResult<()>;

    fn open_window(&self, name: &String) -> ZbusResult<bool>;
    fn close_window(&self, name: &String) -> ZbusResult<bool>;
    fn toggle_window(&self, name: &String) -> ZbusResult<bool>;
    fn run_python(&self, code: &String) -> ZbusResult<()>;
    fn run_file(&self, path: &String) -> ZbusResult<()>;
}

pub trait ExitOnDBusError<T> {
    fn or_exit(self) -> T;
}

impl<T> ExitOnDBusError<T> for ZbusResult<T> {
    fn or_exit(self) -> T {
        self.unwrap_or_else(|e| {
            if let ZBusError::MethodError(err, _, _) = &e {
                if err.as_str() == "org.freedesktop.DBus.Error.ServiceUnknown" {
                    eprintln!("Ignis is not running.");
                    std::process::exit(1);
                }
            }

            eprintln!("D-Bus error occured: {e}");

            std::process::exit(1);
        })
    }
}

pub fn create_manager<'a>() -> IgnisIpcProxyBlocking<'a> {
    let conn = Connection::session().or_exit();
    IgnisIpcProxyBlocking::new(&conn).or_exit()
}
