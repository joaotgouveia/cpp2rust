#include <algorithm>
#include <chrono>
#include <format>
#include <string>
#include <string_view>
#include <utility>

const std::basic_string_view<wchar_t>::value_type *
f10(std::formatter<char, wchar_t> &a0,
    std::basic_format_parse_context<wchar_t> &a1) {
  return a0.parse(a1);
}

template <typename T1>
typename std::basic_format_context<T1, wchar_t>::iterator
f11(const std::formatter<char, wchar_t> &a0, char a1,
    std::basic_format_context<T1, wchar_t> &a2) {
  return a0.template format<T1>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f12(std::formatter<std::chrono::day, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f13(const std::formatter<std::chrono::day, T1> &a0, const std::chrono::day &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t29 = std::formatter<std::chrono::local_info, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f14(std::formatter<std::chrono::local_info, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f15(const std::formatter<std::chrono::local_info, T1> &a0,
    const std::chrono::local_info &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f16(std::formatter<std::chrono::month, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f17(const std::formatter<std::chrono::month, T1> &a0,
    const std::chrono::month &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f18(std::formatter<std::chrono::month_day, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f19(const std::formatter<std::chrono::month_day, T1> &a0,
    const std::chrono::month_day &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f20(std::formatter<std::chrono::month_day_last, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f21(const std::formatter<std::chrono::month_day_last, T1> &a0,
    const std::chrono::month_day_last &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f22(std::formatter<std::chrono::month_weekday, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f23(const std::formatter<std::chrono::month_weekday, T1> &a0,
    const std::chrono::month_weekday &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f24(std::formatter<std::chrono::month_weekday_last, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f25(const std::formatter<std::chrono::month_weekday_last, T1> &a0,
    const std::chrono::month_weekday_last &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t30 = std::formatter<std::chrono::sys_info, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f26(std::formatter<std::chrono::sys_info, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f27(const std::formatter<std::chrono::sys_info, T1> &a0,
    const std::chrono::sys_info &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f28(std::formatter<std::chrono::weekday, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f29(const std::formatter<std::chrono::weekday, T1> &a0,
    const std::chrono::weekday &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f30(std::formatter<std::chrono::weekday_indexed, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f31(const std::formatter<std::chrono::weekday_indexed, T1> &a0,
    const std::chrono::weekday_indexed &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f32(std::formatter<std::chrono::weekday_last, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f33(const std::formatter<std::chrono::weekday_last, T1> &a0,
    const std::chrono::weekday_last &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f34(std::formatter<std::chrono::year, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f35(const std::formatter<std::chrono::year, T1> &a0,
    const std::chrono::year &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f36(std::formatter<std::chrono::year_month, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f37(const std::formatter<std::chrono::year_month, T1> &a0,
    const std::chrono::year_month &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f38(std::formatter<std::chrono::year_month_day, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f39(const std::formatter<std::chrono::year_month_day, T1> &a0,
    const std::chrono::year_month_day &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f40(std::formatter<std::chrono::year_month_day_last, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f41(const std::formatter<std::chrono::year_month_day_last, T1> &a0,
    const std::chrono::year_month_day_last &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f42(std::formatter<std::chrono::year_month_weekday, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f43(const std::formatter<std::chrono::year_month_weekday, T1> &a0,
    const std::chrono::year_month_weekday &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f44(std::formatter<std::chrono::year_month_weekday_last, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f45(const std::formatter<std::chrono::year_month_weekday_last, T1> &a0,
    const std::chrono::year_month_weekday_last &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> std::formatter<const T1 *, T1> f46() {
  return std::formatter<const T1 *, T1>();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f47(std::formatter<const T1 *, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f48(const std::formatter<const T1 *, T1> &a0, const T1 *a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> std::formatter<const void *, T1> f49() {
  return std::formatter<const void *, T1>();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f50(std::formatter<const void *, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f51(const std::formatter<const void *, T1> &a0, const void *a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> std::formatter<T1 *, T1> f52() {
  return std::formatter<T1 *, T1>();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f53(std::formatter<T1 *, T1> &a0, std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f54(const std::formatter<T1 *, T1> &a0, T1 *a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> std::formatter<void *, T1> f55() {
  return std::formatter<void *, T1>();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f56(std::formatter<void *, T1> &a0, std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f57(const std::formatter<void *, T1> &a0, void *a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t31 = std::formatter<std::basic_string_view<char, T1>, char>;

template <typename T1>
std::formatter<std::basic_string_view<char, T1>, char> f58() {
  return std::formatter<std::basic_string_view<char, T1>, char>();
}

template <typename T1>
const std::basic_string_view<char>::value_type *
f59(std::formatter<std::basic_string_view<char, T1>, char> &a0,
    std::basic_format_parse_context<char> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, char>::iterator
f60(const std::formatter<std::basic_string_view<char, T1>, char> &a0,
    std::basic_string_view<char, T1> a1,
    std::basic_format_context<T2, char> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t32 = std::formatter<std::basic_string_view<wchar_t, T1>, wchar_t>;

template <typename T1>
std::formatter<std::basic_string_view<wchar_t, T1>, wchar_t> f61() {
  return std::formatter<std::basic_string_view<wchar_t, T1>, wchar_t>();
}

template <typename T1>
const std::basic_string_view<wchar_t>::value_type *
f62(std::formatter<std::basic_string_view<wchar_t, T1>, wchar_t> &a0,
    std::basic_format_parse_context<wchar_t> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, wchar_t>::iterator
f63(const std::formatter<std::basic_string_view<wchar_t, T1>, wchar_t> &a0,
    std::basic_string_view<wchar_t, T1> a1,
    std::basic_format_context<T2, wchar_t> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> std::formatter<std::nullptr_t, T1> f64() {
  return std::formatter<std::nullptr_t, T1>();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f65(std::formatter<std::nullptr_t, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f66(const std::formatter<std::nullptr_t, T1> &a0, std::nullptr_t a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t33 = std::formatter<T1, T1>;

template <typename T1> std::formatter<T1, T1> f67() {
  return std::formatter<T1, T1>();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f68(std::formatter<T1, T1> &a0, std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f69(const std::formatter<T1, T1> &a0, T1 a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1, typename T2>
using t34 = std::formatter<std::basic_string<char, T1, T2>, char>;

template <typename T1, typename T2>
std::formatter<std::basic_string<char, T1, T2>, char> f70() {
  return std::formatter<std::basic_string<char, T1, T2>, char>();
}

template <typename T1, typename T2>
const std::basic_string_view<char>::value_type *
f71(std::formatter<std::basic_string<char, T1, T2>, char> &a0,
    std::basic_format_parse_context<char> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, char>::iterator
f72(const std::formatter<std::basic_string<char, T1, T2>, char> &a0,
    const std::basic_string<char, T1, T2> &a1,
    std::basic_format_context<T3, char> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
using t35 = std::formatter<std::basic_string<wchar_t, T1, T2>, wchar_t>;

template <typename T1, typename T2>
std::formatter<std::basic_string<wchar_t, T1, T2>, wchar_t> f73() {
  return std::formatter<std::basic_string<wchar_t, T1, T2>, wchar_t>();
}

template <typename T1, typename T2>
const std::basic_string_view<wchar_t>::value_type *
f74(std::formatter<std::basic_string<wchar_t, T1, T2>, wchar_t> &a0,
    std::basic_format_parse_context<wchar_t> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, wchar_t>::iterator
f75(const std::formatter<std::basic_string<wchar_t, T1, T2>, wchar_t> &a0,
    const std::basic_string<wchar_t, T1, T2> &a1,
    std::basic_format_context<T3, wchar_t> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f76(std::formatter<std::chrono::sys_time<T1>, T2> &a0,
    std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f77(const std::formatter<std::chrono::sys_time<T1>, T2> &a0,
    const std::chrono::sys_time<T1> &a1,
    std::basic_format_context<T3, T2> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
using t36 = std::formatter<std::chrono::gps_time<T1>, T2>;

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f78(std::formatter<std::chrono::gps_time<T1>, T2> &a0,
    std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f79(const std::formatter<std::chrono::gps_time<T1>, T2> &a0,
    const std::chrono::gps_time<T1> &a1,
    std::basic_format_context<T3, T2> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f80(std::formatter<std::chrono::local_time<T1>, T2> &a0,
    std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f81(const std::formatter<std::chrono::local_time<T1>, T2> &a0,
    const std::chrono::local_time<T1> &a1,
    std::basic_format_context<T3, T2> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
using t37 = std::formatter<std::chrono::tai_time<T1>, T2>;

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f82(std::formatter<std::chrono::tai_time<T1>, T2> &a0,
    std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f83(const std::formatter<std::chrono::tai_time<T1>, T2> &a0,
    const std::chrono::tai_time<T1> &a1,
    std::basic_format_context<T3, T2> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
using t38 = std::formatter<std::chrono::utc_time<T1>, T2>;

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f84(std::formatter<std::chrono::utc_time<T1>, T2> &a0,
    std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f85(const std::formatter<std::chrono::utc_time<T1>, T2> &a0,
    const std::chrono::utc_time<T1> &a1,
    std::basic_format_context<T3, T2> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2> std::formatter<T1, T2> f86() {
  return std::formatter<T1, T2>();
}

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f87(std::formatter<T1, T2> &a0, std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f88(const std::formatter<T1, T2> &a0, T1 a1,
    std::basic_format_context<T3, T2> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
  requires(std::__format::__is_formattable_integer<T1>)
using t39 = std::formatter<T1, T2>;

template <typename T1, typename T2>
std::formatter<T1, T2> f89()
  requires(std::__format::__is_formattable_integer<T1>)
{
  return std::formatter<T1, T2>();
}

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f90(std::formatter<T1, T2> &a0, std::basic_format_parse_context<T2> &a1)
  requires(std::__format::__is_formattable_integer<T1>)
{
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f91(const std::formatter<T1, T2> &a0, T1 a1,
    std::basic_format_context<T3, T2> &a2)
  requires(std::__format::__is_formattable_integer<T1>)
{
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3>
using t40 =
    std::formatter<std::chrono::hh_mm_ss<std::chrono::duration<T1, T2>>, T3>;

template <typename T1, typename T2, typename T3>
typename std::basic_format_parse_context<T3>::iterator
f92(std::formatter<std::chrono::hh_mm_ss<std::chrono::duration<T1, T2>>, T3>
        &a0,
    std::basic_format_parse_context<T3> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_format_context<T4, T3>::iterator
f93(const std::formatter<std::chrono::hh_mm_ss<std::chrono::duration<T1, T2>>,
                         T3> &a0,
    const std::chrono::hh_mm_ss<std::chrono::duration<T1, T2>> &a1,
    std::basic_format_context<T4, T3> &a2) {
  return a0.template format<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3>
  requires(std::__format::__formattable_impl<T1, T3>)
using t41 = std::formatter<std::chrono::duration<T1, T2>, T3>;

template <typename T1, typename T2, typename T3>
typename std::basic_format_parse_context<T3>::iterator
f94(std::formatter<std::chrono::duration<T1, T2>, T3> &a0,
    std::basic_format_parse_context<T3> &a1)
  requires(std::__format::__formattable_impl<T1, T3>)
{
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_format_context<T4, T3>::iterator
f95(const std::formatter<std::chrono::duration<T1, T2>, T3> &a0,
    const std::chrono::duration<T1, T2> &a1,
    std::basic_format_context<T4, T3> &a2)
  requires(std::__format::__formattable_impl<T1, T3>)
{
  return a0.template format<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3>
using t42 = std::formatter<std::chrono::zoned_time<T1, T2>, T3>;

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_format_context<T4, T3>::iterator
f96(const std::formatter<std::chrono::zoned_time<T1, T2>, T3> &a0,
    const std::chrono::zoned_time<T1, T2> &a1,
    std::basic_format_context<T4, T3> &a2) {
  return a0.template format<T4>(a1, a2);
}
