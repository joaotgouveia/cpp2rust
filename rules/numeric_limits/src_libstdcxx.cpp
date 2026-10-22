#include "../common/init.h"
#include "../common/traits.h"
#include <limits>
#include <utility>

using t5 = std::numeric_limits<char>;

char f1() { return std::numeric_limits<char>::min(); }

char f2() { return std::numeric_limits<char>::max(); }

char f3() { return std::numeric_limits<char>::lowest(); }

char f4() { return std::numeric_limits<char>::epsilon(); }

char f5() { return std::numeric_limits<char>::round_error(); }

char f6() { return std::numeric_limits<char>::infinity(); }

char f7() { return std::numeric_limits<char>::quiet_NaN(); }

char f8() { return std::numeric_limits<char>::signaling_NaN(); }

char f9() { return std::numeric_limits<char>::denorm_min(); }

std::numeric_limits<char> f10() { return std::numeric_limits<char>(); }

std::numeric_limits<char> f11(const std::numeric_limits<char> &a0) {
  return std::numeric_limits<char>(a0);
}

std::numeric_limits<char> f12(std::numeric_limits<char> &&a0) {
  return std::numeric_limits<char>(std::move(a0));
}

std::numeric_limits<char> &f13(std::numeric_limits<char> &a0,
                               const std::numeric_limits<char> &a1) {
  return a0.operator=(a1);
}

std::numeric_limits<char> &f14(std::numeric_limits<char> &a0,
                               std::numeric_limits<char> &&a1) {
  return a0.operator=(std::move(a1));
}

bool f15() { return std::numeric_limits<char>::is_specialized; }

int f16() { return std::numeric_limits<char>::digits; }

int f17() { return std::numeric_limits<char>::digits10; }

int f18() { return std::numeric_limits<char>::max_digits10; }

bool f19() { return std::numeric_limits<char>::is_signed; }

bool f20() { return std::numeric_limits<char>::is_integer; }

bool f21() { return std::numeric_limits<char>::is_exact; }

int f22() { return std::numeric_limits<char>::radix; }

int f23() { return std::numeric_limits<char>::min_exponent; }

int f24() { return std::numeric_limits<char>::min_exponent10; }

int f25() { return std::numeric_limits<char>::max_exponent; }

int f26() { return std::numeric_limits<char>::max_exponent10; }

bool f27() { return std::numeric_limits<char>::has_infinity; }

bool f28() { return std::numeric_limits<char>::has_quiet_NaN; }

bool f29() { return std::numeric_limits<char>::has_signaling_NaN; }

std::float_denorm_style f30() { return std::numeric_limits<char>::has_denorm; }

bool f31() { return std::numeric_limits<char>::has_denorm_loss; }

bool f32() { return std::numeric_limits<char>::is_iec559; }

bool f33() { return std::numeric_limits<char>::is_bounded; }

bool f34() { return std::numeric_limits<char>::is_modulo; }

bool f35() { return std::numeric_limits<char>::traps; }

bool f36() { return std::numeric_limits<char>::tinyness_before; }

std::float_round_style f37() { return std::numeric_limits<char>::round_style; }

using t6 = std::numeric_limits<long double>;

long double f38() { return std::numeric_limits<long double>::min(); }

long double f39() { return std::numeric_limits<long double>::max(); }

long double f40() { return std::numeric_limits<long double>::lowest(); }

long double f41() { return std::numeric_limits<long double>::epsilon(); }

long double f42() { return std::numeric_limits<long double>::round_error(); }

long double f43() { return std::numeric_limits<long double>::infinity(); }

long double f44() { return std::numeric_limits<long double>::quiet_NaN(); }

long double f45() { return std::numeric_limits<long double>::signaling_NaN(); }

long double f46() { return std::numeric_limits<long double>::denorm_min(); }

std::numeric_limits<long double> f47() {
  return std::numeric_limits<long double>();
}

std::numeric_limits<long double>
f48(const std::numeric_limits<long double> &a0) {
  return std::numeric_limits<long double>(a0);
}

std::numeric_limits<long double> f49(std::numeric_limits<long double> &&a0) {
  return std::numeric_limits<long double>(std::move(a0));
}

std::numeric_limits<long double> &
f50(std::numeric_limits<long double> &a0,
    const std::numeric_limits<long double> &a1) {
  return a0.operator=(a1);
}

std::numeric_limits<long double> &f51(std::numeric_limits<long double> &a0,
                                      std::numeric_limits<long double> &&a1) {
  return a0.operator=(std::move(a1));
}

bool f52() { return std::numeric_limits<long double>::is_specialized; }

int f53() { return std::numeric_limits<long double>::digits; }

int f54() { return std::numeric_limits<long double>::digits10; }

int f55() { return std::numeric_limits<long double>::max_digits10; }

bool f56() { return std::numeric_limits<long double>::is_signed; }

bool f57() { return std::numeric_limits<long double>::is_integer; }

bool f58() { return std::numeric_limits<long double>::is_exact; }

int f59() { return std::numeric_limits<long double>::radix; }

int f60() { return std::numeric_limits<long double>::min_exponent; }

int f61() { return std::numeric_limits<long double>::min_exponent10; }

int f62() { return std::numeric_limits<long double>::max_exponent; }

int f63() { return std::numeric_limits<long double>::max_exponent10; }

bool f64() { return std::numeric_limits<long double>::has_infinity; }

bool f65() { return std::numeric_limits<long double>::has_quiet_NaN; }

bool f66() { return std::numeric_limits<long double>::has_signaling_NaN; }

std::float_denorm_style f67() {
  return std::numeric_limits<long double>::has_denorm;
}

bool f68() { return std::numeric_limits<long double>::has_denorm_loss; }

bool f69() { return std::numeric_limits<long double>::is_iec559; }

bool f70() { return std::numeric_limits<long double>::is_bounded; }

bool f71() { return std::numeric_limits<long double>::is_modulo; }

bool f72() { return std::numeric_limits<long double>::traps; }

bool f73() { return std::numeric_limits<long double>::tinyness_before; }

std::float_round_style f74() {
  return std::numeric_limits<long double>::round_style;
}

using t7 = std::numeric_limits<int>;

int f75() { return std::numeric_limits<int>::min(); }

int f76() { return std::numeric_limits<int>::max(); }

int f77() { return std::numeric_limits<int>::lowest(); }

int f78() { return std::numeric_limits<int>::epsilon(); }

int f79() { return std::numeric_limits<int>::round_error(); }

int f80() { return std::numeric_limits<int>::infinity(); }

int f81() { return std::numeric_limits<int>::quiet_NaN(); }

int f82() { return std::numeric_limits<int>::signaling_NaN(); }

int f83() { return std::numeric_limits<int>::denorm_min(); }

std::numeric_limits<int> f84() { return std::numeric_limits<int>(); }

std::numeric_limits<int> f85(const std::numeric_limits<int> &a0) {
  return std::numeric_limits<int>(a0);
}

std::numeric_limits<int> f86(std::numeric_limits<int> &&a0) {
  return std::numeric_limits<int>(std::move(a0));
}

std::numeric_limits<int> &f87(std::numeric_limits<int> &a0,
                              const std::numeric_limits<int> &a1) {
  return a0.operator=(a1);
}

std::numeric_limits<int> &f88(std::numeric_limits<int> &a0,
                              std::numeric_limits<int> &&a1) {
  return a0.operator=(std::move(a1));
}

bool f89() { return std::numeric_limits<int>::is_specialized; }

int f90() { return std::numeric_limits<int>::digits; }

int f91() { return std::numeric_limits<int>::digits10; }

int f92() { return std::numeric_limits<int>::max_digits10; }

bool f93() { return std::numeric_limits<int>::is_signed; }

bool f94() { return std::numeric_limits<int>::is_integer; }

bool f95() { return std::numeric_limits<int>::is_exact; }

int f96() { return std::numeric_limits<int>::radix; }

int f97() { return std::numeric_limits<int>::min_exponent; }

int f98() { return std::numeric_limits<int>::min_exponent10; }

