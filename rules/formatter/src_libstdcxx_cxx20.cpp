#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <chrono>
#include <format>
#include <string>
#include <string_view>
#include <utility>

using t22 = std::formatter<char, wchar_t>;

std::formatter<char, wchar_t> f2() { return std::formatter<char, wchar_t>(); }

const std::basic_string_view<wchar_t>::value_type *
f3(std::formatter<char, wchar_t> &a0,
   std::basic_format_parse_context<wchar_t> &a1) {
  return a0.parse(a1);
}

std::formatter<char, wchar_t> f4(const std::formatter<char, wchar_t> &a0) {
  return std::formatter<char, wchar_t>(a0);
}

std::formatter<char, wchar_t> f5(std::formatter<char, wchar_t> &&a0) {
  return std::formatter<char, wchar_t>(std::move(a0));
}

std::formatter<char, wchar_t> &f6(std::formatter<char, wchar_t> &a0,
                                  const std::formatter<char, wchar_t> &a1) {
  return a0.operator=(a1);
}

std::formatter<char, wchar_t> &f7(std::formatter<char, wchar_t> &a0,
                                  std::formatter<char, wchar_t> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
typename std::basic_format_context<T1, wchar_t>::iterator
f8(const std::formatter<char, wchar_t> &a0, char a1,
   std::basic_format_context<T1, wchar_t> &a2) {
  return a0.template format<T1>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f9(std::formatter<std::chrono::day, T1> &a0,
   std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f10(const std::formatter<std::chrono::day, T1> &a0, const std::chrono::day &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t23 = std::formatter<std::chrono::local_info, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f11(std::formatter<std::chrono::local_info, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f12(const std::formatter<std::chrono::local_info, T1> &a0,
    const std::chrono::local_info &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f13(std::formatter<std::chrono::month, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f14(const std::formatter<std::chrono::month, T1> &a0,
    const std::chrono::month &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f15(std::formatter<std::chrono::month_day, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f16(const std::formatter<std::chrono::month_day, T1> &a0,
    const std::chrono::month_day &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f17(std::formatter<std::chrono::month_day_last, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f18(const std::formatter<std::chrono::month_day_last, T1> &a0,
    const std::chrono::month_day_last &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f19(std::formatter<std::chrono::month_weekday, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f20(const std::formatter<std::chrono::month_weekday, T1> &a0,
    const std::chrono::month_weekday &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f21(std::formatter<std::chrono::month_weekday_last, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f22(const std::formatter<std::chrono::month_weekday_last, T1> &a0,
    const std::chrono::month_weekday_last &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t24 = std::formatter<std::chrono::sys_info, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f23(std::formatter<std::chrono::sys_info, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f24(const std::formatter<std::chrono::sys_info, T1> &a0,
    const std::chrono::sys_info &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f25(std::formatter<std::chrono::weekday, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f26(const std::formatter<std::chrono::weekday, T1> &a0,
    const std::chrono::weekday &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f27(std::formatter<std::chrono::weekday_indexed, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f28(const std::formatter<std::chrono::weekday_indexed, T1> &a0,
    const std::chrono::weekday_indexed &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f29(std::formatter<std::chrono::weekday_last, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f30(const std::formatter<std::chrono::weekday_last, T1> &a0,
    const std::chrono::weekday_last &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f31(std::formatter<std::chrono::year, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f32(const std::formatter<std::chrono::year, T1> &a0,
    const std::chrono::year &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f33(std::formatter<std::chrono::year_month, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f34(const std::formatter<std::chrono::year_month, T1> &a0,
    const std::chrono::year_month &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f35(std::formatter<std::chrono::year_month_day, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f36(const std::formatter<std::chrono::year_month_day, T1> &a0,
    const std::chrono::year_month_day &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f37(std::formatter<std::chrono::year_month_day_last, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f38(const std::formatter<std::chrono::year_month_day_last, T1> &a0,
    const std::chrono::year_month_day_last &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f39(std::formatter<std::chrono::year_month_weekday, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f40(const std::formatter<std::chrono::year_month_weekday, T1> &a0,
    const std::chrono::year_month_weekday &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f41(std::formatter<std::chrono::year_month_weekday_last, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f42(const std::formatter<std::chrono::year_month_weekday_last, T1> &a0,
    const std::chrono::year_month_weekday_last &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> std::formatter<const T1 *, T1> f43() {
  return std::formatter<const T1 *, T1>();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f44(std::formatter<const T1 *, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f45(const std::formatter<const T1 *, T1> &a0, const T1 *a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t25 = std::formatter<const void *, T1>;

template <typename T1> std::formatter<const void *, T1> f46() {
  return std::formatter<const void *, T1>();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f47(std::formatter<const void *, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f48(const std::formatter<const void *, T1> &a0, const void *a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> std::formatter<T1 *, T1> f49() {
  return std::formatter<T1 *, T1>();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f50(std::formatter<T1 *, T1> &a0, std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f51(const std::formatter<T1 *, T1> &a0, T1 *a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t26 = std::formatter<void *, T1>;

template <typename T1> std::formatter<void *, T1> f52() {
  return std::formatter<void *, T1>();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f53(std::formatter<void *, T1> &a0, std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f54(const std::formatter<void *, T1> &a0, void *a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t27 = std::formatter<std::basic_string_view<char, T1>, char>;

template <typename T1>
std::formatter<std::basic_string_view<char, T1>, char> f55() {
  return std::formatter<std::basic_string_view<char, T1>, char>();
}

template <typename T1>
const std::basic_string_view<char>::value_type *
f56(std::formatter<std::basic_string_view<char, T1>, char> &a0,
    std::basic_format_parse_context<char> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, char>::iterator
f57(const std::formatter<std::basic_string_view<char, T1>, char> &a0,
    std::basic_string_view<char, T1> a1,
    std::basic_format_context<T2, char> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t28 = std::formatter<std::basic_string_view<wchar_t, T1>, wchar_t>;

template <typename T1>
std::formatter<std::basic_string_view<wchar_t, T1>, wchar_t> f58() {
  return std::formatter<std::basic_string_view<wchar_t, T1>, wchar_t>();
}

template <typename T1>
const std::basic_string_view<wchar_t>::value_type *
f59(std::formatter<std::basic_string_view<wchar_t, T1>, wchar_t> &a0,
    std::basic_format_parse_context<wchar_t> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, wchar_t>::iterator
f60(const std::formatter<std::basic_string_view<wchar_t, T1>, wchar_t> &a0,
    std::basic_string_view<wchar_t, T1> a1,
    std::basic_format_context<T2, wchar_t> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t29 = std::formatter<std::nullptr_t, T1>;

template <typename T1> std::formatter<std::nullptr_t, T1> f61() {
  return std::formatter<std::nullptr_t, T1>();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f62(std::formatter<std::nullptr_t, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f63(const std::formatter<std::nullptr_t, T1> &a0, std::nullptr_t a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t30 = std::formatter<T1, T1>;

template <typename T1> std::formatter<T1, T1> f64() {
  return std::formatter<T1, T1>();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f65(std::formatter<T1, T1> &a0, std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f66(const std::formatter<T1, T1> &a0, T1 a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1, typename T2>
using t31 = std::formatter<std::basic_string<char, T1, T2>, char>;

template <typename T1, typename T2>
std::formatter<std::basic_string<char, T1, T2>, char> f67() {
  return std::formatter<std::basic_string<char, T1, T2>, char>();
}

template <typename T1, typename T2>
const std::basic_string_view<char>::value_type *
f68(std::formatter<std::basic_string<char, T1, T2>, char> &a0,
    std::basic_format_parse_context<char> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, char>::iterator
f69(const std::formatter<std::basic_string<char, T1, T2>, char> &a0,
    const std::basic_string<char, T1, T2> &a1,
    std::basic_format_context<T3, char> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
using t32 = std::formatter<std::basic_string<wchar_t, T1, T2>, wchar_t>;

template <typename T1, typename T2>
std::formatter<std::basic_string<wchar_t, T1, T2>, wchar_t> f70() {
  return std::formatter<std::basic_string<wchar_t, T1, T2>, wchar_t>();
}

template <typename T1, typename T2>
const std::basic_string_view<wchar_t>::value_type *
f71(std::formatter<std::basic_string<wchar_t, T1, T2>, wchar_t> &a0,
    std::basic_format_parse_context<wchar_t> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, wchar_t>::iterator
f72(const std::formatter<std::basic_string<wchar_t, T1, T2>, wchar_t> &a0,
    const std::basic_string<wchar_t, T1, T2> &a1,
    std::basic_format_context<T3, wchar_t> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f73(std::formatter<std::chrono::sys_time<T1>, T2> &a0,
    std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f74(const std::formatter<std::chrono::sys_time<T1>, T2> &a0,
    const std::chrono::sys_time<T1> &a1,
    std::basic_format_context<T3, T2> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
using t33 = std::formatter<std::chrono::gps_time<T1>, T2>;

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f75(std::formatter<std::chrono::gps_time<T1>, T2> &a0,
    std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f76(const std::formatter<std::chrono::gps_time<T1>, T2> &a0,
    const std::chrono::gps_time<T1> &a1,
    std::basic_format_context<T3, T2> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f77(std::formatter<std::chrono::local_time<T1>, T2> &a0,
    std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f78(const std::formatter<std::chrono::local_time<T1>, T2> &a0,
    const std::chrono::local_time<T1> &a1,
    std::basic_format_context<T3, T2> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
using t34 = std::formatter<std::chrono::tai_time<T1>, T2>;

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f79(std::formatter<std::chrono::tai_time<T1>, T2> &a0,
    std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f80(const std::formatter<std::chrono::tai_time<T1>, T2> &a0,
    const std::chrono::tai_time<T1> &a1,
    std::basic_format_context<T3, T2> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
using t35 = std::formatter<std::chrono::utc_time<T1>, T2>;

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f81(std::formatter<std::chrono::utc_time<T1>, T2> &a0,
    std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f82(const std::formatter<std::chrono::utc_time<T1>, T2> &a0,
    const std::chrono::utc_time<T1> &a1,
    std::basic_format_context<T3, T2> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2> std::formatter<T1, T2> f83() {
  return std::formatter<T1, T2>();
}

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f84(std::formatter<T1, T2> &a0, std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f85(const std::formatter<T1, T2> &a0, T1 a1,
    std::basic_format_context<T3, T2> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3>
using t36 =
    std::formatter<std::chrono::hh_mm_ss<std::chrono::duration<T1, T2>>, T3>;

template <typename T1, typename T2, typename T3>
typename std::basic_format_parse_context<T3>::iterator
f86(std::formatter<std::chrono::hh_mm_ss<std::chrono::duration<T1, T2>>, T3>
        &a0,
    std::basic_format_parse_context<T3> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_format_context<T4, T3>::iterator
f87(const std::formatter<std::chrono::hh_mm_ss<std::chrono::duration<T1, T2>>,
                         T3> &a0,
    const std::chrono::hh_mm_ss<std::chrono::duration<T1, T2>> &a1,
    std::basic_format_context<T4, T3> &a2) {
  return a0.template format<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_parse_context<T3>::iterator
f88(std::formatter<std::chrono::duration<T1, T2>, T3> &a0,
    std::basic_format_parse_context<T3> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_format_context<T4, T3>::iterator
f89(const std::formatter<std::chrono::duration<T1, T2>, T3> &a0,
    const std::chrono::duration<T1, T2> &a1,
    std::basic_format_context<T4, T3> &a2) {
  return a0.template format<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3>
using t37 = std::formatter<std::chrono::zoned_time<T1, T2>, T3>;

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_format_context<T4, T3>::iterator
f90(const std::formatter<std::chrono::zoned_time<T1, T2>, T3> &a0,
    const std::chrono::zoned_time<T1, T2> &a1,
    std::basic_format_context<T4, T3> &a2) {
  return a0.template format<T4>(a1, a2);
}
