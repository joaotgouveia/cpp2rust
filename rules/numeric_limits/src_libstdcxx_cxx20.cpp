#include <limits>
#include <utility>

using t24 = std::numeric_limits<char8_t>;

char8_t f740() { return std::numeric_limits<char8_t>::min(); }

char8_t f741() { return std::numeric_limits<char8_t>::max(); }

char8_t f742() { return std::numeric_limits<char8_t>::lowest(); }

char8_t f743() { return std::numeric_limits<char8_t>::epsilon(); }

char8_t f744() { return std::numeric_limits<char8_t>::round_error(); }

char8_t f745() { return std::numeric_limits<char8_t>::infinity(); }

char8_t f746() { return std::numeric_limits<char8_t>::quiet_NaN(); }

char8_t f747() { return std::numeric_limits<char8_t>::signaling_NaN(); }

char8_t f748() { return std::numeric_limits<char8_t>::denorm_min(); }

std::numeric_limits<char8_t> f749() { return std::numeric_limits<char8_t>(); }

std::numeric_limits<char8_t> f750(const std::numeric_limits<char8_t> &a0) {
  return std::numeric_limits<char8_t>(a0);
}

std::numeric_limits<char8_t> f751(std::numeric_limits<char8_t> &&a0) {
  return std::numeric_limits<char8_t>(std::move(a0));
}

std::numeric_limits<char8_t> &f752(std::numeric_limits<char8_t> &a0,
                                   const std::numeric_limits<char8_t> &a1) {
  return a0.operator=(a1);
}

std::numeric_limits<char8_t> &f753(std::numeric_limits<char8_t> &a0,
                                   std::numeric_limits<char8_t> &&a1) {
  return a0.operator=(std::move(a1));
}

bool f754() { return std::numeric_limits<char8_t>::is_specialized; }

int f755() { return std::numeric_limits<char8_t>::digits; }

int f756() { return std::numeric_limits<char8_t>::digits10; }

int f757() { return std::numeric_limits<char8_t>::max_digits10; }

bool f758() { return std::numeric_limits<char8_t>::is_signed; }

bool f759() { return std::numeric_limits<char8_t>::is_integer; }

bool f760() { return std::numeric_limits<char8_t>::is_exact; }

int f761() { return std::numeric_limits<char8_t>::radix; }

int f762() { return std::numeric_limits<char8_t>::min_exponent; }

int f763() { return std::numeric_limits<char8_t>::min_exponent10; }

int f764() { return std::numeric_limits<char8_t>::max_exponent; }

int f765() { return std::numeric_limits<char8_t>::max_exponent10; }

bool f766() { return std::numeric_limits<char8_t>::has_infinity; }

bool f767() { return std::numeric_limits<char8_t>::has_quiet_NaN; }

bool f768() { return std::numeric_limits<char8_t>::has_signaling_NaN; }

std::float_denorm_style f769() {
  return std::numeric_limits<char8_t>::has_denorm;
}

bool f770() { return std::numeric_limits<char8_t>::has_denorm_loss; }

bool f771() { return std::numeric_limits<char8_t>::is_iec559; }

bool f772() { return std::numeric_limits<char8_t>::is_bounded; }

bool f773() { return std::numeric_limits<char8_t>::is_modulo; }

bool f774() { return std::numeric_limits<char8_t>::traps; }

bool f775() { return std::numeric_limits<char8_t>::tinyness_before; }

std::float_round_style f776() {
  return std::numeric_limits<char8_t>::round_style;
}
