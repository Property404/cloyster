#ifndef __CLOYSTER_INC_ERRNO_H
#define __CLOYSTER_INC_ERRNO_H

int* __errno_location();
#define errno (*__errno_location())

#endif
