use clipboard::{ClipboardContext, ClipboardProvider};
use std::{
    error::Error,
    io::{self, Read},
    sync::Mutex,
};

use lazy_static::lazy_static;

lazy_static! {
    static ref CTX: Mutex<ClipboardContext> = Mutex::new(ClipboardProvider::new().unwrap());
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    CTX.lock().unwrap().set_contents(input)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        // start a thread that runs the command `echo "hello world" | cargo run`
        let mut child = std::process::Command::new("echo")
            .arg("hello world")
            .arg("|")
            .arg("cargo")
            .arg("run")
            .spawn()?;
        // kill the child process after 200ms
        std::thread::sleep(std::time::Duration::from_millis(200));
        // wait for the child process to exit
        child.wait()?;
        // get the clipboard contents
        let mut _ctx = CTX.lock().unwrap();
        let contents = _ctx.get_contents()?;
        assert_eq!(contents, "hello world");
        Ok(())
    }
}
