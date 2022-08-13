#[cfg(test)]
mod tests {


use std::io::{Error, Read, Write};

    pub fn upcase(
        input: &mut impl Read,
        output: &mut impl Write,
    ) -> Result<(), Error> {
        let mut buffer = "".to_string();

        input.read_to_string(&mut buffer)?;
        output.write_all(buffer.to_uppercase().as_bytes())?;

        Ok(())
    }




}
