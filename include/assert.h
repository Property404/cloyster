#ifndef __CLOYSTER_INC_ASSERT_H
#define __CLOYSTER_INC_ASSERT_H

void __assert_fail(const char* assertion, const char* file, unsigned int line,
                   const char* function);

#define assert(expression)                                                                         \
    {                                                                                              \
        if (!(expression)) {                                                                       \
            __assert_fail(#expression, __FILE__, __LINE__, __func__);                              \
        }                                                                                          \
    }

#endif
