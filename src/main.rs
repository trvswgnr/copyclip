use clipboard::{ClipboardContext, ClipboardProvider};
use std::{
    error::Error,
    io::{self, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(input)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        let mut ctx: ClipboardContext = ClipboardProvider::new()?;
        ctx.set_contents("test".to_string())?;
        assert_eq!(ctx.get_contents()?, "test");

        Ok(())
    }
}
