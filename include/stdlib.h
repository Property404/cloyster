#ifndef __CLOYSTER_INC_STDLIB_H
#define __CLOYSTER_INC_STDLIB_H

// Normal process termination
[[noreturn]] void exit(int status);

// Abnormal process termination
[[noreturn]] void abort(void);

#endif