int f99() { return std::numeric_limits<int>::max_exponent; }

int f100() { return std::numeric_limits<int>::max_exponent10; }

bool f101() { return std::numeric_limits<int>::has_infinity; }

bool f102() { return std::numeric_limits<int>::has_quiet_NaN; }

bool f103() { return std::numeric_limits<int>::has_signaling_NaN; }

std::float_denorm_style f104() { return std::numeric_limits<int>::has_denorm; }

bool f105() { return std::numeric_limits<int>::has_denorm_loss; }

bool f106() { return std::numeric_limits<int>::is_iec559; }

bool f107() { return std::numeric_limits<int>::is_bounded; }

bool f108() { return std::numeric_limits<int>::is_modulo; }

bool f109() { return std::numeric_limits<int>::traps; }

bool f110() { return std::numeric_limits<int>::tinyness_before; }

std::float_round_style f111() { return std::numeric_limits<int>::round_style; }

using t8 = std::numeric_limits<long long>;

long long f112() { return std::numeric_limits<long long>::min(); }

long long f113() { return std::numeric_limits<long long>::max(); }

long long f114() { return std::numeric_limits<long long>::lowest(); }

long long f115() { return std::numeric_limits<long long>::epsilon(); }

long long f116() { return std::numeric_limits<long long>::round_error(); }

long long f117() { return std::numeric_limits<long long>::infinity(); }

long long f118() { return std::numeric_limits<long long>::quiet_NaN(); }

long long f119() { return std::numeric_limits<long long>::signaling_NaN(); }

long long f120() { return std::numeric_limits<long long>::denorm_min(); }

std::numeric_limits<long long> f121() {
  return std::numeric_limits<long long>();
}

std::numeric_limits<long long> f122(const std::numeric_limits<long long> &a0) {
  return std::numeric_limits<long long>(a0);
}

std::numeric_limits<long long> f123(std::numeric_limits<long long> &&a0) {
  return std::numeric_limits<long long>(std::move(a0));
}

std::numeric_limits<long long> &f124(std::numeric_limits<long long> &a0,
                                     const std::numeric_limits<long long> &a1) {
  return a0.operator=(a1);
}

std::numeric_limits<long long> &f125(std::numeric_limits<long long> &a0,
                                     std::numeric_limits<long long> &&a1) {
  return a0.operator=(std::move(a1));
}

bool f126() { return std::numeric_limits<long long>::is_specialized; }

int f127() { return std::numeric_limits<long long>::digits; }

int f128() { return std::numeric_limits<long long>::digits10; }

int f129() { return std::numeric_limits<long long>::max_digits10; }

bool f130() { return std::numeric_limits<long long>::is_signed; }

bool f131() { return std::numeric_limits<long long>::is_integer; }

bool f132() { return std::numeric_limits<long long>::is_exact; }

int f133() { return std::numeric_limits<long long>::radix; }

int f134() { return std::numeric_limits<long long>::min_exponent; }

int f135() { return std::numeric_limits<long long>::min_exponent10; }

int f136() { return std::numeric_limits<long long>::max_exponent; }

int f137() { return std::numeric_limits<long long>::max_exponent10; }

bool f138() { return std::numeric_limits<long long>::has_infinity; }

bool f139() { return std::numeric_limits<long long>::has_quiet_NaN; }

bool f140() { return std::numeric_limits<long long>::has_signaling_NaN; }

std::float_denorm_style f141() {
  return std::numeric_limits<long long>::has_denorm;
}

bool f142() { return std::numeric_limits<long long>::has_denorm_loss; }

bool f143() { return std::numeric_limits<long long>::is_iec559; }

bool f144() { return std::numeric_limits<long long>::is_bounded; }

bool f145() { return std::numeric_limits<long long>::is_modulo; }

bool f146() { return std::numeric_limits<long long>::traps; }

bool f147() { return std::numeric_limits<long long>::tinyness_before; }

std::float_round_style f148() {
  return std::numeric_limits<long long>::round_style;
}

using t9 = std::numeric_limits<long>;

long f149() { return std::numeric_limits<long>::min(); }

long f150() { return std::numeric_limits<long>::max(); }

long f151() { return std::numeric_limits<long>::lowest(); }

long f152() { return std::numeric_limits<long>::epsilon(); }

long f153() { return std::numeric_limits<long>::round_error(); }

long f154() { return std::numeric_limits<long>::infinity(); }

long f155() { return std::numeric_limits<long>::quiet_NaN(); }

long f156() { return std::numeric_limits<long>::signaling_NaN(); }

long f157() { return std::numeric_limits<long>::denorm_min(); }

std::numeric_limits<long> f158() { return std::numeric_limits<long>(); }

std::numeric_limits<long> f159(const std::numeric_limits<long> &a0) {
  return std::numeric_limits<long>(a0);
}

std::numeric_limits<long> f160(std::numeric_limits<long> &&a0) {
  return std::numeric_limits<long>(std::move(a0));
}

std::numeric_limits<long> &f161(std::numeric_limits<long> &a0,
                                const std::numeric_limits<long> &a1) {
  return a0.operator=(a1);
}

std::numeric_limits<long> &f162(std::numeric_limits<long> &a0,
                                std::numeric_limits<long> &&a1) {
  return a0.operator=(std::move(a1));
}

bool f163() { return std::numeric_limits<long>::is_specialized; }

int f164() { return std::numeric_limits<long>::digits; }

int f165() { return std::numeric_limits<long>::digits10; }

int f166() { return std::numeric_limits<long>::max_digits10; }

bool f167() { return std::numeric_limits<long>::is_signed; }

bool f168() { return std::numeric_limits<long>::is_integer; }

bool f169() { return std::numeric_limits<long>::is_exact; }

int f170() { return std::numeric_limits<long>::radix; }

int f171() { return std::numeric_limits<long>::min_exponent; }

int f172() { return std::numeric_limits<long>::min_exponent10; }

int f173() { return std::numeric_limits<long>::max_exponent; }

int f174() { return std::numeric_limits<long>::max_exponent10; }

bool f175() { return std::numeric_limits<long>::has_infinity; }

bool f176() { return std::numeric_limits<long>::has_quiet_NaN; }

bool f177() { return std::numeric_limits<long>::has_signaling_NaN; }

std::float_denorm_style f178() { return std::numeric_limits<long>::has_denorm; }

bool f179() { return std::numeric_limits<long>::has_denorm_loss; }

bool f180() { return std::numeric_limits<long>::is_iec559; }

bool f181() { return std::numeric_limits<long>::is_bounded; }

bool f182() { return std::numeric_limits<long>::is_modulo; }

bool f183() { return std::numeric_limits<long>::traps; }

bool f184() { return std::numeric_limits<long>::tinyness_before; }

std::float_round_style f185() { return std::numeric_limits<long>::round_style; }

using t10 = std::numeric_limits<short>;

short f186() { return std::numeric_limits<short>::min(); }

short f187() { return std::numeric_limits<short>::max(); }

short f188() { return std::numeric_limits<short>::lowest(); }

short f189() { return std::numeric_limits<short>::epsilon(); }

short f190() { return std::numeric_limits<short>::round_error(); }

short f191() { return std::numeric_limits<short>::infinity(); }

short f192() { return std::numeric_limits<short>::quiet_NaN(); }

short f193() { return std::numeric_limits<short>::signaling_NaN(); }

short f194() { return std::numeric_limits<short>::denorm_min(); }

std::numeric_limits<short> f195() { return std::numeric_limits<short>(); }

std::numeric_limits<short> f196(const std::numeric_limits<short> &a0) {
  return std::numeric_limits<short>(a0);
}

std::numeric_limits<short> f197(std::numeric_limits<short> &&a0) {
  return std::numeric_limits<short>(std::move(a0));
}

std::numeric_limits<short> &f198(std::numeric_limits<short> &a0,
                                 const std::numeric_limits<short> &a1) {
  return a0.operator=(a1);
}

std::numeric_limits<short> &f199(std::numeric_limits<short> &a0,
                                 std::numeric_limits<short> &&a1) {
  return a0.operator=(std::move(a1));
}

