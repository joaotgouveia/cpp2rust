#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <chrono>
#include <format>
#include <string>
#include <string_view>
#include <utility>

using t1 = std::formatter<char, wchar_t>;

std::formatter<char, wchar_t> f1() { return std::formatter<char, wchar_t>(); }

const std::basic_string_view<wchar_t>::value_type *
f2(std::formatter<char, wchar_t> &a0,
   std::basic_format_parse_context<wchar_t> &a1) {
  return a0.parse(a1);
}

std::formatter<char, wchar_t> f3(const std::formatter<char, wchar_t> &a0) {
  return std::formatter<char, wchar_t>(a0);
}

std::formatter<char, wchar_t> f4(std::formatter<char, wchar_t> &&a0) {
  return std::formatter<char, wchar_t>(std::move(a0));
}

std::formatter<char, wchar_t> &f5(std::formatter<char, wchar_t> &a0,
                                  const std::formatter<char, wchar_t> &a1) {
  return a0.operator=(a1);
}

std::formatter<char, wchar_t> &f6(std::formatter<char, wchar_t> &a0,
                                  std::formatter<char, wchar_t> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
typename std::basic_format_context<T1, wchar_t>::iterator
f7(const std::formatter<char, wchar_t> &a0, char a1,
   std::basic_format_context<T1, wchar_t> &a2) {
  return a0.template format<T1>(a1, a2);
}

template <typename T1> using t2 = std::formatter<std::chrono::day, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f8(std::formatter<std::chrono::day, T1> &a0,
   std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f9(const std::formatter<std::chrono::day, T1> &a0, const std::chrono::day &a1,
   std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t3 = std::formatter<std::chrono::local_info, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f10(std::formatter<std::chrono::local_info, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f11(const std::formatter<std::chrono::local_info, T1> &a0,
    const std::chrono::local_info &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t4 = std::formatter<std::chrono::month, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f12(std::formatter<std::chrono::month, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f13(const std::formatter<std::chrono::month, T1> &a0,
    const std::chrono::month &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t5 = std::formatter<std::chrono::month_day, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f14(std::formatter<std::chrono::month_day, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f15(const std::formatter<std::chrono::month_day, T1> &a0,
    const std::chrono::month_day &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t6 = std::formatter<std::chrono::month_day_last, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f16(std::formatter<std::chrono::month_day_last, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f17(const std::formatter<std::chrono::month_day_last, T1> &a0,
    const std::chrono::month_day_last &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t7 = std::formatter<std::chrono::month_weekday, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f18(std::formatter<std::chrono::month_weekday, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f19(const std::formatter<std::chrono::month_weekday, T1> &a0,
    const std::chrono::month_weekday &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t8 = std::formatter<std::chrono::month_weekday_last, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f20(std::formatter<std::chrono::month_weekday_last, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f21(const std::formatter<std::chrono::month_weekday_last, T1> &a0,
    const std::chrono::month_weekday_last &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t9 = std::formatter<std::chrono::sys_info, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f22(std::formatter<std::chrono::sys_info, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f23(const std::formatter<std::chrono::sys_info, T1> &a0,
    const std::chrono::sys_info &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t10 = std::formatter<std::chrono::weekday, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f24(std::formatter<std::chrono::weekday, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f25(const std::formatter<std::chrono::weekday, T1> &a0,
    const std::chrono::weekday &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t11 = std::formatter<std::chrono::weekday_indexed, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f26(std::formatter<std::chrono::weekday_indexed, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f27(const std::formatter<std::chrono::weekday_indexed, T1> &a0,
    const std::chrono::weekday_indexed &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t12 = std::formatter<std::chrono::weekday_last, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f28(std::formatter<std::chrono::weekday_last, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f29(const std::formatter<std::chrono::weekday_last, T1> &a0,
    const std::chrono::weekday_last &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t13 = std::formatter<std::chrono::year, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f30(std::formatter<std::chrono::year, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f31(const std::formatter<std::chrono::year, T1> &a0,
    const std::chrono::year &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t14 = std::formatter<std::chrono::year_month, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f32(std::formatter<std::chrono::year_month, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f33(const std::formatter<std::chrono::year_month, T1> &a0,
    const std::chrono::year_month &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t15 = std::formatter<std::chrono::year_month_day, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f34(std::formatter<std::chrono::year_month_day, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f35(const std::formatter<std::chrono::year_month_day, T1> &a0,
    const std::chrono::year_month_day &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t16 = std::formatter<std::chrono::year_month_day_last, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f36(std::formatter<std::chrono::year_month_day_last, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f37(const std::formatter<std::chrono::year_month_day_last, T1> &a0,
    const std::chrono::year_month_day_last &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t17 = std::formatter<std::chrono::year_month_weekday, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f38(std::formatter<std::chrono::year_month_weekday, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f39(const std::formatter<std::chrono::year_month_weekday, T1> &a0,
    const std::chrono::year_month_weekday &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t18 = std::formatter<std::chrono::year_month_weekday_last, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f40(std::formatter<std::chrono::year_month_weekday_last, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f41(const std::formatter<std::chrono::year_month_weekday_last, T1> &a0,
    const std::chrono::year_month_weekday_last &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t19 = std::formatter<const T1 *, T1>;

template <typename T1> std::formatter<const T1 *, T1> f42() {
  return std::formatter<const T1 *, T1>();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f43(std::formatter<const T1 *, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f44(const std::formatter<const T1 *, T1> &a0, const T1 *a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t20 = std::formatter<const void *, T1>;

template <typename T1> std::formatter<const void *, T1> f45() {
  return std::formatter<const void *, T1>();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f46(std::formatter<const void *, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f47(const std::formatter<const void *, T1> &a0, const void *a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t21 = std::formatter<T1 *, T1>;

template <typename T1> std::formatter<T1 *, T1> f48() {
  return std::formatter<T1 *, T1>();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f49(std::formatter<T1 *, T1> &a0, std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f50(const std::formatter<T1 *, T1> &a0, T1 *a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t22 = std::formatter<void *, T1>;

template <typename T1> std::formatter<void *, T1> f51() {
  return std::formatter<void *, T1>();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f52(std::formatter<void *, T1> &a0, std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f53(const std::formatter<void *, T1> &a0, void *a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t23 = std::formatter<std::basic_string_view<char, T1>, char>;

template <typename T1>
std::formatter<std::basic_string_view<char, T1>, char> f54() {
  return std::formatter<std::basic_string_view<char, T1>, char>();
}

template <typename T1>
const std::basic_string_view<char>::value_type *
f55(std::formatter<std::basic_string_view<char, T1>, char> &a0,
    std::basic_format_parse_context<char> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, char>::iterator
f56(const std::formatter<std::basic_string_view<char, T1>, char> &a0,
    std::basic_string_view<char, T1> a1,
    std::basic_format_context<T2, char> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t24 = std::formatter<std::basic_string_view<wchar_t, T1>, wchar_t>;

template <typename T1>
std::formatter<std::basic_string_view<wchar_t, T1>, wchar_t> f57() {
  return std::formatter<std::basic_string_view<wchar_t, T1>, wchar_t>();
}

template <typename T1>
const std::basic_string_view<wchar_t>::value_type *
f58(std::formatter<std::basic_string_view<wchar_t, T1>, wchar_t> &a0,
    std::basic_format_parse_context<wchar_t> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, wchar_t>::iterator
f59(const std::formatter<std::basic_string_view<wchar_t, T1>, wchar_t> &a0,
    std::basic_string_view<wchar_t, T1> a1,
    std::basic_format_context<T2, wchar_t> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t25 = std::formatter<std::nullptr_t, T1>;

template <typename T1> std::formatter<std::nullptr_t, T1> f60() {
  return std::formatter<std::nullptr_t, T1>();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f61(std::formatter<std::nullptr_t, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f62(const std::formatter<std::nullptr_t, T1> &a0, std::nullptr_t a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t26 = std::formatter<T1, T1>;

template <typename T1> std::formatter<T1, T1> f63() {
  return std::formatter<T1, T1>();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f64(std::formatter<T1, T1> &a0, std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f65(const std::formatter<T1, T1> &a0, T1 a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1, std::size_t T2> using t27 = std::formatter<T1[T2], T1>;

template <typename T1, typename T2>
using t28 = std::formatter<std::basic_string<char, T1, T2>, char>;

template <typename T1, typename T2>
std::formatter<std::basic_string<char, T1, T2>, char> f66() {
  return std::formatter<std::basic_string<char, T1, T2>, char>();
}

template <typename T1, typename T2>
const std::basic_string_view<char>::value_type *
f67(std::formatter<std::basic_string<char, T1, T2>, char> &a0,
    std::basic_format_parse_context<char> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, char>::iterator
f68(const std::formatter<std::basic_string<char, T1, T2>, char> &a0,
    const std::basic_string<char, T1, T2> &a1,
    std::basic_format_context<T3, char> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
using t29 = std::formatter<std::basic_string<wchar_t, T1, T2>, wchar_t>;

template <typename T1, typename T2>
std::formatter<std::basic_string<wchar_t, T1, T2>, wchar_t> f69() {
  return std::formatter<std::basic_string<wchar_t, T1, T2>, wchar_t>();
}

template <typename T1, typename T2>
const std::basic_string_view<wchar_t>::value_type *
f70(std::formatter<std::basic_string<wchar_t, T1, T2>, wchar_t> &a0,
    std::basic_format_parse_context<wchar_t> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, wchar_t>::iterator
f71(const std::formatter<std::basic_string<wchar_t, T1, T2>, wchar_t> &a0,
    const std::basic_string<wchar_t, T1, T2> &a1,
    std::basic_format_context<T3, wchar_t> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
using t30 = std::formatter<std::chrono::sys_time<T1>, T2>;

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f72(std::formatter<std::chrono::sys_time<T1>, T2> &a0,
    std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f73(const std::formatter<std::chrono::sys_time<T1>, T2> &a0,
    const std::chrono::sys_time<T1> &a1,
    std::basic_format_context<T3, T2> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
using t31 = std::formatter<std::chrono::gps_time<T1>, T2>;

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f74(std::formatter<std::chrono::gps_time<T1>, T2> &a0,
    std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f75(const std::formatter<std::chrono::gps_time<T1>, T2> &a0,
    const std::chrono::gps_time<T1> &a1,
    std::basic_format_context<T3, T2> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
using t32 = std::formatter<std::chrono::local_time<T1>, T2>;

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f76(std::formatter<std::chrono::local_time<T1>, T2> &a0,
    std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f77(const std::formatter<std::chrono::local_time<T1>, T2> &a0,
    const std::chrono::local_time<T1> &a1,
    std::basic_format_context<T3, T2> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
using t33 = std::formatter<std::chrono::tai_time<T1>, T2>;

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f78(std::formatter<std::chrono::tai_time<T1>, T2> &a0,
    std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f79(const std::formatter<std::chrono::tai_time<T1>, T2> &a0,
    const std::chrono::tai_time<T1> &a1,
    std::basic_format_context<T3, T2> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
using t34 = std::formatter<std::chrono::utc_time<T1>, T2>;

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f80(std::formatter<std::chrono::utc_time<T1>, T2> &a0,
    std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f81(const std::formatter<std::chrono::utc_time<T1>, T2> &a0,
    const std::chrono::utc_time<T1> &a1,
    std::basic_format_context<T3, T2> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2> using t35 = std::formatter<T1, T2>;

template <typename T1, typename T2> std::formatter<T1, T2> f82() {
  return std::formatter<T1, T2>();
}

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f83(std::formatter<T1, T2> &a0, std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f84(const std::formatter<T1, T2> &a0, T1 a1,
    std::basic_format_context<T3, T2> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3>
using t36 =
    std::formatter<std::chrono::hh_mm_ss<std::chrono::duration<T1, T2>>, T3>;

template <typename T1, typename T2, typename T3>
typename std::basic_format_parse_context<T3>::iterator
f85(std::formatter<std::chrono::hh_mm_ss<std::chrono::duration<T1, T2>>, T3>
        &a0,
    std::basic_format_parse_context<T3> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_format_context<T4, T3>::iterator
f86(const std::formatter<std::chrono::hh_mm_ss<std::chrono::duration<T1, T2>>,
                         T3> &a0,
    const std::chrono::hh_mm_ss<std::chrono::duration<T1, T2>> &a1,
    std::basic_format_context<T4, T3> &a2) {
  return a0.template format<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3>
using t37 = std::formatter<std::chrono::duration<T1, T2>, T3>;

template <typename T1, typename T2, typename T3>
typename std::basic_format_parse_context<T3>::iterator
f87(std::formatter<std::chrono::duration<T1, T2>, T3> &a0,
    std::basic_format_parse_context<T3> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_format_context<T4, T3>::iterator
f88(const std::formatter<std::chrono::duration<T1, T2>, T3> &a0,
    const std::chrono::duration<T1, T2> &a1,
    std::basic_format_context<T4, T3> &a2) {
  return a0.template format<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3>
using t38 = std::formatter<std::chrono::zoned_time<T1, T2>, T3>;

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_format_context<T4, T3>::iterator
f89(const std::formatter<std::chrono::zoned_time<T1, T2>, T3> &a0,
    const std::chrono::zoned_time<T1, T2> &a1,
    std::basic_format_context<T4, T3> &a2) {
  return a0.template format<T4>(a1, a2);
}
