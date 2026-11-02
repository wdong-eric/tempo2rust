/* Single-thread allocation observation, never used for performance timings.
 * Darwin interposes external references; Linux --wrap observes static-link
 * references. Neither counts private allocator calls hidden inside libc. */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#ifndef AO_COUNTER_CLIENT
static uint64_t calls[4], bytes[3];
static int active;
#ifdef __APPLE__
#define REAL(name) name
#define WRAP(name) ao_##name
#define INTERPOSE(name) \
    __attribute__((used, section("__DATA,__interpose"))) \
    static const struct { const void *replacement, *original; } pair_##name = \
        { (const void *)WRAP(name), (const void *)name };
#else
#define REAL(name) __real_##name
#define WRAP(name) __wrap_##name
#define INTERPOSE(name)
extern void *__real_malloc(size_t);
extern void *__real_calloc(size_t, size_t);
extern void *__real_realloc(void *, size_t);
extern void __real_free(void *);
#endif
void *WRAP(malloc)(size_t n) {
    if (active) { ++calls[0]; bytes[0] += n; }
    return REAL(malloc)(n);
}
void *WRAP(calloc)(size_t n, size_t size) {
    if (active) { ++calls[1]; bytes[1] += n*size; }
    return REAL(calloc)(n, size);
}
void *WRAP(realloc)(void *p, size_t n) {
    if (active) { ++calls[2]; bytes[2] += n; }
    return REAL(realloc)(p, n);
}
void WRAP(free)(void *p) {
    if (active) ++calls[3];
    REAL(free)(p);
}
INTERPOSE(malloc)
INTERPOSE(calloc)
INTERPOSE(realloc)
INTERPOSE(free)
void ao_start(void) {
    active = 1;
}
void ao_stop(void) {
    active = 0;
    fprintf(stderr, "AO_ALLOC malloc=%llu calloc=%llu realloc=%llu free=%llu bytes=%llu\n",
        (unsigned long long)calls[0], (unsigned long long)calls[1],
        (unsigned long long)calls[2], (unsigned long long)calls[3],
        (unsigned long long)(bytes[0]+bytes[1]+bytes[2]));
}
#endif

#ifndef AO_COUNTER_LIBRARY
extern void ao_start(void);
extern void ao_stop(void);
#define main evaluation_main
#include "../parallel_execution/evaluation.c"
#undef main
int main(int argc, char **argv) {
    ao_start();
    int status = evaluation_main(argc, argv);
    ao_stop();
    return status;
}
#endif