bool f200() { return std::numeric_limits<short>::is_specialized; }

int f201() { return std::numeric_limits<short>::digits; }

int f202() { return std::numeric_limits<short>::digits10; }

int f203() { return std::numeric_limits<short>::max_digits10; }

bool f204() { return std::numeric_limits<short>::is_signed; }

bool f205() { return std::numeric_limits<short>::is_integer; }

bool f206() { return std::numeric_limits<short>::is_exact; }

int f207() { return std::numeric_limits<short>::radix; }

int f208() { return std::numeric_limits<short>::min_exponent; }

int f209() { return std::numeric_limits<short>::min_exponent10; }

int f210() { return std::numeric_limits<short>::max_exponent; }

int f211() { return std::numeric_limits<short>::max_exponent10; }

bool f212() { return std::numeric_limits<short>::has_infinity; }

bool f213() { return std::numeric_limits<short>::has_quiet_NaN; }

bool f214() { return std::numeric_limits<short>::has_signaling_NaN; }

std::float_denorm_style f215() {
  return std::numeric_limits<short>::has_denorm;
}

bool f216() { return std::numeric_limits<short>::has_denorm_loss; }

bool f217() { return std::numeric_limits<short>::is_iec559; }

bool f218() { return std::numeric_limits<short>::is_bounded; }

bool f219() { return std::numeric_limits<short>::is_modulo; }

bool f220() { return std::numeric_limits<short>::traps; }

bool f221() { return std::numeric_limits<short>::tinyness_before; }

std::float_round_style f222() {
  return std::numeric_limits<short>::round_style;
}

using t11 = std::numeric_limits<wchar_t>;

wchar_t f223() { return std::numeric_limits<wchar_t>::min(); }

wchar_t f224() { return std::numeric_limits<wchar_t>::max(); }

wchar_t f225() { return std::numeric_limits<wchar_t>::lowest(); }

wchar_t f226() { return std::numeric_limits<wchar_t>::epsilon(); }

wchar_t f227() { return std::numeric_limits<wchar_t>::round_error(); }

wchar_t f228() { return std::numeric_limits<wchar_t>::infinity(); }

wchar_t f229() { return std::numeric_limits<wchar_t>::quiet_NaN(); }

wchar_t f230() { return std::numeric_limits<wchar_t>::signaling_NaN(); }

wchar_t f231() { return std::numeric_limits<wchar_t>::denorm_min(); }

std::numeric_limits<wchar_t> f232() { return std::numeric_limits<wchar_t>(); }

std::numeric_limits<wchar_t> f233(const std::numeric_limits<wchar_t> &a0) {
  return std::numeric_limits<wchar_t>(a0);
}

std::numeric_limits<wchar_t> f234(std::numeric_limits<wchar_t> &&a0) {
  return std::numeric_limits<wchar_t>(std::move(a0));
}

std::numeric_limits<wchar_t> &f235(std::numeric_limits<wchar_t> &a0,
                                   const std::numeric_limits<wchar_t> &a1) {
  return a0.operator=(a1);
}

std::numeric_limits<wchar_t> &f236(std::numeric_limits<wchar_t> &a0,
                                   std::numeric_limits<wchar_t> &&a1) {
  return a0.operator=(std::move(a1));
}

bool f237() { return std::numeric_limits<wchar_t>::is_specialized; }

int f238() { return std::numeric_limits<wchar_t>::digits; }

int f239() { return std::numeric_limits<wchar_t>::digits10; }

int f240() { return std::numeric_limits<wchar_t>::max_digits10; }

bool f241() { return std::numeric_limits<wchar_t>::is_signed; }

bool f242() { return std::numeric_limits<wchar_t>::is_integer; }

bool f243() { return std::numeric_limits<wchar_t>::is_exact; }

int f244() { return std::numeric_limits<wchar_t>::radix; }

int f245() { return std::numeric_limits<wchar_t>::min_exponent; }

int f246() { return std::numeric_limits<wchar_t>::min_exponent10; }

int f247() { return std::numeric_limits<wchar_t>::max_exponent; }

int f248() { return std::numeric_limits<wchar_t>::max_exponent10; }

bool f249() { return std::numeric_limits<wchar_t>::has_infinity; }

bool f250() { return std::numeric_limits<wchar_t>::has_quiet_NaN; }

bool f251() { return std::numeric_limits<wchar_t>::has_signaling_NaN; }

std::float_denorm_style f252() {
  return std::numeric_limits<wchar_t>::has_denorm;
}

bool f253() { return std::numeric_limits<wchar_t>::has_denorm_loss; }

bool f254() { return std::numeric_limits<wchar_t>::is_iec559; }

bool f255() { return std::numeric_limits<wchar_t>::is_bounded; }

bool f256() { return std::numeric_limits<wchar_t>::is_modulo; }

bool f257() { return std::numeric_limits<wchar_t>::traps; }

bool f258() { return std::numeric_limits<wchar_t>::tinyness_before; }

std::float_round_style f259() {
  return std::numeric_limits<wchar_t>::round_style;
}

using t12 = std::numeric_limits<bool>;

bool f260() { return std::numeric_limits<bool>::min(); }

bool f261() { return std::numeric_limits<bool>::max(); }

bool f262() { return std::numeric_limits<bool>::lowest(); }

bool f263() { return std::numeric_limits<bool>::epsilon(); }

bool f264() { return std::numeric_limits<bool>::round_error(); }

bool f265() { return std::numeric_limits<bool>::infinity(); }

bool f266() { return std::numeric_limits<bool>::quiet_NaN(); }

bool f267() { return std::numeric_limits<bool>::signaling_NaN(); }

bool f268() { return std::numeric_limits<bool>::denorm_min(); }

std::numeric_limits<bool> f269() { return std::numeric_limits<bool>(); }

std::numeric_limits<bool> f270(const std::numeric_limits<bool> &a0) {
  return std::numeric_limits<bool>(a0);
}

std::numeric_limits<bool> f271(std::numeric_limits<bool> &&a0) {
  return std::numeric_limits<bool>(std::move(a0));
}

std::numeric_limits<bool> &f272(std::numeric_limits<bool> &a0,
                                const std::numeric_limits<bool> &a1) {
  return a0.operator=(a1);
}

std::numeric_limits<bool> &f273(std::numeric_limits<bool> &a0,
                                std::numeric_limits<bool> &&a1) {
  return a0.operator=(std::move(a1));
}

bool f274() { return std::numeric_limits<bool>::is_specialized; }

int f275() { return std::numeric_limits<bool>::digits; }

int f276() { return std::numeric_limits<bool>::digits10; }

int f277() { return std::numeric_limits<bool>::max_digits10; }

bool f278() { return std::numeric_limits<bool>::is_signed; }

bool f279() { return std::numeric_limits<bool>::is_integer; }

bool f280() { return std::numeric_limits<bool>::is_exact; }

int f281() { return std::numeric_limits<bool>::radix; }

int f282() { return std::numeric_limits<bool>::min_exponent; }

int f283() { return std::numeric_limits<bool>::min_exponent10; }

int f284() { return std::numeric_limits<bool>::max_exponent; }

int f285() { return std::numeric_limits<bool>::max_exponent10; }

bool f286() { return std::numeric_limits<bool>::has_infinity; }

bool f287() { return std::numeric_limits<bool>::has_quiet_NaN; }

bool f288() { return std::numeric_limits<bool>::has_signaling_NaN; }

std::float_denorm_style f289() { return std::numeric_limits<bool>::has_denorm; }

bool f290() { return std::numeric_limits<bool>::has_denorm_loss; }

bool f291() { return std::numeric_limits<bool>::is_iec559; }

bool f292() { return std::numeric_limits<bool>::is_bounded; }

bool f293() { return std::numeric_limits<bool>::is_modulo; }

bool f294() { return std::numeric_limits<bool>::traps; }

bool f295() { return std::numeric_limits<bool>::tinyness_before; }

std::float_round_style f296() { return std::numeric_limits<bool>::round_style; }

