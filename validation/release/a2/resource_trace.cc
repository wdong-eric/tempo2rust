#include "resource_trace.h"
#include <cstdlib>
#include <cerrno>
static const char *stage = "startup";
extern "C" void reference_stage(const char *value) { stage = value; }
extern "C" FILE *reference_fopen_at(const char *path, const char *mode, const char *caller) {
    FILE *result = fopen(path, mode);
    const int saved_errno = errno;
    const char *log = getenv("REFERENCE_RESOURCE_LOG");
    if (log) {
        FILE *out = fopen(log, "a");
        if (!out) { perror("resource trace"); exit(2); }
        fprintf(out, "%s\t%s\t%s\t%d\t%d\t%s\n", stage, path, mode, result != NULL, result ? 0 : saved_errno, caller);
        fclose(out);
    }
    errno = saved_errno;
    return result;
}
