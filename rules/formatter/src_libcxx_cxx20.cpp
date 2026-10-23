#include <chrono>
#include <format>
#include <string>
#include <string_view>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
typename T2::iterator f118(std::formatter<std::chrono::day, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator f119(std::formatter<std::chrono::month, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator f120(std::formatter<std::chrono::month_day, T1> &a0,
                           T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator f121(std::formatter<std::chrono::month_day_last, T1> &a0,
                           T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator f122(std::formatter<std::chrono::month_weekday, T1> &a0,
                           T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator
f123(std::formatter<std::chrono::month_weekday_last, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator f124(std::formatter<std::chrono::weekday, T1> &a0,
                           T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator f125(std::formatter<std::chrono::weekday_indexed, T1> &a0,
                           T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator f126(std::formatter<std::chrono::weekday_last, T1> &a0,
                           T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator f127(std::formatter<std::chrono::year, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator f128(std::formatter<std::chrono::year_month, T1> &a0,
                           T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator f129(std::formatter<std::chrono::year_month_day, T1> &a0,
                           T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator
f130(std::formatter<std::chrono::year_month_day_last, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator
f131(std::formatter<std::chrono::year_month_weekday, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator
f132(std::formatter<std::chrono::year_month_weekday_last, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator f133(const std::formatter<const T1 *, T1> &a0,
                           const T1 *a1, T2 &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1, typename T2>
typename T2::iterator f134(const std::formatter<T1 *, T1> &a0, T1 *a1, T2 &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t50 = std::formatter<bool, T1>;

template <typename T1, typename T2>
typename T2::iterator f135(std::formatter<bool, T1> &a0, T2 &a1) {
  return a0.template parse<T2>(a1);
}

template <typename T1, typename T2>
typename T2::iterator f136(const std::formatter<bool, T1> &a0, bool a1,
                           T2 &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1, typename T2>
using t51 = std::formatter<std::chrono::hh_mm_ss<T1>, T2>;

template <typename T1, typename T2, typename T3>
typename T3::iterator f137(std::formatter<std::chrono::hh_mm_ss<T1>, T2> &a0,
                           T3 &a1) {
  return a0.template parse<T3>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f138(std::formatter<std::chrono::local_time<T1>, T2> &a0,
                           T3 &a1) {
  return a0.template parse<T3>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f139(std::formatter<std::chrono::sys_time<T1>, T2> &a0,
                           T3 &a1) {
  return a0.template parse<T3>(a1);
}

template <typename T1, typename T2>
using t52 = std::formatter<std::basic_string_view<T1, T2>, T1>;

template <typename T1, typename T2, typename T3>
typename T3::iterator
f140(const std::formatter<std::basic_string_view<T1, T2>, T1> &a0,
     std::basic_string_view<T1, T2> a1, T3 &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename T4::iterator
f141(std::formatter<std::chrono::duration<T1, T2>, T3> &a0, T4 &a1) {
  return a0.template parse<T4>(a1);
}

template <typename T1, typename T2, typename T3>
using t53 = std::formatter<std::basic_string<T1, T2, T3>, T1>;

template <typename T1, typename T2, typename T3, typename T4>
typename T4::iterator
f142(const std::formatter<std::basic_string<T1, T2, T3>, T1> &a0,
     const std::basic_string<T1, T2, T3> &a1, T4 &a2) {
  return a0.template format<T4>(a1, a2);
}
