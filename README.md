# jitcalc-rs

This is my solution to the [JIT Calculator Challenge](https://ochagavia.nl/blog/the-jit-calculator-challenge/).

The challenge is to create a "JIT compiler" for the language that the following mini-interpreter gives rise to:
```rust
fn main() {
    // A simple integer calculator:
    // `+` or `-` means add or subtract by 1
    // `*` or `/` means multiply or divide by 2

    let program = "+ + * - /";
    let mut accumulator = 0;

    for token in program.chars() {
        match token {
            '+' => accumulator += 1,
            '-' => accumulator -= 1,
            '*' => accumulator *= 2,
            '/' => accumulator /= 2,
            _ => { /* ignore everything else */ }
        }
    }

    println!("The program \"{}\" calculates the value {}",
              program, accumulator);
}
```

This is an educational exercise to demonstrate the absolute basics of JIT compilation.

# Notes

## Multiple Solutions

I provide two solutions which are nearly identical. The only real difference is that solution 2 emits pre-assembled code, whereas solution 1 emits textual assembly which is then passed through an assembler.

## Dependencies

Solution 1 requires GNU `binutils`. Please make sure `binutils` is installed prior to running with `--soln1`.

## Portability

This solution is not portable and only runs on `x86_64`. To support a different architecture, the `compile` function can be changed with minimal effort.

## Division

The division instruction emitted is `sal $1, %eax`. This is naive since it does not mimic the exact behavior of integer division in Rust. This behaves like floor division, like `//` in Python. When dividing a negative odd integer by 2, shifting left will yield a value one less than integer dividing. In other words, `-(2k+1) >> 1 = -(2k+1)/2 - 1` where `k` is a positive integer. This can be "fixed" by using something like `idiv`.

## References

* [Metaprogramming and JIT Compilers - A Brief Overview](https://www.youtube.com/watch?v=FFgvV0sA3kU)

