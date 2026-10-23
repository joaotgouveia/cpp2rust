#include <limits>
#include <utility>

template <typename T1>
auto f777() -> decltype(std::numeric_limits<const T1>::min()) {
  return std::numeric_limits<const T1>::min();
}

template <typename T1>
auto f778() -> decltype(std::numeric_limits<const T1>::max()) {
  return std::numeric_limits<const T1>::max();
}

template <typename T1>
auto f779() -> decltype(std::numeric_limits<const T1>::lowest()) {
  return std::numeric_limits<const T1>::lowest();
}

template <typename T1>
auto f780() -> decltype(std::numeric_limits<const T1>::epsilon()) {
  return std::numeric_limits<const T1>::epsilon();
}

template <typename T1>
auto f781() -> decltype(std::numeric_limits<const T1>::round_error()) {
  return std::numeric_limits<const T1>::round_error();
}

template <typename T1>
auto f782() -> decltype(std::numeric_limits<const T1>::infinity()) {
  return std::numeric_limits<const T1>::infinity();
}

template <typename T1>
auto f783() -> decltype(std::numeric_limits<const T1>::quiet_NaN()) {
  return std::numeric_limits<const T1>::quiet_NaN();
}

template <typename T1>
auto f784() -> decltype(std::numeric_limits<const T1>::signaling_NaN()) {
  return std::numeric_limits<const T1>::signaling_NaN();
}

template <typename T1>
auto f785() -> decltype(std::numeric_limits<const T1>::denorm_min()) {
  return std::numeric_limits<const T1>::denorm_min();
}

template <typename T1>
auto f786() -> decltype(std::numeric_limits<volatile T1>::min()) {
  return std::numeric_limits<volatile T1>::min();
}

template <typename T1>
auto f787() -> decltype(std::numeric_limits<volatile T1>::max()) {
  return std::numeric_limits<volatile T1>::max();
}

template <typename T1>
auto f788() -> decltype(std::numeric_limits<volatile T1>::lowest()) {
  return std::numeric_limits<volatile T1>::lowest();
}

template <typename T1>
auto f789() -> decltype(std::numeric_limits<volatile T1>::epsilon()) {
  return std::numeric_limits<volatile T1>::epsilon();
}

template <typename T1>
auto f790() -> decltype(std::numeric_limits<volatile T1>::round_error()) {
  return std::numeric_limits<volatile T1>::round_error();
}

template <typename T1>
auto f791() -> decltype(std::numeric_limits<volatile T1>::infinity()) {
  return std::numeric_limits<volatile T1>::infinity();
}

template <typename T1>
auto f792() -> decltype(std::numeric_limits<volatile T1>::quiet_NaN()) {
  return std::numeric_limits<volatile T1>::quiet_NaN();
}

template <typename T1>
auto f793() -> decltype(std::numeric_limits<volatile T1>::signaling_NaN()) {
  return std::numeric_limits<volatile T1>::signaling_NaN();
}

template <typename T1>
auto f794() -> decltype(std::numeric_limits<volatile T1>::denorm_min()) {
  return std::numeric_limits<volatile T1>::denorm_min();
}

template <typename T1>
auto f795() -> decltype(std::numeric_limits<const volatile T1>::min()) {
  return std::numeric_limits<const volatile T1>::min();
}

template <typename T1>
auto f796() -> decltype(std::numeric_limits<const volatile T1>::max()) {
  return std::numeric_limits<const volatile T1>::max();
}

template <typename T1>
auto f797() -> decltype(std::numeric_limits<const volatile T1>::lowest()) {
  return std::numeric_limits<const volatile T1>::lowest();
}

template <typename T1>
auto f798() -> decltype(std::numeric_limits<const volatile T1>::epsilon()) {
  return std::numeric_limits<const volatile T1>::epsilon();
}

template <typename T1>
auto f799() -> decltype(std::numeric_limits<const volatile T1>::round_error()) {
  return std::numeric_limits<const volatile T1>::round_error();
}

template <typename T1>
auto f800() -> decltype(std::numeric_limits<const volatile T1>::infinity()) {
  return std::numeric_limits<const volatile T1>::infinity();
}

