/* Reuse AO's allocator observer for a direct Rust process. Whole-process
 * counts at 1001 minus 1 iterations isolate repeated work from startup/I/O.
 * Counter runs are separate from all uninstrumented performance samples. */
#define AO_COUNTER_LIBRARY
#include "allocations.c"
__attribute__((constructor)) static void start_counts(void) { ao_start(); }
__attribute__((destructor)) static void stop_counts(void) { ao_stop(); }
