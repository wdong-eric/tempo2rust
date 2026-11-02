/* AN observer only. No payload reads: mincore queries the page tables. */
#define _DEFAULT_SOURCE
#include <assert.h>
#include <errno.h>
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/mman.h>
#include <sys/resource.h>
#include <time.h>
#include <unistd.h>
#ifdef __APPLE__
#include <mach/mach.h>
#include <mach/mach_vm.h>
#include <malloc/malloc.h>
#else
#include <malloc.h>
#endif

static double seconds(void) {
    struct timespec t;
    assert(clock_gettime(CLOCK_MONOTONIC, &t) == 0);
    return t.tv_sec + t.tv_nsec * 1e-9;
}

static uint64_t rss(void) {
#ifdef __APPLE__
    struct mach_task_basic_info info;
    mach_msg_type_number_t count = MACH_TASK_BASIC_INFO_COUNT;
    assert(task_info(mach_task_self(), MACH_TASK_BASIC_INFO,
                     (task_info_t)&info, &count) == KERN_SUCCESS);
    return info.resident_size;
#else
    uint64_t size, resident;
    FILE *f = fopen("/proc/self/statm", "r");
    assert(f && fscanf(f, "%" SCNu64 " %" SCNu64, &size, &resident) == 2);
    fclose(f);
    return resident * (uint64_t)sysconf(_SC_PAGESIZE);
#endif
}

/* A nonnull base is a live malloc/System allocation start. A null base marks
 * an address range after logical release: query only the OS, never malloc_size
 * or freed storage. No payload pointer is dereferenced in either case. */
void an_snapshot(const char *phase, const void *const *pointers,
                 const size_t *bytes, const void *const *bases,
                 size_t count, double elapsed) {
    double start = seconds();
    uint64_t before = rss();
    size_t page = (size_t)sysconf(_SC_PAGESIZE);
    struct rusage usage;
    assert(getrusage(RUSAGE_SELF, &usage) == 0);
    uint64_t peak = (uint64_t)usage.ru_maxrss;
#ifndef __APPLE__
    peak *= 1024;
#endif
    printf("{\"phase\":\"%s\",\"operation_s\":%.9f,\"rss_bytes\":%" PRIu64
           ",\"maxrss_bytes\":%" PRIu64 ",\"minor_faults\":%ld,\"major_faults\":%ld,"
           "\"user_s\":%.6f,\"system_s\":%.6f,\"page_size\":%zu,\"ranges\":[",
           phase, elapsed, before, peak, usage.ru_minflt, usage.ru_majflt,
           usage.ru_utime.tv_sec + usage.ru_utime.tv_usec * 1e-6,
           usage.ru_stime.tv_sec + usage.ru_stime.tv_usec * 1e-6, page);
    for (size_t i = 0; i < count; ++i) {
        uintptr_t address = (uintptr_t)pointers[i];
        uintptr_t aligned = address - address % page;
        size_t span = bytes[i] + address - aligned;
        size_t pages = (span + page - 1) / page;
        unsigned char *vec = calloc(pages, 1);
        assert(vec);
        int result = mincore((void *)aligned, span, (void *)vec);
        int error = result == 0 ? 0 : errno;
        size_t resident = 0;
        if (result == 0)
            for (size_t j = 0; j < pages; ++j) resident += (vec[j] & 1) != 0;
#ifdef __APPLE__
        size_t usable = bases[i] ? malloc_size(bases[i]) : 0;
        mach_vm_address_t region = address;
        mach_vm_size_t region_size = 0;
        vm_region_basic_info_data_64_t info;
        mach_msg_type_number_t n = VM_REGION_BASIC_INFO_COUNT_64;
        mach_port_t object = MACH_PORT_NULL;
        kern_return_t status = mach_vm_region(mach_task_self(), &region, &region_size,
                             VM_REGION_BASIC_INFO_64, (vm_region_info_t)&info, &n, &object);
        if (status != KERN_SUCCESS || region > address) { region = 0; region_size = 0; }
        if (object != MACH_PORT_NULL) mach_port_deallocate(mach_task_self(), object);
#else
        size_t usable = bases[i] ? malloc_usable_size((void *)bases[i]) : 0;
        uint64_t region = 0, region_size = 0, lo, hi;
        char line[512];
        FILE *maps = fopen("/proc/self/maps", "r");
        assert(maps);
        while (fgets(line, sizeof(line), maps))
            if (sscanf(line, "%" SCNx64 "-%" SCNx64, &lo, &hi) == 2 &&
                lo <= address && address < hi) { region = lo; region_size = hi - lo; break; }
        fclose(maps);
#endif
        printf("%s{\"start\":%" PRIuPTR ",\"end\":%" PRIuPTR
               ",\"payload_bytes\":%zu,\"allocation_live\":%s,\"allocator_usable_bytes\":%zu,"
               "\"mapped_region_start\":%" PRIu64 ",\"mapped_region_bytes\":%" PRIu64
               ",\"pages\":%zu,\"resident_pages\":%zu,\"mincore_errno\":%d}",
               i ? "," : "", address, address + bytes[i], bytes[i], bases[i] ? "true" : "false", usable,
               (uint64_t)region, (uint64_t)region_size, pages, resident, error);
        free(vec);
    }
    printf("],\"rss_after_observer_bytes\":%" PRIu64 ",\"observer_s\":%.9f}\n",
           rss(), seconds() - start);
    fflush(stdout);
}
