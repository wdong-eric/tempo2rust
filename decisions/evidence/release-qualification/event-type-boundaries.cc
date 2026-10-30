// formResiduals.C:675-728 and TKlongdouble.float128.h at the pinned source.
// Unevaluated C++ expressions check the type promotions; no timing model runs.
#include "TKlongdouble.float128.h"
#include <cstdio>
#include <type_traits>
#include <cfloat>
static double freqSSB;
static long double freq, dt, dm, tau, gamma, amp, sig, val;
static longdouble bbat, epoch, spin, phase4;
static_assert(std::is_same_v<decltype(freqSSB/1.4e9),double>);
static_assert(std::is_same_v<decltype(bbat-epoch),longdouble>);
static_assert(std::is_same_v<decltype(powl(freq,gamma)),longdouble>);
static_assert(std::is_same_v<decltype(-dt/tau),long double>);
static_assert(std::is_same_v<decltype(exp(-dt/tau)),long double>);
static_assert(std::is_same_v<decltype(dm*powl(freq,gamma)),longdouble>);
static_assert(std::is_same_v<decltype(dm*powl(freq,gamma)*exp(-dt/tau)*spin),longdouble>);
static_assert(std::is_same_v<decltype(-dt*dt/2./sig/sig),long double>);
static_assert(std::is_same_v<decltype(amp*powl(freq,gamma)*exp(-dt*dt/2./sig/sig)),longdouble>);
static_assert(std::is_same_v<decltype(val*spin),longdouble>);
static_assert(std::is_same_v<decltype(phase4+val*spin),longdouble>);
int main() {
 std::printf("ordinary_long_double_bits\t%d\n",LDBL_MANT_DIG);
 std::puts("frequency_division\tdouble");
 std::puts("epoch_subtraction\tbinary128_then_assign_ordinary_long_double");
 std::puts("powl_macro\tbinary128");
 std::puts("exponential_argument_and_result\tordinary_long_double");
 std::puts("exponential_event_product\tbinary128");
 std::puts("gaussian_product\tbinary128_then_assign_ordinary_long_double_val");
 std::puts("phase4_accumulation\tbinary128");
}
