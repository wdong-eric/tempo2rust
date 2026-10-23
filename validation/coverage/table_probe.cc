// Assignment F probe of raw canonical table routines linked before A2 support library.
// Does not call initialiseOne, timing, fitting, or undefined singleton interpolation.
#include "tempo2.h"
#include "tabulatedfunction.h"
#include <cmath>
#include <cstdio>
#include <string>

int main(int argc, char **argv) {
    if (argc != 3) return 2;
    const char *names[] = {"skipped_row", "all_duplicate", "duplicate_end", "singleton_proxy", "decreasing_proxy"};
    const double epochs[] = {1.5, 1.0, 2.0, 0.0, 1.5};
    FILE *output = std::fopen(argv[2], "w");
    if (!output) return 3;
    std::fprintf(output, "{\"tables\":[");
    for (int i = 0; i < 5; ++i) {
        std::string path = std::string(argv[1]) + "/" + names[i] + ".clk";
        TabulatedFunction table;
        TabulatedFunction_load(&table, &path[0]);
        const double value = TabulatedFunction_getValue(&table, epochs[i]);
        std::fprintf(output, "%s{\"case\":\"%s\",\"samples\":%zu,\"value\":\"",
                     i ? "," : "", names[i], table.samples.size());
        if (std::isnan(value)) std::fprintf(output, "NaN");
        else if (std::isinf(value)) std::fprintf(output, "%sInfinity", value > 0 ? "+" : "-");
        else std::fprintf(output, "%.17g", value);
        std::fprintf(output, "\"}");
    }
    const longdouble prefix = parse_longdouble("1junk");
    const longdouble hex = parse_longdouble("0x1p0");
    const longdouble nan = parse_longdouble("NaN");
    std::fprintf(output, "],\"parser\":{\"numeric_prefix\":%.17g,\"hex\":%.17g,\"nan_class\":\"%s\"}}\n",
                 (double)prefix, (double)hex, std::isnan((double)nan) ? "NaN" : "other");
    std::fclose(output);
    return 0;
}
