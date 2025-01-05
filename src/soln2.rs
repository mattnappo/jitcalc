use super::*;

pub struct Soln2;

impl Jit for Soln2 {
    fn jit(program: &str) -> Result<Vec<u8>, JitError> {
        // Map each arithmetic symbol to an assembled instruction
        let calculation: Vec<u8> = program
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| match c {
                '+' => Ok(vec![0x48, 0xff, 0xc0]), // inc %rax
                '-' => Ok(vec![0x48, 0xff, 0xc8]), // dec %rax
                '*' => Ok(vec![0x48, 0xd1, 0xe0]), // sal $1, %rax
                '/' => Ok(vec![0x48, 0xd1, 0xf8]), // sar $1, %rax
                _ => return Err(JitError::InvalidSymbol(c)),
            })
            .collect::<Result<Vec<Vec<u8>>, JitError>>()?
            .into_iter()
            .flatten()
            .collect();

        // Build the final assembled code
        let mut code = vec![0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00]; // mov $0, %eax
        code.extend(calculation);
        code.push(0xC3); // ret

        Ok(code)
    }
}
