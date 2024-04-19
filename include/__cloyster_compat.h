#ifndef __CLOYSTER_INC_PRIV_COMPAT_H
#define __CLOYSTER_INC_PRIV_COMPAT_H

#ifndef __cplusplus
    // C23 includes the '#warning' directive, constexpr, nullptr,
    // bool without stdbool, binary prefixes, and a lot of other
    // niceties
    #if __STDC_VERSION__ < 202311L
        // C11/C17 needs this because `bool` isn't a keyword yet
        #include <stdbool.h>

        #if __STDC_VERSION__ <= 201710L
            #warning Cloyster headers only support C23 and above
        #endif
    #endif
// C++23 includes the '#warning' directive
#elif __cplusplus <= 202002L
    #warning Cloyster headers only support C++23 and above
#endif

#endif
