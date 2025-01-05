use std::mem;
use std::ptr;
use std::{fmt, io};

use libc::{mmap, munmap, MAP_ANON, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE};

const TMP_OBJFILE: &str = "/tmp/jitcalc.o";
const TMP_BINFILE: &str = "/tmp/jitcalc.bin";

/// JIT-compilers have the following interface described by this trait.
pub trait Jit {
    /// JIT-compile a user-supplied program.
    fn jit(program: &str) -> Result<Vec<u8>, JitError>;

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

        // Memcpy the assembled code into the mmap'd region
        unsafe {
            ptr::copy_nonoverlapping(code.as_ptr(), addr as *mut u8, code.len());
        }

        // Interpret/cast the region to a function pointer
        let calc: unsafe fn() -> i64 = unsafe { mem::transmute(addr) };

        // Execute the region, which can be done by calling the function pointer
        let result = unsafe { calc() };

        // Unmap & free
        let unmap_status = unsafe { munmap(addr, mmap_size) };
        if unmap_status != 0 {
            return Err(JitError::MmapFailure(unmap_status as usize));
        }

        Ok(result)
    }
}

#[derive(Debug)]
pub enum JitError {
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
            JitError::MmapFailure(size) => {
                write!(f, "failed to allocate or free mmap region with size {size}")
            }
        }
    }
}

impl From<io::Error> for JitError {
    fn from(err: io::Error) -> Self {
        JitError::IoError(err)
    }
}

impl std::error::Error for JitError {}

pub mod soln1;
pub use soln1::Soln1;

pub mod soln2;
pub use soln2::Soln2;
