use libc::{mmap, munmap, MAP_ANON, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE};
use std::fmt;
use std::io::Write;
use std::mem;
use std::process::{Command, Stdio};
use std::ptr;
use std::io;

const TMP_OBJFILE: &str = "/tmp/jitcalc.o";
const TMP_BINFILE: &str = "/tmp/jitcalc.bin";

#[derive(Debug)]
enum JitError {
    InvalidSymbol(char),
    IoError(io::Error),
    AsmFailure(i32),
    BinFailure(i32),
    MmapFailure(usize),
}

impl fmt::Display for JitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JitError::InvalidSymbol(c) => write!(f, "invalid symbol: {c}"),
            JitError::IoError(e) => write!(f, "{e}"),
            JitError::AsmFailure(code) => write!(f, "failed to assemble program: {code}"),
            JitError::BinFailure(code) => {
                write!(f, "failed to extract binary from ELF: {code}")
            }
            JitError::MmapFailure(size) => write!(f, "failed to allocate or free mmap region with size {size}"),
        }
    }
}

impl From<io::Error> for JitError {
    fn from(err: io::Error) -> Self {
        JitError::IoError(err)
    }
}

impl std::error::Error for JitError {}

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
    Ok(format!("movq $0, %rax\nmovq $2, %rcx\n{}\nret\n", calculation))
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

    // Use objcopy to extract the raw binary of the assembly
    // Write the output to TMP_BINFILE
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

/// JIT-compile a user-supplied program.
fn jit(program: &str) -> Result<Vec<u8>, JitError> {
    let asm = compile(program)?;
    assemble(&asm)
}

/// Execute a JIT-compiled code block.
fn run(code: &[u8]) -> Result<i64, JitError> {
    let mmap_size = code.len();

    // Allocate an executable mmap region
    let addr = unsafe {
        mmap(
            ptr::null_mut(),
            mmap_size,
            PROT_READ | PROT_WRITE | PROT_EXEC, // RWX permissions
            MAP_ANON | MAP_PRIVATE,             // Anon and private pages
            -1,                                 
            0,
        )
    };

    if addr == libc::MAP_FAILED {
        return Err(JitError::MmapFailure(mmap_size));
    }

    // Copy the assembled code into the mmap region
    unsafe {
        ptr::copy_nonoverlapping(code.as_ptr(), addr as *mut u8, code.len());
    }

    // Interpret/cast the region to a function pointer
    let calc: unsafe fn() -> i64 = unsafe { mem::transmute(addr) };

    // Execute the region, which can be done by calling the function pointer
    let result = unsafe { calc() };

    // Unmap the memory after we're done with it
    let unmap_status = unsafe { munmap(addr, mmap_size) };
    if unmap_status != 0 {
        return Err(JitError::MmapFailure(unmap_status as usize));
    }

    Ok(result)
}

/// Basically just runs `run(&jit(input))` in a loop
fn repl() -> Result<(), io::Error> {
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush()?;
        
        // Read user input
        input.clear();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input == "exit" {
            break;
        }

        // JIT-compile and run the user-supplied program
        match jit(input) {
            Ok(code) => match run(&code) {
                Ok(result) => println!("{result}"),
                Err(e) => eprintln!("error: {e}"),
            },
            Err(e) => eprintln!("error: {e}"),
        }
    }

    Ok(())
    
}


fn main() -> Result<(), io::Error> {
    repl()?;
    
    Ok(())
}
