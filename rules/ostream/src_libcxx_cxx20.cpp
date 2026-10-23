#include <chrono>
#include <iosfwd>
#include <ostream>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f23(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::day &a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f24(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::month &a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f25(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::month_day &a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f26(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::month_day_last &a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f27(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::month_weekday &a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f28(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::month_weekday_last &a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f29(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::sys_days &a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f30(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::weekday &a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f31(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::weekday_indexed &a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f32(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::weekday_last &a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f33(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::year &a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f34(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::year_month &a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f35(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::year_month_day &a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f36(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::year_month_day_last &a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f37(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::year_month_weekday &a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &
f38(std::basic_ostream<T1, T2> &a0,
    const std::chrono::year_month_weekday_last &a1) {
  return a0 << a1;
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f39(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::sys_time<T3> &a1) {
  return a0 << a1;
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f40(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::hh_mm_ss<T3> a1) {
  return a0 << a1;
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f41(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::file_time<T3> a1) {
  return a0 << a1;
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f42(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::local_time<T3> a1) {
  return a0 << a1;
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_ostream<T1, T2> &f43(std::basic_ostream<T1, T2> &a0,
                                const std::chrono::duration<T3, T4> &a1) {
  return a0 << a1;
}
