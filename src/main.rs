use clipboard::{ClipboardContext, ClipboardProvider};
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

#[cfg(test)]
mod tests {
    use std::io::{self, Cursor, Read};

    use super::*;
    use serial_test::serial;

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
    fn test_clipboard_set_contents() {
        let input = "test clipboard set contents";
        let mut input_buf = Cursor::new(input);
        let mut output = Vec::new();

        let result = run_program(&mut input_buf, &mut output);

        assert!(result.is_ok());

        let ctx = result.unwrap();
        assert!(ctx.is_some());

        let clipboard_contents = ctx.unwrap().get_contents().unwrap();
        assert_eq!(clipboard_contents, input);
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
    fn test_pipe() -> Result<(), Box<dyn Error>> {
        // Create a new process that runs `echo "test"`.
        let mut child = std::process::Command::new("echo")
            .arg("test 2")
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        // Get the stdout of the child process.
        let stdout = child.stdout.take().unwrap();

        // Read the stdout of the child process into a string.
        let mut input = String::new();
        io::BufReader::new(stdout).read_to_string(&mut input)?;

        // Set the contents of the clipboard to the string.
        let mut ctx: ClipboardContext = ClipboardProvider::new()?;
        ctx.set_contents(input)?;

        // Assert that the contents of the clipboard is the same as the string.
        assert_eq!(ctx.get_contents()?, "test 2\n"); // @note: the newline is added by the `echo` command.

        Ok(())
    }
}
