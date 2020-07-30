use std::ffi::OsString;
use std::io::ErrorKind;
use std::iter::once;
use std::process::{exit, Command};
use std::{env, io};

fn main() -> io::Result<()> {
    let mpv = "/usr/bin/mpv";
    let args = env::args_os().skip(1);

    let mut gnome = false;
    if let Ok(desktop) = env::var("XDG_CURRENT_DESKTOP") {
        if desktop == "GNOME" {
            gnome = true;
        }
    }

    let mut command = if gnome {
        let c = Command::new("gnome-session-inhibit");
        add_args(c, once(OsString::from(mpv)).chain(args))
    } else {
        let c = Command::new(mpv);
        add_args(c, args)
    };

    let status = command.status()?;
    if let Some(code) = status.code() {
        exit(code);
    }
    Err(io::Error::new(
        ErrorKind::Other,
        "Process terminated by signal",
    ))
}

fn add_args<I>(mut command: Command, args: I) -> Command
where
    I: IntoIterator<Item = OsString>,
{
    for arg in args {
        command.arg(arg);
    }
    command
}