using t13 = std::numeric_limits<unsigned char>;

unsigned char f297() { return std::numeric_limits<unsigned char>::min(); }

unsigned char f298() { return std::numeric_limits<unsigned char>::max(); }

unsigned char f299() { return std::numeric_limits<unsigned char>::lowest(); }

unsigned char f300() { return std::numeric_limits<unsigned char>::epsilon(); }

unsigned char f301() {
  return std::numeric_limits<unsigned char>::round_error();
}

unsigned char f302() { return std::numeric_limits<unsigned char>::infinity(); }

unsigned char f303() { return std::numeric_limits<unsigned char>::quiet_NaN(); }

unsigned char f304() {
  return std::numeric_limits<unsigned char>::signaling_NaN();
}

unsigned char f305() {
  return std::numeric_limits<unsigned char>::denorm_min();
}

std::numeric_limits<unsigned char> f306() {
  return std::numeric_limits<unsigned char>();
}

std::numeric_limits<unsigned char>
f307(const std::numeric_limits<unsigned char> &a0) {
  return std::numeric_limits<unsigned char>(a0);
}

std::numeric_limits<unsigned char>
f308(std::numeric_limits<unsigned char> &&a0) {
  return std::numeric_limits<unsigned char>(std::move(a0));
}

std::numeric_limits<unsigned char> &
f309(std::numeric_limits<unsigned char> &a0,
     const std::numeric_limits<unsigned char> &a1) {
  return a0.operator=(a1);
}

std::numeric_limits<unsigned char> &
f310(std::numeric_limits<unsigned char> &a0,
     std::numeric_limits<unsigned char> &&a1) {
  return a0.operator=(std::move(a1));
}

bool f311() { return std::numeric_limits<unsigned char>::is_specialized; }

int f312() { return std::numeric_limits<unsigned char>::digits; }

int f313() { return std::numeric_limits<unsigned char>::digits10; }

int f314() { return std::numeric_limits<unsigned char>::max_digits10; }

bool f315() { return std::numeric_limits<unsigned char>::is_signed; }

bool f316() { return std::numeric_limits<unsigned char>::is_integer; }

bool f317() { return std::numeric_limits<unsigned char>::is_exact; }

int f318() { return std::numeric_limits<unsigned char>::radix; }

int f319() { return std::numeric_limits<unsigned char>::min_exponent; }

int f320() { return std::numeric_limits<unsigned char>::min_exponent10; }

int f321() { return std::numeric_limits<unsigned char>::max_exponent; }

int f322() { return std::numeric_limits<unsigned char>::max_exponent10; }

bool f323() { return std::numeric_limits<unsigned char>::has_infinity; }

bool f324() { return std::numeric_limits<unsigned char>::has_quiet_NaN; }

bool f325() { return std::numeric_limits<unsigned char>::has_signaling_NaN; }

std::float_denorm_style f326() {
  return std::numeric_limits<unsigned char>::has_denorm;
}

bool f327() { return std::numeric_limits<unsigned char>::has_denorm_loss; }

bool f328() { return std::numeric_limits<unsigned char>::is_iec559; }

bool f329() { return std::numeric_limits<unsigned char>::is_bounded; }

bool f330() { return std::numeric_limits<unsigned char>::is_modulo; }

bool f331() { return std::numeric_limits<unsigned char>::traps; }

bool f332() { return std::numeric_limits<unsigned char>::tinyness_before; }

std::float_round_style f333() {
  return std::numeric_limits<unsigned char>::round_style;
}

using t14 = std::numeric_limits<double>;

double f334() { return std::numeric_limits<double>::min(); }

double f335() { return std::numeric_limits<double>::max(); }

double f336() { return std::numeric_limits<double>::lowest(); }

double f337() { return std::numeric_limits<double>::epsilon(); }

double f338() { return std::numeric_limits<double>::round_error(); }

double f339() { return std::numeric_limits<double>::infinity(); }

double f340() { return std::numeric_limits<double>::quiet_NaN(); }

double f341() { return std::numeric_limits<double>::signaling_NaN(); }

double f342() { return std::numeric_limits<double>::denorm_min(); }

std::numeric_limits<double> f343() { return std::numeric_limits<double>(); }

std::numeric_limits<double> f344(const std::numeric_limits<double> &a0) {
  return std::numeric_limits<double>(a0);
}

std::numeric_limits<double> f345(std::numeric_limits<double> &&a0) {
  return std::numeric_limits<double>(std::move(a0));
}

std::numeric_limits<double> &f346(std::numeric_limits<double> &a0,
                                  const std::numeric_limits<double> &a1) {
  return a0.operator=(a1);
}

std::numeric_limits<double> &f347(std::numeric_limits<double> &a0,
                                  std::numeric_limits<double> &&a1) {
  return a0.operator=(std::move(a1));
}

bool f348() { return std::numeric_limits<double>::is_specialized; }

int f349() { return std::numeric_limits<double>::digits; }

int f350() { return std::numeric_limits<double>::digits10; }

int f351() { return std::numeric_limits<double>::max_digits10; }

bool f352() { return std::numeric_limits<double>::is_signed; }

bool f353() { return std::numeric_limits<double>::is_integer; }

bool f354() { return std::numeric_limits<double>::is_exact; }

int f355() { return std::numeric_limits<double>::radix; }

int f356() { return std::numeric_limits<double>::min_exponent; }

int f357() { return std::numeric_limits<double>::min_exponent10; }

int f358() { return std::numeric_limits<double>::max_exponent; }

int f359() { return std::numeric_limits<double>::max_exponent10; }

bool f360() { return std::numeric_limits<double>::has_infinity; }

bool f361() { return std::numeric_limits<double>::has_quiet_NaN; }

bool f362() { return std::numeric_limits<double>::has_signaling_NaN; }

std::float_denorm_style f363() {
  return std::numeric_limits<double>::has_denorm;
}

bool f364() { return std::numeric_limits<double>::has_denorm_loss; }

bool f365() { return std::numeric_limits<double>::is_iec559; }

bool f366() { return std::numeric_limits<double>::is_bounded; }

bool f367() { return std::numeric_limits<double>::is_modulo; }

bool f368() { return std::numeric_limits<double>::traps; }

bool f369() { return std::numeric_limits<double>::tinyness_before; }

std::float_round_style f370() {
  return std::numeric_limits<double>::round_style;
}

using t15 = std::numeric_limits<float>;

float f371() { return std::numeric_limits<float>::min(); }

float f372() { return std::numeric_limits<float>::max(); }

float f373() { return std::numeric_limits<float>::lowest(); }

float f374() { return std::numeric_limits<float>::epsilon(); }

float f375() { return std::numeric_limits<float>::round_error(); }

float f376() { return std::numeric_limits<float>::infinity(); }

float f377() { return std::numeric_limits<float>::quiet_NaN(); }

float f378() { return std::numeric_limits<float>::signaling_NaN(); }

float f379() { return std::numeric_limits<float>::denorm_min(); }

std::numeric_limits<float> f380() { return std::numeric_limits<float>(); }

std::numeric_limits<float> f381(const std::numeric_limits<float> &a0) {
  return std::numeric_limits<float>(a0);
}

std::numeric_limits<float> f382(std::numeric_limits<float> &&a0) {
  return std::numeric_limits<float>(std::move(a0));
}

std::numeric_limits<float> &f383(std::numeric_limits<float> &a0,
                                 const std::numeric_limits<float> &a1) {
  return a0.operator=(a1);
}

std::numeric_limits<float> &f384(std::numeric_limits<float> &a0,
                                 std::numeric_limits<float> &&a1) {
  return a0.operator=(std::move(a1));
}

bool f385() { return std::numeric_limits<float>::is_specialized; }

int f386() { return std::numeric_limits<float>::digits; }

int f387() { return std::numeric_limits<float>::digits10; }

int f388() { return std::numeric_limits<float>::max_digits10; }

bool f389() { return std::numeric_limits<float>::is_signed; }

bool f390() { return std::numeric_limits<float>::is_integer; }

bool f391() { return std::numeric_limits<float>::is_exact; }

