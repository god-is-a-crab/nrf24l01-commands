pub mod commands;
pub mod registers;

pub use commands::Command;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_register() {
        let command_word = Command::WriteRegister(registers::Address::SetupRetr as u8).word();
        assert_eq!(command_word, 0b0010_0100);
    }
}