template <typename T1>
auto f801() -> decltype(std::numeric_limits<const volatile T1>::quiet_NaN()) {
  return std::numeric_limits<const volatile T1>::quiet_NaN();
}

template <typename T1>
auto f802()
    -> decltype(std::numeric_limits<const volatile T1>::signaling_NaN()) {
  return std::numeric_limits<const volatile T1>::signaling_NaN();
}

template <typename T1>
auto f803() -> decltype(std::numeric_limits<const volatile T1>::denorm_min()) {
  return std::numeric_limits<const volatile T1>::denorm_min();
}

template <typename T1> auto f804() -> decltype(std::numeric_limits<T1>::min()) {
  return std::numeric_limits<T1>::min();
}

template <typename T1> auto f805() -> decltype(std::numeric_limits<T1>::max()) {
  return std::numeric_limits<T1>::max();
}

template <typename T1>
auto f806() -> decltype(std::numeric_limits<T1>::lowest()) {
  return std::numeric_limits<T1>::lowest();
}

template <typename T1>
auto f807() -> decltype(std::numeric_limits<T1>::epsilon()) {
  return std::numeric_limits<T1>::epsilon();
}

template <typename T1>
auto f808() -> decltype(std::numeric_limits<T1>::round_error()) {
  return std::numeric_limits<T1>::round_error();
}

template <typename T1>
auto f809() -> decltype(std::numeric_limits<T1>::infinity()) {
  return std::numeric_limits<T1>::infinity();
}

template <typename T1>
auto f810() -> decltype(std::numeric_limits<T1>::quiet_NaN()) {
  return std::numeric_limits<T1>::quiet_NaN();
}

template <typename T1>
auto f811() -> decltype(std::numeric_limits<T1>::signaling_NaN()) {
  return std::numeric_limits<T1>::signaling_NaN();
}

template <typename T1>
auto f812() -> decltype(std::numeric_limits<T1>::denorm_min()) {
  return std::numeric_limits<T1>::denorm_min();
}

template <typename T1> bool f813() {
  return std::numeric_limits<T1>::is_specialized;
}

template <typename T1> int f814() { return std::numeric_limits<T1>::digits; }

template <typename T1> int f815() { return std::numeric_limits<T1>::digits10; }

template <typename T1> int f816() {
  return std::numeric_limits<T1>::max_digits10;
}

template <typename T1> bool f817() {
  return std::numeric_limits<T1>::is_signed;
}

template <typename T1> bool f818() {
  return std::numeric_limits<T1>::is_integer;
}

template <typename T1> bool f819() { return std::numeric_limits<T1>::is_exact; }

template <typename T1> int f820() { return std::numeric_limits<T1>::radix; }

template <typename T1> int f821() {
  return std::numeric_limits<T1>::min_exponent;
}

template <typename T1> int f822() {
  return std::numeric_limits<T1>::min_exponent10;
}

template <typename T1> int f823() {
  return std::numeric_limits<T1>::max_exponent;
}

template <typename T1> int f824() {
  return std::numeric_limits<T1>::max_exponent10;
}

template <typename T1> bool f825() {
  return std::numeric_limits<T1>::has_infinity;
}

template <typename T1> bool f826() {
  return std::numeric_limits<T1>::has_quiet_NaN;
}

template <typename T1> bool f827() {
  return std::numeric_limits<T1>::has_signaling_NaN;
}

template <typename T1> std::float_denorm_style f828() {
  return std::numeric_limits<T1>::has_denorm;
}

template <typename T1> bool f829() {
  return std::numeric_limits<T1>::has_denorm_loss;
}

template <typename T1> bool f830() {
  return std::numeric_limits<T1>::is_iec559;
}

template <typename T1> bool f831() {
  return std::numeric_limits<T1>::is_bounded;
}

template <typename T1> bool f832() {
  return std::numeric_limits<T1>::is_modulo;
}

template <typename T1> bool f833() { return std::numeric_limits<T1>::traps; }

template <typename T1> bool f834() {
  return std::numeric_limits<T1>::tinyness_before;
}

template <typename T1> std::float_round_style f835() {
  return std::numeric_limits<T1>::round_style;
}
