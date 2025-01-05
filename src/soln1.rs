use std::io::Write;
use std::process::{Command, Stdio};

use super::*;

/// Convert a program in the source language into a sequence of assembly
/// instructions.
fn compile(program: &str) -> Result<String, JitError> {
    // Map each arithmetic symbol to an assembly instruction
    let calculation = program
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '+' => Ok("inc %rax"),
            '-' => Ok("dec %rax"),
            '*' => Ok("sal $1, %rax"),
            '/' => Ok("sar $1, %rax"),
            _ => return Err(JitError::InvalidSymbol(c)),
        })
        .collect::<Result<Vec<&str>, JitError>>()?
        .join("\n");

    // Build the final assembly code string
    Ok(format!("movq $0, %rax\n{}\nret\n", calculation))
}

/// Run `as` and `objcopy` to assemble an assembly string.
fn assemble(asm: &str) -> Result<Vec<u8>, JitError> {
    // Use 'as' to assemble the assembly string
    let status = Command::new("as")
        .arg("-o")
        .arg(TMP_OBJFILE)
        .stdin(Stdio::piped())
        .spawn()
        .and_then(|mut cmd| {
            {
                let mut stdin = cmd.stdin.take().expect("failed to get stdin for 'as'");
                stdin.write_all(asm.as_bytes())?;
            }
            cmd.wait()
        })?;

    if !status.success() {
        return Err(JitError::AsmFailure(status.code().unwrap()));
    }

    // Use objcopy to extract the raw binary of the assembly and write the
    // output to TMP_BINFILE
    let status = Command::new("objcopy")
        .arg("-O")
        .arg("binary")
        .arg(TMP_OBJFILE)
        .arg(TMP_BINFILE)
        .status()?;

    if !status.success() {
        return Err(JitError::BinFailure(status.code().unwrap()));
    }

    // Read out the output of TMP_BINFILE which now contains the final assembly
    Ok(std::fs::read(TMP_BINFILE)?)
}

pub struct Soln1;

impl Jit for Soln1 {
    fn jit(program: &str) -> Result<Vec<u8>, JitError> {
        let asm = compile(program)?;
        assemble(&asm)
    }
}
