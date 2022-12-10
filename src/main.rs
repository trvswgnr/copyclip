use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::{error::Error, io};

fn run_program(
    input: &mut dyn io::Read,
    output: &mut dyn io::Write,
) -> Result<Option<ClipboardContext>, Box<dyn Error>> {
    let mut input_string = String::new();
    input.read_to_string(&mut input_string)?;

    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(input_string)?;

    let result = match ctx
        .get_contents()
        .unwrap_or_else(|_| "".to_string())
        .as_str()
    {
        "" => {
            output.write_all(b"Clipboard is empty.\n")?;
            Err("Clipboard is empty.".into())
        }
        _ => {
            output.write_all(b"Copied to clipboard!\n")?;
            Ok(Some(ctx))
        }
    };

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    match run_program(&mut io::stdin(), &mut io::stdout()) {
        Ok(ctx) => ctx,
        Err(err) => {
            eprintln!("{}", err);
            return Ok(());
        }
    };

    Ok(())
}

// #[cfg(any(windows, target_os = "macos"))]
#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use std::io::Read;
    use std::io::{self, Cursor};

    use super::*;
    use serial_test::serial;

    trait WaitTimeout {
        fn wait_timeout(
            &mut self,
            timeout_seconds: u64,
        ) -> io::Result<Option<std::process::ExitStatus>>;
    }

    impl WaitTimeout for std::process::Child {
        fn wait_timeout(
            &mut self,
            timeout_seconds: u64,
        ) -> io::Result<Option<std::process::ExitStatus>> {
            let timeout = std::time::Duration::from_secs(timeout_seconds);
            // Wait for the process to exit, or kill it if it takes too long.
            let start = std::time::Instant::now();
            let mut ended_properly = false;
            while start.elapsed() < timeout {
                ended_properly = match self.try_wait()? {
                    Some(status) => return Ok(Some(status)),
                    None => {
                        std::thread::sleep(std::time::Duration::from_millis(1));
                        false
                    }
                }
            }

            if ended_properly {
                return Ok(None);
            }

            // kill the process
            self.kill()?;

            // wait for the process to exit
            self.wait()?;

            Ok(None)
        }
    }

    #[test]
    #[serial]
    fn test_stdin_read() -> Result<(), Box<dyn Error>> {
        let input = "test stdin read";
        let mut input_buf = Cursor::new(input);
        let mut output = Vec::new();

        let result = run_program(&mut input_buf, &mut output);

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    #[serial]
    fn test_clipboard_context_creation() -> Result<(), Box<dyn Error>> {
        let input = "test input";
        let mut input_buf = Cursor::new(input);
        let mut output = Vec::new();

        let result = run_program(&mut input_buf, &mut output);

        assert!(result.is_ok());

        let ctx = result?;
        assert!(ctx.is_some());

        Ok(())
    }

    #[test]
    #[serial]
    fn test_clipboard_set_contents() -> Result<(), Box<dyn Error>> {
        let input = "test clipboard set contents";
        let mut input_buf = Cursor::new(input);
        let mut output = Vec::new();

        let result = run_program(&mut input_buf, &mut output);

        assert!(result.is_ok());

        let ctx = result?;
        assert!(ctx.is_some());

        let clipboard_contents = ctx.unwrap().get_contents()?;
        assert_eq!(clipboard_contents, input);

        assert_eq!(output, b"Copied to clipboard!\n");

        Ok(())
    }

    #[test]
    #[serial]
    fn test_clipboard_empty() -> Result<(), Box<dyn Error>> {
        let input = "";
        let mut input_buf = Cursor::new(input);
        let mut output = Cursor::new(Vec::new());
        let result = run_program(&mut input_buf, &mut output);

        assert!(result.is_err());

        // get the error message from result
        unsafe {
            let error = result.unwrap_err_unchecked();
            let error_message = error.to_string();
            assert_eq!(error_message, "Clipboard is empty.");
        }

        Ok(())
    }

    #[test]
    #[serial]
    fn test_fails_on_invalid_input() -> Result<(), Box<dyn Error>> {
        // make sure that the clipboard is empty
        let mut ctx: ClipboardContext = ClipboardProvider::new()?;
        ctx.set_contents("".to_string())?;
        // make sure the program fails on invalid input
        let input = "";
        let mut input_buf = Cursor::new(input);
        let mut output = Vec::new();
        let result = run_program(&mut input_buf, &mut output);
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    #[serial]
    #[cfg(target_os = "macos")] // TODO: make this work on windows and linux.
    fn test_pipe() -> Result<(), Box<dyn Error>> {
        // new process that runs `echo "test"`.
        let mut child_echo = std::process::Command::new("echo")
            .arg("test 2")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        // wait for the child process to finish, or kill it if it takes too long.
        let status = child_echo.wait_timeout(10)?;
        assert!(status.is_some(), "Child `echo` process timed out!");

        // pipe the output of the child process to a new process that runs the `cargo run` command.
        let child_echo_stdout = child_echo.stdout.take().unwrap();

        let mut child_cargo_run = std::process::Command::new("cargo")
            .arg("run")
            .stdin(child_echo_stdout)
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        // wait for the child process to finish, or kill it if it takes too long.
        let status = child_cargo_run.wait_timeout(10)?;
        assert!(status.is_some(), "Child `cargo run` process timed out!");

        // check contents of the clipboard are the same as the string.
        let mut ctx: ClipboardContext = ClipboardProvider::new()?;
        assert_eq!(ctx.get_contents()?, "test 2\n"); // * the newline is added by the `echo` command.

        // check the output of the `cargo run` command
        let mut output = String::new();
        let child_cargo_run_stdout = child_cargo_run.stdout.take().unwrap();
        io::BufReader::new(child_cargo_run_stdout).read_to_string(&mut output)?;
        assert_eq!(output, "Copied to clipboard!\n");

        Ok(())
    }
}
