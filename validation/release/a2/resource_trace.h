#pragma once
#include <stdio.h>
#ifdef __cplusplus
extern "C" {
#endif
FILE *reference_fopen_at(const char *path, const char *mode, const char *caller);
void reference_stage(const char *stage);
#ifdef __cplusplus
}
#endif

#define reference_fopen(path, mode) reference_fopen_at(path, mode, __FILE__)