int f392() { return std::numeric_limits<float>::radix; }

int f393() { return std::numeric_limits<float>::min_exponent; }

int f394() { return std::numeric_limits<float>::min_exponent10; }

int f395() { return std::numeric_limits<float>::max_exponent; }

int f396() { return std::numeric_limits<float>::max_exponent10; }

bool f397() { return std::numeric_limits<float>::has_infinity; }

bool f398() { return std::numeric_limits<float>::has_quiet_NaN; }

bool f399() { return std::numeric_limits<float>::has_signaling_NaN; }

std::float_denorm_style f400() {
  return std::numeric_limits<float>::has_denorm;
}

bool f401() { return std::numeric_limits<float>::has_denorm_loss; }

bool f402() { return std::numeric_limits<float>::is_iec559; }

bool f403() { return std::numeric_limits<float>::is_bounded; }

bool f404() { return std::numeric_limits<float>::is_modulo; }

bool f405() { return std::numeric_limits<float>::traps; }

bool f406() { return std::numeric_limits<float>::tinyness_before; }

std::float_round_style f407() {
  return std::numeric_limits<float>::round_style;
}

using t16 = std::numeric_limits<unsigned int>;

unsigned int f408() { return std::numeric_limits<unsigned int>::min(); }

unsigned int f409() { return std::numeric_limits<unsigned int>::max(); }

unsigned int f410() { return std::numeric_limits<unsigned int>::lowest(); }

unsigned int f411() { return std::numeric_limits<unsigned int>::epsilon(); }

unsigned int f412() { return std::numeric_limits<unsigned int>::round_error(); }

unsigned int f413() { return std::numeric_limits<unsigned int>::infinity(); }

unsigned int f414() { return std::numeric_limits<unsigned int>::quiet_NaN(); }

unsigned int f415() {
  return std::numeric_limits<unsigned int>::signaling_NaN();
}

unsigned int f416() { return std::numeric_limits<unsigned int>::denorm_min(); }

std::numeric_limits<unsigned int> f417() {
  return std::numeric_limits<unsigned int>();
}

std::numeric_limits<unsigned int>
f418(const std::numeric_limits<unsigned int> &a0) {
  return std::numeric_limits<unsigned int>(a0);
}

std::numeric_limits<unsigned int> f419(std::numeric_limits<unsigned int> &&a0) {
  return std::numeric_limits<unsigned int>(std::move(a0));
}

std::numeric_limits<unsigned int> &
f420(std::numeric_limits<unsigned int> &a0,
     const std::numeric_limits<unsigned int> &a1) {
  return a0.operator=(a1);
}

std::numeric_limits<unsigned int> &
f421(std::numeric_limits<unsigned int> &a0,
     std::numeric_limits<unsigned int> &&a1) {
  return a0.operator=(std::move(a1));
}

bool f422() { return std::numeric_limits<unsigned int>::is_specialized; }

int f423() { return std::numeric_limits<unsigned int>::digits; }

int f424() { return std::numeric_limits<unsigned int>::digits10; }

int f425() { return std::numeric_limits<unsigned int>::max_digits10; }

bool f426() { return std::numeric_limits<unsigned int>::is_signed; }

bool f427() { return std::numeric_limits<unsigned int>::is_integer; }

bool f428() { return std::numeric_limits<unsigned int>::is_exact; }

int f429() { return std::numeric_limits<unsigned int>::radix; }

int f430() { return std::numeric_limits<unsigned int>::min_exponent; }

int f431() { return std::numeric_limits<unsigned int>::min_exponent10; }

int f432() { return std::numeric_limits<unsigned int>::max_exponent; }

int f433() { return std::numeric_limits<unsigned int>::max_exponent10; }

bool f434() { return std::numeric_limits<unsigned int>::has_infinity; }

bool f435() { return std::numeric_limits<unsigned int>::has_quiet_NaN; }

bool f436() { return std::numeric_limits<unsigned int>::has_signaling_NaN; }

std::float_denorm_style f437() {
  return std::numeric_limits<unsigned int>::has_denorm;
}

bool f438() { return std::numeric_limits<unsigned int>::has_denorm_loss; }

bool f439() { return std::numeric_limits<unsigned int>::is_iec559; }

bool f440() { return std::numeric_limits<unsigned int>::is_bounded; }

bool f441() { return std::numeric_limits<unsigned int>::is_modulo; }

bool f442() { return std::numeric_limits<unsigned int>::traps; }

bool f443() { return std::numeric_limits<unsigned int>::tinyness_before; }

std::float_round_style f444() {
  return std::numeric_limits<unsigned int>::round_style;
}

using t17 = std::numeric_limits<unsigned __int128>;

unsigned __int128 f445() {
  return std::numeric_limits<unsigned __int128>::min();
}

unsigned __int128 f446() {
  return std::numeric_limits<unsigned __int128>::max();
}

unsigned __int128 f447() {
  return std::numeric_limits<unsigned __int128>::lowest();
}

unsigned __int128 f448() {
  return std::numeric_limits<unsigned __int128>::epsilon();
}

unsigned __int128 f449() {
  return std::numeric_limits<unsigned __int128>::round_error();
}

unsigned __int128 f450() {
  return std::numeric_limits<unsigned __int128>::infinity();
}

unsigned __int128 f451() {
  return std::numeric_limits<unsigned __int128>::quiet_NaN();
}

unsigned __int128 f452() {
  return std::numeric_limits<unsigned __int128>::signaling_NaN();
}

unsigned __int128 f453() {
  return std::numeric_limits<unsigned __int128>::denorm_min();
}

std::numeric_limits<unsigned __int128> f454() {
  return std::numeric_limits<unsigned __int128>();
}

std::numeric_limits<unsigned __int128>
f455(const std::numeric_limits<unsigned __int128> &a0) {
  return std::numeric_limits<unsigned __int128>(a0);
}

std::numeric_limits<unsigned __int128>
f456(std::numeric_limits<unsigned __int128> &&a0) {
  return std::numeric_limits<unsigned __int128>(std::move(a0));
}

std::numeric_limits<unsigned __int128> &
f457(std::numeric_limits<unsigned __int128> &a0,
     const std::numeric_limits<unsigned __int128> &a1) {
  return a0.operator=(a1);
}

std::numeric_limits<unsigned __int128> &
f458(std::numeric_limits<unsigned __int128> &a0,
     std::numeric_limits<unsigned __int128> &&a1) {
  return a0.operator=(std::move(a1));
}

bool f459() { return std::numeric_limits<unsigned __int128>::is_specialized; }

int f460() { return std::numeric_limits<unsigned __int128>::max_digits10; }

int f461() { return std::numeric_limits<unsigned __int128>::digits; }

int f462() { return std::numeric_limits<unsigned __int128>::digits10; }

bool f463() { return std::numeric_limits<unsigned __int128>::is_signed; }

bool f464() { return std::numeric_limits<unsigned __int128>::is_integer; }

bool f465() { return std::numeric_limits<unsigned __int128>::is_exact; }

int f466() { return std::numeric_limits<unsigned __int128>::radix; }

int f467() { return std::numeric_limits<unsigned __int128>::min_exponent; }

int f468() { return std::numeric_limits<unsigned __int128>::min_exponent10; }

int f469() { return std::numeric_limits<unsigned __int128>::max_exponent; }

int f470() { return std::numeric_limits<unsigned __int128>::max_exponent10; }

bool f471() { return std::numeric_limits<unsigned __int128>::has_infinity; }

bool f472() { return std::numeric_limits<unsigned __int128>::has_quiet_NaN; }

bool f473() {
  return std::numeric_limits<unsigned __int128>::has_signaling_NaN;
}

std::float_denorm_style f474() {
  return std::numeric_limits<unsigned __int128>::has_denorm;
}

bool f475() { return std::numeric_limits<unsigned __int128>::has_denorm_loss; }

bool f476() { return std::numeric_limits<unsigned __int128>::is_iec559; }

bool f477() { return std::numeric_limits<unsigned __int128>::is_bounded; }

