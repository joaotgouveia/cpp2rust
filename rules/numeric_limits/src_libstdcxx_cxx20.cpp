#include <limits>
#include <utility>

using t26 = std::numeric_limits<char8_t>;

char8_t f814() { return std::numeric_limits<char8_t>::min(); }

char8_t f815() { return std::numeric_limits<char8_t>::max(); }

char8_t f816() { return std::numeric_limits<char8_t>::lowest(); }

char8_t f817() { return std::numeric_limits<char8_t>::epsilon(); }

char8_t f818() { return std::numeric_limits<char8_t>::round_error(); }

char8_t f819() { return std::numeric_limits<char8_t>::infinity(); }

char8_t f820() { return std::numeric_limits<char8_t>::quiet_NaN(); }

char8_t f821() { return std::numeric_limits<char8_t>::signaling_NaN(); }

char8_t f822() { return std::numeric_limits<char8_t>::denorm_min(); }

std::numeric_limits<char8_t> f823() { return std::numeric_limits<char8_t>(); }

std::numeric_limits<char8_t> f824(const std::numeric_limits<char8_t> &a0) {
  return std::numeric_limits<char8_t>(a0);
}

std::numeric_limits<char8_t> f825(std::numeric_limits<char8_t> &&a0) {
  return std::numeric_limits<char8_t>(std::move(a0));
}

std::numeric_limits<char8_t> &f826(std::numeric_limits<char8_t> &a0,
                                   const std::numeric_limits<char8_t> &a1) {
  return a0.operator=(a1);
}

std::numeric_limits<char8_t> &f827(std::numeric_limits<char8_t> &a0,
                                   std::numeric_limits<char8_t> &&a1) {
  return a0.operator=(std::move(a1));
}

bool f828() { return std::numeric_limits<char8_t>::is_specialized; }

int f829() { return std::numeric_limits<char8_t>::digits; }

int f830() { return std::numeric_limits<char8_t>::digits10; }

int f831() { return std::numeric_limits<char8_t>::max_digits10; }

bool f832() { return std::numeric_limits<char8_t>::is_signed; }

bool f833() { return std::numeric_limits<char8_t>::is_integer; }

bool f834() { return std::numeric_limits<char8_t>::is_exact; }

int f835() { return std::numeric_limits<char8_t>::radix; }

int f836() { return std::numeric_limits<char8_t>::min_exponent; }

int f837() { return std::numeric_limits<char8_t>::min_exponent10; }

int f838() { return std::numeric_limits<char8_t>::max_exponent; }

int f839() { return std::numeric_limits<char8_t>::max_exponent10; }

bool f840() { return std::numeric_limits<char8_t>::has_infinity; }

bool f841() { return std::numeric_limits<char8_t>::has_quiet_NaN; }

bool f842() { return std::numeric_limits<char8_t>::has_signaling_NaN; }

std::float_denorm_style f843() {
  return std::numeric_limits<char8_t>::has_denorm;
}

bool f844() { return std::numeric_limits<char8_t>::has_denorm_loss; }

bool f845() { return std::numeric_limits<char8_t>::is_iec559; }

bool f846() { return std::numeric_limits<char8_t>::is_bounded; }

bool f847() { return std::numeric_limits<char8_t>::is_modulo; }

bool f848() { return std::numeric_limits<char8_t>::traps; }

bool f849() { return std::numeric_limits<char8_t>::tinyness_before; }

std::float_round_style f850() {
  return std::numeric_limits<char8_t>::round_style;
}
