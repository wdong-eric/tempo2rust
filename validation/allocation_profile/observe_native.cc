// Source-bound native factory/initialiseOne control; never patch canonical A2.
#include "tempo2.h"
#include "TKmatrix.h"
#include <cassert>
#include <chrono>
#include <cstdlib>
#include <cstring>
#include <vector>
extern "C" void an_snapshot(const char *, const void *const *, const size_t *,
                             const void *const *, size_t, double);
using Clock = std::chrono::steady_clock;
static void snapshot(const char *phase, const std::vector<double **>& matrices, double elapsed) {
    std::vector<const void *> pointers, bases;
    std::vector<size_t> bytes;
    for (auto m: matrices) {
        pointers.push_back(m[0]); bases.push_back(m[0] - 2);
        bytes.push_back(size_t(MAX_FIT) * MAX_FIT * sizeof(double));
    }
    an_snapshot(phase, pointers.data(), bytes.data(), bases.data(), matrices.size(), elapsed);
}
int main(int argc, char **argv) {
    assert(argc == 4);
    int count = atoi(argv[2]); assert(count == 1 || count == 2);
    bool context = strcmp(argv[1], "context") == 0;
    assert(context || strcmp(argv[1], "matrix") == 0);
    MAX_OBSN = 10000; MAX_PSR = count;
    snapshot("warmup", {}, 0); snapshot("start", {}, 0);
    std::vector<double **> matrices;
    // A2's constructor input uses zeroed struct storage; covariance still comes
    // from the unmodified TKmatrix.C calloc path, including its hidden header.
    pulsar *psr = context ? (pulsar *)calloc(count, sizeof(pulsar)) : nullptr;
    for (int i = 0; i < count; ++i) {
        auto t = Clock::now();
        if (context) { initialiseOne(psr + i, 1, 1); matrices.push_back(psr[i].covar); }
        else matrices.push_back(malloc_blas(MAX_FIT, MAX_FIT));
        double elapsed = std::chrono::duration<double>(Clock::now() - t).count();
        snapshot(context ? "initialised" : "allocated", matrices, elapsed);
    }
    if (strcmp(argv[3], "untouched") != 0) {
        auto t = Clock::now();
        for (auto m: matrices) {
            if (strcmp(argv[3], "write") == 0) {
                for (size_t i = 0; i < size_t(MAX_FIT)*MAX_FIT; ++i) m[0][i] = 1.0;
            } else {
                assert(strcmp(argv[3], "zero") == 0 || strcmp(argv[3], "read") == 0);
                volatile const double *cells = m[0];
                for (size_t i = 0; i < size_t(MAX_FIT)*MAX_FIT; ++i) {
                    double value = cells[i]; uint64_t bits;
                    memcpy(&bits, &value, sizeof(bits)); assert(bits == 0);
                }
            }
        }
        snapshot("touched", matrices, std::chrono::duration<double>(Clock::now() - t).count());
    }
    std::vector<const void *> released, bases(count, nullptr);
    std::vector<size_t> bytes(count, size_t(MAX_FIT) * MAX_FIT * sizeof(double));
    for (auto m: matrices) released.push_back(m[0]);
    for (int i = 0; i < count; ++i) {
        if (context) destroyOne(psr + i);
        else free_blas(matrices[i]);
    }
    free(psr);
    an_snapshot("dropped", released.data(), bytes.data(), bases.data(), released.size(), 0);
}
