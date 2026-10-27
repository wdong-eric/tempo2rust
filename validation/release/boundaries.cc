#include "tempo2.h"
#include <cstdio>
#include <cstring>
int main() {
    const char* cases[][4] = {
        {"half_below","nlong","0x1.ffffffffffffffffffffffffffffp-2","0"},
        {"negative_half_inside","nlong","-0x1.ffffffffffffffffffffffffffffp-2","0"},
        {"half","nlong","0.5","0"},{"negative_half","nlong","-0.5","0"},
        {"half_above","nlong","0x1.0000000000000000000000000001p-1","0"},
        {"one_half","nlong","1.5","0"},{"negative_one_half","nlong","-1.5","0"},
        {"integer_limit","nlong","9223372036854775807","0"},
        {"mod_integer","mod","5.5","2"},{"mod_negative_exact","mod","-4","2"},
        {"mod_negative_zero_divisor","mod","-0","-2"},
        {"mod_decimal","mod","0.3","0.1"},{"mod_rounded_quotient","mod","1","0.1"},
        {"mod_negative_quotient","mod","-1","0.1"},
        {"mod_infinite_divisor","mod","1","inf"},{"mod_zero_infinite_divisor","mod","0","inf"}};
    for (const auto& row : cases) {
        longdouble a=strtoflt128(row[2],nullptr),b=strtoflt128(row[3],nullptr);
        longdouble value=strcmp(row[1],"nlong")==0 ? fortran_nlong(a) : fortran_mod(a,b);
        char x[128],y[128],z[128];
        quadmath_snprintf(x,sizeof x,"%.28Qa",a);quadmath_snprintf(y,sizeof y,"%.28Qa",b);quadmath_snprintf(z,sizeof z,"%.28Qa",value);
        printf("%s\t%s\t%s\t%s\t%s\n",row[0],row[1],x,y,z);
    }
}
