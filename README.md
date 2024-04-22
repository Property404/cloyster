# Cloyster

Experiment to see how difficult it is to write a libc replacement

A big flaw here is that thread locals are barely implemented, so errno is only
protected in Rust with `spin::Mutex` 😬

Enough functionality is implemented to run
[dbfi](https://github.com/Property404/dbfi) nearly unmodified (the time
function needs to be set to `time()`)

Only supports Linux(x86_64 and RISC-V)

Note that this requires a nightly compiler

## Crates

* Cloyster - Cloyster C library
* Shellder - Rust implementation of C functions without exports

## Statically linking Cloyster

```
cargo build
...
gcc my_program.c -ffreestanding -nostdlib target/debug/libcloyster.a
./a.out
```

## License

LGPLv2.1 OR LGPLv3 at your option
