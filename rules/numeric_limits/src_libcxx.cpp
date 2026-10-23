#include <limits>
#include <utility>

template <typename T1> bool f777() {
  return std::numeric_limits<T1>::is_specialized;
}

template <typename T1> int f778() { return std::numeric_limits<T1>::digits; }

template <typename T1> int f779() { return std::numeric_limits<T1>::digits10; }

template <typename T1> int f780() {
  return std::numeric_limits<T1>::max_digits10;
}

template <typename T1> bool f781() {
  return std::numeric_limits<T1>::is_signed;
}

template <typename T1> bool f782() {
  return std::numeric_limits<T1>::is_integer;
}

template <typename T1> bool f783() { return std::numeric_limits<T1>::is_exact; }

template <typename T1> int f784() { return std::numeric_limits<T1>::radix; }

template <typename T1> int f785() {
  return std::numeric_limits<T1>::min_exponent;
}

template <typename T1> int f786() {
  return std::numeric_limits<T1>::min_exponent10;
}

template <typename T1> int f787() {
  return std::numeric_limits<T1>::max_exponent;
}

template <typename T1> int f788() {
  return std::numeric_limits<T1>::max_exponent10;
}

template <typename T1> bool f789() {
  return std::numeric_limits<T1>::has_infinity;
}

template <typename T1> bool f790() {
  return std::numeric_limits<T1>::has_quiet_NaN;
}

template <typename T1> bool f791() {
  return std::numeric_limits<T1>::has_signaling_NaN;
}

template <typename T1> std::float_denorm_style f792() {
  return std::numeric_limits<T1>::has_denorm;
}

template <typename T1> bool f793() {
  return std::numeric_limits<T1>::has_denorm_loss;
}

template <typename T1> bool f794() {
  return std::numeric_limits<T1>::is_iec559;
}

template <typename T1> bool f795() {
  return std::numeric_limits<T1>::is_bounded;
}

template <typename T1> bool f796() {
  return std::numeric_limits<T1>::is_modulo;
}

template <typename T1> bool f797() { return std::numeric_limits<T1>::traps; }

template <typename T1> bool f798() {
  return std::numeric_limits<T1>::tinyness_before;
}

template <typename T1> std::float_round_style f799() {
  return std::numeric_limits<T1>::round_style;
}
