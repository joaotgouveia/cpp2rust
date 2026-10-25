#include <limits>
#include <utility>

template <typename T1>
auto f851() -> decltype(std::numeric_limits<const T1>::min()) {
  return std::numeric_limits<const T1>::min();
}

template <typename T1>
auto f852() -> decltype(std::numeric_limits<const T1>::max()) {
  return std::numeric_limits<const T1>::max();
}

template <typename T1>
auto f853() -> decltype(std::numeric_limits<const T1>::lowest()) {
  return std::numeric_limits<const T1>::lowest();
}

template <typename T1>
auto f854() -> decltype(std::numeric_limits<const T1>::epsilon()) {
  return std::numeric_limits<const T1>::epsilon();
}

template <typename T1>
auto f855() -> decltype(std::numeric_limits<const T1>::round_error()) {
  return std::numeric_limits<const T1>::round_error();
}

template <typename T1>
auto f856() -> decltype(std::numeric_limits<const T1>::infinity()) {
  return std::numeric_limits<const T1>::infinity();
}

template <typename T1>
auto f857() -> decltype(std::numeric_limits<const T1>::quiet_NaN()) {
  return std::numeric_limits<const T1>::quiet_NaN();
}

template <typename T1>
auto f858() -> decltype(std::numeric_limits<const T1>::signaling_NaN()) {
  return std::numeric_limits<const T1>::signaling_NaN();
}

template <typename T1>
auto f859() -> decltype(std::numeric_limits<const T1>::denorm_min()) {
  return std::numeric_limits<const T1>::denorm_min();
}

template <typename T1>
auto f860() -> decltype(std::numeric_limits<volatile T1>::min()) {
  return std::numeric_limits<volatile T1>::min();
}

template <typename T1>
auto f861() -> decltype(std::numeric_limits<volatile T1>::max()) {
  return std::numeric_limits<volatile T1>::max();
}

template <typename T1>
auto f862() -> decltype(std::numeric_limits<volatile T1>::lowest()) {
  return std::numeric_limits<volatile T1>::lowest();
}

template <typename T1>
auto f863() -> decltype(std::numeric_limits<volatile T1>::epsilon()) {
  return std::numeric_limits<volatile T1>::epsilon();
}

template <typename T1>
auto f864() -> decltype(std::numeric_limits<volatile T1>::round_error()) {
  return std::numeric_limits<volatile T1>::round_error();
}

template <typename T1>
auto f865() -> decltype(std::numeric_limits<volatile T1>::infinity()) {
  return std::numeric_limits<volatile T1>::infinity();
}

template <typename T1>
auto f866() -> decltype(std::numeric_limits<volatile T1>::quiet_NaN()) {
  return std::numeric_limits<volatile T1>::quiet_NaN();
}

template <typename T1>
auto f867() -> decltype(std::numeric_limits<volatile T1>::signaling_NaN()) {
  return std::numeric_limits<volatile T1>::signaling_NaN();
}

template <typename T1>
auto f868() -> decltype(std::numeric_limits<volatile T1>::denorm_min()) {
  return std::numeric_limits<volatile T1>::denorm_min();
}

template <typename T1>
auto f869() -> decltype(std::numeric_limits<const volatile T1>::min()) {
  return std::numeric_limits<const volatile T1>::min();
}

template <typename T1>
auto f870() -> decltype(std::numeric_limits<const volatile T1>::max()) {
  return std::numeric_limits<const volatile T1>::max();
}

template <typename T1>
auto f871() -> decltype(std::numeric_limits<const volatile T1>::lowest()) {
  return std::numeric_limits<const volatile T1>::lowest();
}

template <typename T1>
auto f872() -> decltype(std::numeric_limits<const volatile T1>::epsilon()) {
  return std::numeric_limits<const volatile T1>::epsilon();
}

template <typename T1>
auto f873() -> decltype(std::numeric_limits<const volatile T1>::round_error()) {
  return std::numeric_limits<const volatile T1>::round_error();
}

template <typename T1>
auto f874() -> decltype(std::numeric_limits<const volatile T1>::infinity()) {
  return std::numeric_limits<const volatile T1>::infinity();
}