bool f478() { return std::numeric_limits<unsigned __int128>::is_modulo; }

bool f479() { return std::numeric_limits<unsigned __int128>::traps; }

bool f480() { return std::numeric_limits<unsigned __int128>::tinyness_before; }

std::float_round_style f481() {
  return std::numeric_limits<unsigned __int128>::round_style;
}

using t18 = std::numeric_limits<unsigned long long>;

unsigned long long f482() {
  return std::numeric_limits<unsigned long long>::min();
}

unsigned long long f483() {
  return std::numeric_limits<unsigned long long>::max();
}

unsigned long long f484() {
  return std::numeric_limits<unsigned long long>::lowest();
}

unsigned long long f485() {
  return std::numeric_limits<unsigned long long>::epsilon();
}

unsigned long long f486() {
  return std::numeric_limits<unsigned long long>::round_error();
}

unsigned long long f487() {
  return std::numeric_limits<unsigned long long>::infinity();
}

unsigned long long f488() {
  return std::numeric_limits<unsigned long long>::quiet_NaN();
}

unsigned long long f489() {
  return std::numeric_limits<unsigned long long>::signaling_NaN();
}

unsigned long long f490() {
  return std::numeric_limits<unsigned long long>::denorm_min();
}

std::numeric_limits<unsigned long long> f491() {
  return std::numeric_limits<unsigned long long>();
}

std::numeric_limits<unsigned long long>
f492(const std::numeric_limits<unsigned long long> &a0) {
  return std::numeric_limits<unsigned long long>(a0);
}

std::numeric_limits<unsigned long long>
f493(std::numeric_limits<unsigned long long> &&a0) {
  return std::numeric_limits<unsigned long long>(std::move(a0));
}

std::numeric_limits<unsigned long long> &
f494(std::numeric_limits<unsigned long long> &a0,
     const std::numeric_limits<unsigned long long> &a1) {
  return a0.operator=(a1);
}

std::numeric_limits<unsigned long long> &
f495(std::numeric_limits<unsigned long long> &a0,
     std::numeric_limits<unsigned long long> &&a1) {
  return a0.operator=(std::move(a1));
}

bool f496() { return std::numeric_limits<unsigned long long>::is_specialized; }

int f497() { return std::numeric_limits<unsigned long long>::digits; }

int f498() { return std::numeric_limits<unsigned long long>::digits10; }

int f499() { return std::numeric_limits<unsigned long long>::max_digits10; }

bool f500() { return std::numeric_limits<unsigned long long>::is_signed; }

bool f501() { return std::numeric_limits<unsigned long long>::is_integer; }

bool f502() { return std::numeric_limits<unsigned long long>::is_exact; }

int f503() { return std::numeric_limits<unsigned long long>::radix; }

int f504() { return std::numeric_limits<unsigned long long>::min_exponent; }

int f505() { return std::numeric_limits<unsigned long long>::min_exponent10; }

int f506() { return std::numeric_limits<unsigned long long>::max_exponent; }

int f507() { return std::numeric_limits<unsigned long long>::max_exponent10; }

bool f508() { return std::numeric_limits<unsigned long long>::has_infinity; }

bool f509() { return std::numeric_limits<unsigned long long>::has_quiet_NaN; }

bool f510() {
  return std::numeric_limits<unsigned long long>::has_signaling_NaN;
}

std::float_denorm_style f511() {
  return std::numeric_limits<unsigned long long>::has_denorm;
}

bool f512() { return std::numeric_limits<unsigned long long>::has_denorm_loss; }

bool f513() { return std::numeric_limits<unsigned long long>::is_iec559; }

bool f514() { return std::numeric_limits<unsigned long long>::is_bounded; }

bool f515() { return std::numeric_limits<unsigned long long>::is_modulo; }

bool f516() { return std::numeric_limits<unsigned long long>::traps; }

bool f517() { return std::numeric_limits<unsigned long long>::tinyness_before; }

std::float_round_style f518() {
  return std::numeric_limits<unsigned long long>::round_style;
}

using t19 = std::numeric_limits<unsigned long>;

unsigned long f519() { return std::numeric_limits<unsigned long>::min(); }

unsigned long f520() { return std::numeric_limits<unsigned long>::max(); }

unsigned long f521() { return std::numeric_limits<unsigned long>::lowest(); }

unsigned long f522() { return std::numeric_limits<unsigned long>::epsilon(); }

unsigned long f523() {
  return std::numeric_limits<unsigned long>::round_error();
}

unsigned long f524() { return std::numeric_limits<unsigned long>::infinity(); }

unsigned long f525() { return std::numeric_limits<unsigned long>::quiet_NaN(); }

unsigned long f526() {
  return std::numeric_limits<unsigned long>::signaling_NaN();
}

unsigned long f527() {
  return std::numeric_limits<unsigned long>::denorm_min();
}

std::numeric_limits<unsigned long> f528() {
  return std::numeric_limits<unsigned long>();
}

std::numeric_limits<unsigned long>
f529(const std::numeric_limits<unsigned long> &a0) {
  return std::numeric_limits<unsigned long>(a0);
}

std::numeric_limits<unsigned long>
f530(std::numeric_limits<unsigned long> &&a0) {
  return std::numeric_limits<unsigned long>(std::move(a0));
}

std::numeric_limits<unsigned long> &
f531(std::numeric_limits<unsigned long> &a0,
     const std::numeric_limits<unsigned long> &a1) {
  return a0.operator=(a1);
}

std::numeric_limits<unsigned long> &
f532(std::numeric_limits<unsigned long> &a0,
     std::numeric_limits<unsigned long> &&a1) {
  return a0.operator=(std::move(a1));
}

bool f533() { return std::numeric_limits<unsigned long>::is_specialized; }

int f534() { return std::numeric_limits<unsigned long>::digits; }

int f535() { return std::numeric_limits<unsigned long>::digits10; }

int f536() { return std::numeric_limits<unsigned long>::max_digits10; }

bool f537() { return std::numeric_limits<unsigned long>::is_signed; }

bool f538() { return std::numeric_limits<unsigned long>::is_integer; }

bool f539() { return std::numeric_limits<unsigned long>::is_exact; }

int f540() { return std::numeric_limits<unsigned long>::radix; }

int f541() { return std::numeric_limits<unsigned long>::min_exponent; }

int f542() { return std::numeric_limits<unsigned long>::min_exponent10; }

int f543() { return std::numeric_limits<unsigned long>::max_exponent; }

int f544() { return std::numeric_limits<unsigned long>::max_exponent10; }

bool f545() { return std::numeric_limits<unsigned long>::has_infinity; }

bool f546() { return std::numeric_limits<unsigned long>::has_quiet_NaN; }

bool f547() { return std::numeric_limits<unsigned long>::has_signaling_NaN; }

std::float_denorm_style f548() {
  return std::numeric_limits<unsigned long>::has_denorm;
}

bool f549() { return std::numeric_limits<unsigned long>::has_denorm_loss; }

bool f550() { return std::numeric_limits<unsigned long>::is_iec559; }

bool f551() { return std::numeric_limits<unsigned long>::is_bounded; }

bool f552() { return std::numeric_limits<unsigned long>::is_modulo; }

bool f553() { return std::numeric_limits<unsigned long>::traps; }

bool f554() { return std::numeric_limits<unsigned long>::tinyness_before; }

std::float_round_style f555() {
  return std::numeric_limits<unsigned long>::round_style;
}

using t20 = std::numeric_limits<char16_t>;

char16_t f556() { return std::numeric_limits<char16_t>::min(); }

char16_t f557() { return std::numeric_limits<char16_t>::max(); }

char16_t f558() { return std::numeric_limits<char16_t>::lowest(); }

char16_t f559() { return std::numeric_limits<char16_t>::epsilon(); }

char16_t f560() { return std::numeric_limits<char16_t>::round_error(); }

char16_t f561() { return std::numeric_limits<char16_t>::infinity(); }

char16_t f562() { return std::numeric_limits<char16_t>::quiet_NaN(); }

char16_t f563() { return std::numeric_limits<char16_t>::signaling_NaN(); }

