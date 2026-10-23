#include <chrono>
#include <utility>

template <typename T1> using t1 = std::chrono::hh_mm_ss<T1>;

template <typename T1> std::chrono::hh_mm_ss<T1> f1() {
  return std::chrono::hh_mm_ss<T1>();
}

template <typename T1> std::chrono::hh_mm_ss<T1> f2(T1 a0) {
  return std::chrono::hh_mm_ss<T1>(a0);
}

template <typename T1> bool f3(const std::chrono::hh_mm_ss<T1> &a0) {
  return a0.is_negative();
}

template <typename T1>
std::chrono::hours f4(const std::chrono::hh_mm_ss<T1> &a0) {
  return a0.hours();
}

template <typename T1>
std::chrono::minutes f5(const std::chrono::hh_mm_ss<T1> &a0) {
  return a0.minutes();
}

template <typename T1>
std::chrono::seconds f6(const std::chrono::hh_mm_ss<T1> &a0) {
  return a0.seconds();
}

template <typename T1>
typename std::chrono::hh_mm_ss<T1>::precision
f7(const std::chrono::hh_mm_ss<T1> &a0) {
  return a0.subseconds();
}

template <typename T1>
typename std::chrono::hh_mm_ss<T1>::precision
f8(const std::chrono::hh_mm_ss<T1> &a0) {
  return static_cast<typename std::chrono::hh_mm_ss<T1>::precision>(a0);
}

template <typename T1>
typename std::chrono::hh_mm_ss<T1>::precision
f9(const std::chrono::hh_mm_ss<T1> &a0) {
  return a0.to_duration();
}

template <typename T1> unsigned int f10() {
  return std::chrono::hh_mm_ss<T1>::fractional_width;
}