template <typename T1>
auto f875() -> decltype(std::numeric_limits<const volatile T1>::quiet_NaN()) {
  return std::numeric_limits<const volatile T1>::quiet_NaN();
}

template <typename T1>
auto f876()
    -> decltype(std::numeric_limits<const volatile T1>::signaling_NaN()) {
  return std::numeric_limits<const volatile T1>::signaling_NaN();
}

template <typename T1>
auto f877() -> decltype(std::numeric_limits<const volatile T1>::denorm_min()) {
  return std::numeric_limits<const volatile T1>::denorm_min();
}

template <typename T1> auto f878() -> decltype(std::numeric_limits<T1>::min()) {
  return std::numeric_limits<T1>::min();
}

template <typename T1> auto f879() -> decltype(std::numeric_limits<T1>::max()) {
  return std::numeric_limits<T1>::max();
}

template <typename T1>
auto f880() -> decltype(std::numeric_limits<T1>::lowest()) {
  return std::numeric_limits<T1>::lowest();
}

template <typename T1>
auto f881() -> decltype(std::numeric_limits<T1>::epsilon()) {
  return std::numeric_limits<T1>::epsilon();
}

template <typename T1>
auto f882() -> decltype(std::numeric_limits<T1>::round_error()) {
  return std::numeric_limits<T1>::round_error();
}

template <typename T1>
auto f883() -> decltype(std::numeric_limits<T1>::infinity()) {
  return std::numeric_limits<T1>::infinity();
}

template <typename T1>
auto f884() -> decltype(std::numeric_limits<T1>::quiet_NaN()) {
  return std::numeric_limits<T1>::quiet_NaN();
}

template <typename T1>
auto f885() -> decltype(std::numeric_limits<T1>::signaling_NaN()) {
  return std::numeric_limits<T1>::signaling_NaN();
}

template <typename T1>
auto f886() -> decltype(std::numeric_limits<T1>::denorm_min()) {
  return std::numeric_limits<T1>::denorm_min();
}

template <typename T1> bool f887() {
  return std::numeric_limits<T1>::is_specialized;
}

template <typename T1> int f888() { return std::numeric_limits<T1>::digits; }

template <typename T1> int f889() { return std::numeric_limits<T1>::digits10; }

template <typename T1> int f890() {
  return std::numeric_limits<T1>::max_digits10;
}

template <typename T1> bool f891() {
  return std::numeric_limits<T1>::is_signed;
}

template <typename T1> bool f892() {
  return std::numeric_limits<T1>::is_integer;
}

template <typename T1> bool f893() { return std::numeric_limits<T1>::is_exact; }

template <typename T1> int f894() { return std::numeric_limits<T1>::radix; }

template <typename T1> int f895() {
  return std::numeric_limits<T1>::min_exponent;
}

template <typename T1> int f896() {
  return std::numeric_limits<T1>::min_exponent10;
}

template <typename T1> int f897() {
  return std::numeric_limits<T1>::max_exponent;
}

template <typename T1> int f898() {
  return std::numeric_limits<T1>::max_exponent10;
}

template <typename T1> bool f899() {
  return std::numeric_limits<T1>::has_infinity;
}

template <typename T1> bool f900() {
  return std::numeric_limits<T1>::has_quiet_NaN;
}

template <typename T1> bool f901() {
  return std::numeric_limits<T1>::has_signaling_NaN;
}

template <typename T1> std::float_denorm_style f902() {
  return std::numeric_limits<T1>::has_denorm;
}

template <typename T1> bool f903() {
  return std::numeric_limits<T1>::has_denorm_loss;
}

template <typename T1> bool f904() {
  return std::numeric_limits<T1>::is_iec559;
}

template <typename T1> bool f905() {
  return std::numeric_limits<T1>::is_bounded;
}

template <typename T1> bool f906() {
  return std::numeric_limits<T1>::is_modulo;
}

template <typename T1> bool f907() { return std::numeric_limits<T1>::traps; }

template <typename T1> bool f908() {
  return std::numeric_limits<T1>::tinyness_before;
}

template <typename T1> std::float_round_style f909() {
  return std::numeric_limits<T1>::round_style;
}
