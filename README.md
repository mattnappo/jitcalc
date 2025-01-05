# jitcalc-rs

This is my solution to the [JIT Calculator Challenge](https://ochagavia.nl/blog/the-jit-calculator-challenge/).

# Notes

## Dependencies

This solution requires GNU `binutils`. Please make sure `binutils` is installed prior to running.

## Portability

This solution is not portable and only runs on `x86_64`. To support a different architecture, the `compile` function can be changed with minimal effort.

## Division

The division instruction emitted is `sal $1, %eax`. This is naive since it does not mimic the exact behavior of integer division in Rust. When dividing a negative odd integer by 2, shifting left will yield a value one less than integer dividing. In other words, `-(2k+1) >> 1 = -(2k+1)/2 - 1`.

This can be fixed by using a proper division instruction, or by adding an `inc %eax` when necessary (would need to do this statically, which defeats the point).
