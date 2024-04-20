#ifndef __CLOYSTER_INC_STDIO_H
#define __CLOYSTER_INC_STDIO_H
#include <stddef.h>

// We can leave the actual definition to Rust code
typedef struct FILE FILE;

FILE* fopen(const char* restrict pathname, const char* restrict mode);
size_t fread(void* ptr, size_t size, size_t nmemb, FILE* restrict stream);
size_t fwrite(const void* ptr, size_t size, size_t nmemb, FILE* restrict stream);
int fclose(FILE* stream);

int printf(const char* restrict format, ...);
int puts(const char* s);

#endif
