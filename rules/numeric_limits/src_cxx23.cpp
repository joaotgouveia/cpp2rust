#include <limits>
#include <utility>

using t1 = std::numeric_limits<char>;

char f1() { return std::numeric_limits<char>::min(); }

char f2() { return std::numeric_limits<char>::max(); }

char f3() { return std::numeric_limits<char>::lowest(); }

long double f38() { return std::numeric_limits<long double>::min(); }

long double f39() { return std::numeric_limits<long double>::max(); }

long double f40() { return std::numeric_limits<long double>::lowest(); }

using t3 = std::numeric_limits<int>;

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

__int128 f112() { return std::numeric_limits<__int128>::min(); }

__int128 f113() { return std::numeric_limits<__int128>::max(); }

__int128 f116() { return std::numeric_limits<__int128>::lowest(); }

using t5 = std::numeric_limits<long long>;

long long f149() { return std::numeric_limits<long long>::min(); }

long long f150() { return std::numeric_limits<long long>::max(); }

long long f151() { return std::numeric_limits<long long>::lowest(); }

using t6 = std::numeric_limits<long>;

long f186() { return std::numeric_limits<long>::min(); }

long f187() { return std::numeric_limits<long>::max(); }

long f188() { return std::numeric_limits<long>::lowest(); }

using t8 = std::numeric_limits<short>;

short f260() { return std::numeric_limits<short>::min(); }

short f261() { return std::numeric_limits<short>::max(); }

short f262() { return std::numeric_limits<short>::lowest(); }

using t9 = std::numeric_limits<wchar_t>;

wchar_t f297() { return std::numeric_limits<wchar_t>::min(); }

wchar_t f298() { return std::numeric_limits<wchar_t>::max(); }

wchar_t f299() { return std::numeric_limits<wchar_t>::lowest(); }

bool f334() { return std::numeric_limits<bool>::min(); }

bool f335() { return std::numeric_limits<bool>::max(); }

bool f336() { return std::numeric_limits<bool>::lowest(); }

using t11 = std::numeric_limits<unsigned char>;

unsigned char f371() { return std::numeric_limits<unsigned char>::min(); }

unsigned char f372() { return std::numeric_limits<unsigned char>::max(); }

unsigned char f373() { return std::numeric_limits<unsigned char>::lowest(); }

using t12 = std::numeric_limits<double>;

double f408() { return std::numeric_limits<double>::min(); }

double f409() { return std::numeric_limits<double>::max(); }

double f410() { return std::numeric_limits<double>::lowest(); }

double f411() { return std::numeric_limits<double>::epsilon(); }

double f412() { return std::numeric_limits<double>::round_error(); }

double f413() { return std::numeric_limits<double>::infinity(); }

double f414() { return std::numeric_limits<double>::quiet_NaN(); }

double f415() { return std::numeric_limits<double>::signaling_NaN(); }

double f416() { return std::numeric_limits<double>::denorm_min(); }

std::numeric_limits<double> f417() { return std::numeric_limits<double>(); }

std::numeric_limits<double> f418(const std::numeric_limits<double> &a0) {
  return std::numeric_limits<double>(a0);
}

std::numeric_limits<double> f419(std::numeric_limits<double> &&a0) {
  return std::numeric_limits<double>(std::move(a0));
}

std::numeric_limits<double> &f420(std::numeric_limits<double> &a0,
                                  const std::numeric_limits<double> &a1) {
  return a0.operator=(a1);
}

std::numeric_limits<double> &f421(std::numeric_limits<double> &a0,
                                  std::numeric_limits<double> &&a1) {
  return a0.operator=(std::move(a1));
}

bool f422() { return std::numeric_limits<double>::is_specialized; }

int f423() { return std::numeric_limits<double>::digits; }

int f424() { return std::numeric_limits<double>::digits10; }

int f425() { return std::numeric_limits<double>::max_digits10; }

bool f426() { return std::numeric_limits<double>::is_signed; }

bool f427() { return std::numeric_limits<double>::is_integer; }

bool f428() { return std::numeric_limits<double>::is_exact; }

int f429() { return std::numeric_limits<double>::radix; }

int f430() { return std::numeric_limits<double>::min_exponent; }

int f431() { return std::numeric_limits<double>::min_exponent10; }

int f432() { return std::numeric_limits<double>::max_exponent; }

int f433() { return std::numeric_limits<double>::max_exponent10; }

bool f434() { return std::numeric_limits<double>::has_infinity; }

bool f435() { return std::numeric_limits<double>::has_quiet_NaN; }

bool f436() { return std::numeric_limits<double>::has_signaling_NaN; }

std::float_denorm_style f437() {
  return std::numeric_limits<double>::has_denorm;
}

bool f438() { return std::numeric_limits<double>::has_denorm_loss; }

bool f439() { return std::numeric_limits<double>::is_iec559; }

bool f440() { return std::numeric_limits<double>::is_bounded; }

bool f441() { return std::numeric_limits<double>::is_modulo; }

bool f442() { return std::numeric_limits<double>::traps; }

bool f443() { return std::numeric_limits<double>::tinyness_before; }

std::float_round_style f444() {
  return std::numeric_limits<double>::round_style;
}

float f445() { return std::numeric_limits<float>::min(); }

float f446() { return std::numeric_limits<float>::max(); }

float f447() { return std::numeric_limits<float>::lowest(); }

using t14 = std::numeric_limits<unsigned int>;

unsigned int f482() { return std::numeric_limits<unsigned int>::min(); }

unsigned int f483() { return std::numeric_limits<unsigned int>::max(); }

unsigned int f484() { return std::numeric_limits<unsigned int>::lowest(); }

using t16 = std::numeric_limits<unsigned long long>;

unsigned long long f556() {
  return std::numeric_limits<unsigned long long>::min();
}

unsigned long long f557() {
  return std::numeric_limits<unsigned long long>::max();
}

unsigned long long f558() {
  return std::numeric_limits<unsigned long long>::lowest();
}

using t17 = std::numeric_limits<unsigned long>;

unsigned long f593() { return std::numeric_limits<unsigned long>::min(); }

unsigned long f594() { return std::numeric_limits<unsigned long>::max(); }

unsigned long f595() { return std::numeric_limits<unsigned long>::lowest(); }

using t18 = std::numeric_limits<char16_t>;

char16_t f630() { return std::numeric_limits<char16_t>::min(); }

char16_t f631() { return std::numeric_limits<char16_t>::max(); }

char16_t f632() { return std::numeric_limits<char16_t>::lowest(); }

using t19 = std::numeric_limits<signed char>;

signed char f667() { return std::numeric_limits<signed char>::min(); }

signed char f668() { return std::numeric_limits<signed char>::max(); }

signed char f669() { return std::numeric_limits<signed char>::lowest(); }

using t20 = std::numeric_limits<unsigned short>;

unsigned short f704() { return std::numeric_limits<unsigned short>::min(); }

unsigned short f705() { return std::numeric_limits<unsigned short>::max(); }

unsigned short f706() { return std::numeric_limits<unsigned short>::lowest(); }

using t21 = std::numeric_limits<char8_t>;

char8_t f741() { return std::numeric_limits<char8_t>::min(); }

char8_t f742() { return std::numeric_limits<char8_t>::max(); }

char8_t f743() { return std::numeric_limits<char8_t>::lowest(); }

using t22 = std::numeric_limits<char32_t>;

char32_t f778() { return std::numeric_limits<char32_t>::min(); }

char32_t f779() { return std::numeric_limits<char32_t>::max(); }

char32_t f780() { return std::numeric_limits<char32_t>::lowest(); }