char16_t f564() { return std::numeric_limits<char16_t>::denorm_min(); }

std::numeric_limits<char16_t> f565() { return std::numeric_limits<char16_t>(); }

std::numeric_limits<char16_t> f566(const std::numeric_limits<char16_t> &a0) {
  return std::numeric_limits<char16_t>(a0);
}

std::numeric_limits<char16_t> f567(std::numeric_limits<char16_t> &&a0) {
  return std::numeric_limits<char16_t>(std::move(a0));
}

std::numeric_limits<char16_t> &f568(std::numeric_limits<char16_t> &a0,
                                    const std::numeric_limits<char16_t> &a1) {
  return a0.operator=(a1);
}

std::numeric_limits<char16_t> &f569(std::numeric_limits<char16_t> &a0,
                                    std::numeric_limits<char16_t> &&a1) {
  return a0.operator=(std::move(a1));
}

bool f570() { return std::numeric_limits<char16_t>::is_specialized; }

int f571() { return std::numeric_limits<char16_t>::digits; }

int f572() { return std::numeric_limits<char16_t>::digits10; }

int f573() { return std::numeric_limits<char16_t>::max_digits10; }

bool f574() { return std::numeric_limits<char16_t>::is_signed; }

bool f575() { return std::numeric_limits<char16_t>::is_integer; }

bool f576() { return std::numeric_limits<char16_t>::is_exact; }

int f577() { return std::numeric_limits<char16_t>::radix; }

int f578() { return std::numeric_limits<char16_t>::min_exponent; }

int f579() { return std::numeric_limits<char16_t>::min_exponent10; }

int f580() { return std::numeric_limits<char16_t>::max_exponent; }

int f581() { return std::numeric_limits<char16_t>::max_exponent10; }

bool f582() { return std::numeric_limits<char16_t>::has_infinity; }

bool f583() { return std::numeric_limits<char16_t>::has_quiet_NaN; }

bool f584() { return std::numeric_limits<char16_t>::has_signaling_NaN; }

std::float_denorm_style f585() {
  return std::numeric_limits<char16_t>::has_denorm;
}

bool f586() { return std::numeric_limits<char16_t>::has_denorm_loss; }

bool f587() { return std::numeric_limits<char16_t>::is_iec559; }

bool f588() { return std::numeric_limits<char16_t>::is_bounded; }

bool f589() { return std::numeric_limits<char16_t>::is_modulo; }

bool f590() { return std::numeric_limits<char16_t>::traps; }

bool f591() { return std::numeric_limits<char16_t>::tinyness_before; }

std::float_round_style f592() {
  return std::numeric_limits<char16_t>::round_style;
}

using t21 = std::numeric_limits<signed char>;

signed char f593() { return std::numeric_limits<signed char>::min(); }

signed char f594() { return std::numeric_limits<signed char>::max(); }

signed char f595() { return std::numeric_limits<signed char>::lowest(); }

signed char f596() { return std::numeric_limits<signed char>::epsilon(); }

signed char f597() { return std::numeric_limits<signed char>::round_error(); }

signed char f598() { return std::numeric_limits<signed char>::infinity(); }

signed char f599() { return std::numeric_limits<signed char>::quiet_NaN(); }

signed char f600() { return std::numeric_limits<signed char>::signaling_NaN(); }

signed char f601() { return std::numeric_limits<signed char>::denorm_min(); }

std::numeric_limits<signed char> f602() {
  return std::numeric_limits<signed char>();
}

std::numeric_limits<signed char>
f603(const std::numeric_limits<signed char> &a0) {
  return std::numeric_limits<signed char>(a0);
}

std::numeric_limits<signed char> f604(std::numeric_limits<signed char> &&a0) {
  return std::numeric_limits<signed char>(std::move(a0));
}

std::numeric_limits<signed char> &
f605(std::numeric_limits<signed char> &a0,
     const std::numeric_limits<signed char> &a1) {
  return a0.operator=(a1);
}

std::numeric_limits<signed char> &f606(std::numeric_limits<signed char> &a0,
                                       std::numeric_limits<signed char> &&a1) {
  return a0.operator=(std::move(a1));
}

bool f607() { return std::numeric_limits<signed char>::is_specialized; }

int f608() { return std::numeric_limits<signed char>::digits; }

int f609() { return std::numeric_limits<signed char>::digits10; }

int f610() { return std::numeric_limits<signed char>::max_digits10; }

bool f611() { return std::numeric_limits<signed char>::is_signed; }

bool f612() { return std::numeric_limits<signed char>::is_integer; }

bool f613() { return std::numeric_limits<signed char>::is_exact; }

int f614() { return std::numeric_limits<signed char>::radix; }

int f615() { return std::numeric_limits<signed char>::min_exponent; }

int f616() { return std::numeric_limits<signed char>::min_exponent10; }

int f617() { return std::numeric_limits<signed char>::max_exponent; }

int f618() { return std::numeric_limits<signed char>::max_exponent10; }

bool f619() { return std::numeric_limits<signed char>::has_infinity; }

bool f620() { return std::numeric_limits<signed char>::has_quiet_NaN; }

bool f621() { return std::numeric_limits<signed char>::has_signaling_NaN; }

std::float_denorm_style f622() {
  return std::numeric_limits<signed char>::has_denorm;
}

bool f623() { return std::numeric_limits<signed char>::has_denorm_loss; }

bool f624() { return std::numeric_limits<signed char>::is_iec559; }

bool f625() { return std::numeric_limits<signed char>::is_bounded; }

bool f626() { return std::numeric_limits<signed char>::is_modulo; }

bool f627() { return std::numeric_limits<signed char>::traps; }

bool f628() { return std::numeric_limits<signed char>::tinyness_before; }

std::float_round_style f629() {
  return std::numeric_limits<signed char>::round_style;
}

using t22 = std::numeric_limits<unsigned short>;

unsigned short f630() { return std::numeric_limits<unsigned short>::min(); }

unsigned short f631() { return std::numeric_limits<unsigned short>::max(); }

unsigned short f632() { return std::numeric_limits<unsigned short>::lowest(); }

unsigned short f633() { return std::numeric_limits<unsigned short>::epsilon(); }

unsigned short f634() {
  return std::numeric_limits<unsigned short>::round_error();
}

unsigned short f635() {
  return std::numeric_limits<unsigned short>::infinity();
}

unsigned short f636() {
  return std::numeric_limits<unsigned short>::quiet_NaN();
}

unsigned short f637() {
  return std::numeric_limits<unsigned short>::signaling_NaN();
}

unsigned short f638() {
  return std::numeric_limits<unsigned short>::denorm_min();
}

std::numeric_limits<unsigned short> f639() {
  return std::numeric_limits<unsigned short>();
}

std::numeric_limits<unsigned short>
f640(const std::numeric_limits<unsigned short> &a0) {
  return std::numeric_limits<unsigned short>(a0);
}

std::numeric_limits<unsigned short>
f641(std::numeric_limits<unsigned short> &&a0) {
  return std::numeric_limits<unsigned short>(std::move(a0));
}

std::numeric_limits<unsigned short> &
f642(std::numeric_limits<unsigned short> &a0,
     const std::numeric_limits<unsigned short> &a1) {
  return a0.operator=(a1);
}

std::numeric_limits<unsigned short> &
f643(std::numeric_limits<unsigned short> &a0,
     std::numeric_limits<unsigned short> &&a1) {
  return a0.operator=(std::move(a1));
}

bool f644() { return std::numeric_limits<unsigned short>::is_specialized; }

int f645() { return std::numeric_limits<unsigned short>::digits; }

int f646() { return std::numeric_limits<unsigned short>::digits10; }

int f647() { return std::numeric_limits<unsigned short>::max_digits10; }

bool f648() { return std::numeric_limits<unsigned short>::is_signed; }

bool f649() { return std::numeric_limits<unsigned short>::is_integer; }

bool f650() { return std::numeric_limits<unsigned short>::is_exact; }

int f651() { return std::numeric_limits<unsigned short>::radix; }

