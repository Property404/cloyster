#ifndef __CLOYSTER_INC_TIME_H
#define __CLOYSTER_INC_TIME_H
#include <stdint.h>

typedef int64_t time_t;
struct timespec;

time_t time(time_t* tloc);

#ifdef __unix__
int nanosleep(const struct timespec* req, struct timespec* rem);
#endif

#endif
