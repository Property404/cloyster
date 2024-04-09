# Cloyster

Experiment to see how difficult it is to write a libc replacement

A big flaw here is that thread locals are not implemented, so errno is only
protected in Rust with `spin::Mutex` 😬

Enough functionality is implemented to run
[dbfi](https://github.com/Property404/dbfi) nearly unmodified (the time
function needs to be set to `time()`)

Header files not included (just use glibc's)

Only supports x86_64 on Linux