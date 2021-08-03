use std::ffi::OsString;
use std::iter::once;
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::{env, io};

#[cfg(target_os = "linux")]
fn main() -> io::Result<()> {
    let mpv = "/usr/bin/mpv";
    let args = env::args_os().skip(1);

    let mut gnome = false;
    if let Ok(status) = Command::new("gnome-session-inhibit").arg("true").status() {
        if status.success() {
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

    let err = command.exec();
    Err(err)
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