int f652() { return std::numeric_limits<unsigned short>::min_exponent; }

int f653() { return std::numeric_limits<unsigned short>::min_exponent10; }

int f654() { return std::numeric_limits<unsigned short>::max_exponent; }

int f655() { return std::numeric_limits<unsigned short>::max_exponent10; }

bool f656() { return std::numeric_limits<unsigned short>::has_infinity; }

bool f657() { return std::numeric_limits<unsigned short>::has_quiet_NaN; }

bool f658() { return std::numeric_limits<unsigned short>::has_signaling_NaN; }

std::float_denorm_style f659() {
  return std::numeric_limits<unsigned short>::has_denorm;
}

bool f660() { return std::numeric_limits<unsigned short>::has_denorm_loss; }

bool f661() { return std::numeric_limits<unsigned short>::is_iec559; }

bool f662() { return std::numeric_limits<unsigned short>::is_bounded; }

bool f663() { return std::numeric_limits<unsigned short>::is_modulo; }

bool f664() { return std::numeric_limits<unsigned short>::traps; }

bool f665() { return std::numeric_limits<unsigned short>::tinyness_before; }

std::float_round_style f666() {
  return std::numeric_limits<unsigned short>::round_style;
}

using t23 = std::numeric_limits<char32_t>;

char32_t f667() { return std::numeric_limits<char32_t>::min(); }

char32_t f668() { return std::numeric_limits<char32_t>::max(); }

char32_t f669() { return std::numeric_limits<char32_t>::lowest(); }

char32_t f670() { return std::numeric_limits<char32_t>::epsilon(); }

char32_t f671() { return std::numeric_limits<char32_t>::round_error(); }

char32_t f672() { return std::numeric_limits<char32_t>::infinity(); }

char32_t f673() { return std::numeric_limits<char32_t>::quiet_NaN(); }

char32_t f674() { return std::numeric_limits<char32_t>::signaling_NaN(); }

char32_t f675() { return std::numeric_limits<char32_t>::denorm_min(); }

std::numeric_limits<char32_t> f676() { return std::numeric_limits<char32_t>(); }

std::numeric_limits<char32_t> f677(const std::numeric_limits<char32_t> &a0) {
  return std::numeric_limits<char32_t>(a0);
}

std::numeric_limits<char32_t> f678(std::numeric_limits<char32_t> &&a0) {
  return std::numeric_limits<char32_t>(std::move(a0));
}

std::numeric_limits<char32_t> &f679(std::numeric_limits<char32_t> &a0,
                                    const std::numeric_limits<char32_t> &a1) {
  return a0.operator=(a1);
}

std::numeric_limits<char32_t> &f680(std::numeric_limits<char32_t> &a0,
                                    std::numeric_limits<char32_t> &&a1) {
  return a0.operator=(std::move(a1));
}

bool f681() { return std::numeric_limits<char32_t>::is_specialized; }

int f682() { return std::numeric_limits<char32_t>::digits; }

int f683() { return std::numeric_limits<char32_t>::digits10; }

int f684() { return std::numeric_limits<char32_t>::max_digits10; }

bool f685() { return std::numeric_limits<char32_t>::is_signed; }

bool f686() { return std::numeric_limits<char32_t>::is_integer; }

bool f687() { return std::numeric_limits<char32_t>::is_exact; }

int f688() { return std::numeric_limits<char32_t>::radix; }

int f689() { return std::numeric_limits<char32_t>::min_exponent; }

int f690() { return std::numeric_limits<char32_t>::min_exponent10; }

int f691() { return std::numeric_limits<char32_t>::max_exponent; }

int f692() { return std::numeric_limits<char32_t>::max_exponent10; }

bool f693() { return std::numeric_limits<char32_t>::has_infinity; }

bool f694() { return std::numeric_limits<char32_t>::has_quiet_NaN; }

bool f695() { return std::numeric_limits<char32_t>::has_signaling_NaN; }

std::float_denorm_style f696() {
  return std::numeric_limits<char32_t>::has_denorm;
}

bool f697() { return std::numeric_limits<char32_t>::has_denorm_loss; }

bool f698() { return std::numeric_limits<char32_t>::is_iec559; }

bool f699() { return std::numeric_limits<char32_t>::is_bounded; }

bool f700() { return std::numeric_limits<char32_t>::is_modulo; }

bool f701() { return std::numeric_limits<char32_t>::traps; }

bool f702() { return std::numeric_limits<char32_t>::tinyness_before; }

std::float_round_style f703() {
  return std::numeric_limits<char32_t>::round_style;
}

template <typename T1> T1 f704() {
  return std::numeric_limits<const T1>::min();
}

template <typename T1> T1 f705() {
  return std::numeric_limits<const T1>::max();
}

template <typename T1> T1 f706() {
  return std::numeric_limits<const T1>::lowest();
}

template <typename T1> T1 f707() {
  return std::numeric_limits<const T1>::epsilon();
}

template <typename T1> T1 f708() {
  return std::numeric_limits<const T1>::round_error();
}

template <typename T1> T1 f709() {
  return std::numeric_limits<const T1>::infinity();
}

template <typename T1> T1 f710() {
  return std::numeric_limits<const T1>::quiet_NaN();
}

template <typename T1> T1 f711() {
  return std::numeric_limits<const T1>::signaling_NaN();
}

template <typename T1> T1 f712() {
  return std::numeric_limits<const T1>::denorm_min();
}

template <typename T1> T1 f713() {
  return std::numeric_limits<volatile T1>::min();
}

template <typename T1> T1 f714() {
  return std::numeric_limits<volatile T1>::max();
}

template <typename T1> T1 f715() {
  return std::numeric_limits<volatile T1>::lowest();
}

template <typename T1> T1 f716() {
  return std::numeric_limits<volatile T1>::epsilon();
}

template <typename T1> T1 f717() {
  return std::numeric_limits<volatile T1>::round_error();
}

template <typename T1> T1 f718() {
  return std::numeric_limits<volatile T1>::infinity();
}

template <typename T1> T1 f719() {
  return std::numeric_limits<volatile T1>::quiet_NaN();
}

template <typename T1> T1 f720() {
  return std::numeric_limits<volatile T1>::signaling_NaN();
}

template <typename T1> T1 f721() {
  return std::numeric_limits<volatile T1>::denorm_min();
}

template <typename T1> T1 f722() {
  return std::numeric_limits<const volatile T1>::min();
}

template <typename T1> T1 f723() {
  return std::numeric_limits<const volatile T1>::max();
}

template <typename T1> T1 f724() {
  return std::numeric_limits<const volatile T1>::lowest();
}

template <typename T1> T1 f725() {
  return std::numeric_limits<const volatile T1>::epsilon();
}

template <typename T1> T1 f726() {
  return std::numeric_limits<const volatile T1>::round_error();
}

template <typename T1> T1 f727() {
  return std::numeric_limits<const volatile T1>::infinity();
}

template <typename T1> T1 f728() {
  return std::numeric_limits<const volatile T1>::quiet_NaN();
}

template <typename T1> T1 f729() {
  return std::numeric_limits<const volatile T1>::signaling_NaN();
}

template <typename T1> T1 f730() {
  return std::numeric_limits<const volatile T1>::denorm_min();
}

template <typename T1> T1 f731() { return std::numeric_limits<T1>::min(); }

template <typename T1> T1 f732() { return std::numeric_limits<T1>::max(); }

template <typename T1> T1 f733() { return std::numeric_limits<T1>::lowest(); }

template <typename T1> T1 f734() { return std::numeric_limits<T1>::epsilon(); }

template <typename T1> T1 f735() {
  return std::numeric_limits<T1>::round_error();
}

template <typename T1> T1 f736() { return std::numeric_limits<T1>::infinity(); }

template <typename T1> T1 f737() {
  return std::numeric_limits<T1>::quiet_NaN();
}

template <typename T1> T1 f738() {
  return std::numeric_limits<T1>::signaling_NaN();
}

template <typename T1> T1 f739() {
  return std::numeric_limits<T1>::denorm_min();
}
