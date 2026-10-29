#include <algorithm>
#include <barrier>
#include <chrono>
#include <format>
#include <queue>
#include <stack>
#include <stacktrace>
#include <string_view>
#include <thread>
#include <utility>

using t1 = std::formatter<std::stacktrace_entry>;

const std::basic_string_view<char>::value_type *
f1(std::formatter<std::stacktrace_entry> &a0,
   std::basic_format_parse_context<char> &a1) {
  return a0.parse(a1);
}

std::formatter<std::stacktrace_entry> f2() {
  return std::formatter<std::stacktrace_entry>();
}

std::formatter<std::stacktrace_entry>
f3(const std::formatter<std::stacktrace_entry> &a0) {
  return std::formatter<std::stacktrace_entry>(a0);
}

std::formatter<std::stacktrace_entry>
f4(std::formatter<std::stacktrace_entry> &&a0) {
  return std::formatter<std::stacktrace_entry>(std::move(a0));
}

std::formatter<std::stacktrace_entry> &
f5(std::formatter<std::stacktrace_entry> &a0,
   const std::formatter<std::stacktrace_entry> &a1) {
  return a0.operator=(a1);
}

std::formatter<std::stacktrace_entry> &
f6(std::formatter<std::stacktrace_entry> &a0,
   std::formatter<std::stacktrace_entry> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
typename std::basic_format_context<T1, char>::iterator
f7(const std::formatter<std::stacktrace_entry> &a0,
   const std::stacktrace_entry &a1, std::basic_format_context<T1, char> &a2) {
  return a0.template format<T1>(a1, a2);
}

using t2 = std::formatter<const char *, wchar_t>;

using t3 = std::formatter<char *, wchar_t>;

using t4 = std::formatter<char, wchar_t>;

std::formatter<char, wchar_t> f8() { return std::formatter<char, wchar_t>(); }

const std::basic_string_view<wchar_t>::value_type *
f9(std::formatter<char, wchar_t> &a0,
   std::basic_format_parse_context<wchar_t> &a1) {
  return a0.parse(a1);
}

void f10(std::formatter<char, wchar_t> &a0) { return a0.set_debug_format(); }

std::formatter<char, wchar_t> f11(const std::formatter<char, wchar_t> &a0) {
  return std::formatter<char, wchar_t>(a0);
}

std::formatter<char, wchar_t> f12(std::formatter<char, wchar_t> &&a0) {
  return std::formatter<char, wchar_t>(std::move(a0));
}

std::formatter<char, wchar_t> &f13(std::formatter<char, wchar_t> &a0,
                                   const std::formatter<char, wchar_t> &a1) {
  return a0.operator=(a1);
}

std::formatter<char, wchar_t> &f14(std::formatter<char, wchar_t> &a0,
                                   std::formatter<char, wchar_t> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
typename std::basic_format_context<T1, wchar_t>::iterator
f15(const std::formatter<char, wchar_t> &a0, char a1,
    std::basic_format_context<T1, wchar_t> &a2) {
  return a0.template format<T1>(a1, a2);
}

template <std::size_t T1> using t5 = std::formatter<char[T1], wchar_t>;

template <typename T1> using t6 = std::formatter<std::chrono::day, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f16(std::formatter<std::chrono::day, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f17(const std::formatter<std::chrono::day, T1> &a0, const std::chrono::day &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t7 = std::formatter<std::chrono::local_info, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f18(std::formatter<std::chrono::local_info, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f19(const std::formatter<std::chrono::local_info, T1> &a0,
    const std::chrono::local_info &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t8 = std::formatter<std::chrono::month, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f20(std::formatter<std::chrono::month, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f21(const std::formatter<std::chrono::month, T1> &a0,
    const std::chrono::month &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t9 = std::formatter<std::chrono::month_day, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f22(std::formatter<std::chrono::month_day, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f23(const std::formatter<std::chrono::month_day, T1> &a0,
    const std::chrono::month_day &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t10 = std::formatter<std::chrono::month_day_last, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f24(std::formatter<std::chrono::month_day_last, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f25(const std::formatter<std::chrono::month_day_last, T1> &a0,
    const std::chrono::month_day_last &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t11 = std::formatter<std::chrono::month_weekday, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f26(std::formatter<std::chrono::month_weekday, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f27(const std::formatter<std::chrono::month_weekday, T1> &a0,
    const std::chrono::month_weekday &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t12 = std::formatter<std::chrono::month_weekday_last, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f28(std::formatter<std::chrono::month_weekday_last, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f29(const std::formatter<std::chrono::month_weekday_last, T1> &a0,
    const std::chrono::month_weekday_last &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t13 = std::formatter<std::chrono::sys_info, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f30(std::formatter<std::chrono::sys_info, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f31(const std::formatter<std::chrono::sys_info, T1> &a0,
    const std::chrono::sys_info &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t14 = std::formatter<std::chrono::weekday, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f32(std::formatter<std::chrono::weekday, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f33(const std::formatter<std::chrono::weekday, T1> &a0,
    const std::chrono::weekday &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t15 = std::formatter<std::chrono::weekday_indexed, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f34(std::formatter<std::chrono::weekday_indexed, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f35(const std::formatter<std::chrono::weekday_indexed, T1> &a0,
    const std::chrono::weekday_indexed &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t16 = std::formatter<std::chrono::weekday_last, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f36(std::formatter<std::chrono::weekday_last, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f37(const std::formatter<std::chrono::weekday_last, T1> &a0,
    const std::chrono::weekday_last &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t17 = std::formatter<std::chrono::year, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f38(std::formatter<std::chrono::year, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f39(const std::formatter<std::chrono::year, T1> &a0,
    const std::chrono::year &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t18 = std::formatter<std::chrono::year_month, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f40(std::formatter<std::chrono::year_month, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f41(const std::formatter<std::chrono::year_month, T1> &a0,
    const std::chrono::year_month &a1, std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t19 = std::formatter<std::chrono::year_month_day, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f42(std::formatter<std::chrono::year_month_day, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f43(const std::formatter<std::chrono::year_month_day, T1> &a0,
    const std::chrono::year_month_day &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t20 = std::formatter<std::chrono::year_month_day_last, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f44(std::formatter<std::chrono::year_month_day_last, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f45(const std::formatter<std::chrono::year_month_day_last, T1> &a0,
    const std::chrono::year_month_day_last &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t21 = std::formatter<std::chrono::year_month_weekday, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f46(std::formatter<std::chrono::year_month_weekday, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f47(const std::formatter<std::chrono::year_month_weekday, T1> &a0,
    const std::chrono::year_month_weekday &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t22 = std::formatter<std::chrono::year_month_weekday_last, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f48(std::formatter<std::chrono::year_month_weekday_last, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f49(const std::formatter<std::chrono::year_month_weekday_last, T1> &a0,
    const std::chrono::year_month_weekday_last &a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
  requires(std::is_pointer_v<std::thread::native_handle_type> ||
           std::is_integral_v<std::thread::native_handle_type>)
using t23 = std::formatter<std::thread::id, T1>;

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f50(std::formatter<std::thread::id, T1> &a0,
    std::basic_format_parse_context<T1> &a1)
  requires(std::is_pointer_v<std::thread::native_handle_type> ||
           std::is_integral_v<std::thread::native_handle_type>)
{
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f51(const std::formatter<std::thread::id, T1> &a0, std::thread::id a1,
    std::basic_format_context<T2, T1> &a2)
  requires(std::is_pointer_v<std::thread::native_handle_type> ||
           std::is_integral_v<std::thread::native_handle_type>)
{
  return a0.template format<T2>(std::move(a1), a2);
}

template <typename T1> using t24 = std::formatter<const T1 *, T1>;

template <typename T1> std::formatter<const T1 *, T1> f52() {
  return std::formatter<const T1 *, T1>();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f53(std::formatter<const T1 *, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1> void f54(std::formatter<const T1 *, T1> &a0) {
  return a0.set_debug_format();
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f55(const std::formatter<const T1 *, T1> &a0, const T1 *a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t25 = std::formatter<const void *, T1>;

template <typename T1> std::formatter<const void *, T1> f56() {
  return std::formatter<const void *, T1>();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f57(std::formatter<const void *, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f58(const std::formatter<const void *, T1> &a0, const void *a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t26 = std::formatter<T1 *, T1>;

template <typename T1> std::formatter<T1 *, T1> f59() {
  return std::formatter<T1 *, T1>();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f60(std::formatter<T1 *, T1> &a0, std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1> void f61(std::formatter<T1 *, T1> &a0) {
  return a0.set_debug_format();
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f62(const std::formatter<T1 *, T1> &a0, T1 *a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t27 = std::formatter<void *, T1>;

template <typename T1> std::formatter<void *, T1> f63() {
  return std::formatter<void *, T1>();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f64(std::formatter<void *, T1> &a0, std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f65(const std::formatter<void *, T1> &a0, void *a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t28 = std::formatter<std::basic_stacktrace<T1>>;

template <typename T1>
const std::basic_string_view<char>::value_type *
f66(std::formatter<std::basic_stacktrace<T1>> &a0,
    std::basic_format_parse_context<char> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, char>::iterator
f67(const std::formatter<std::basic_stacktrace<T1>> &a0,
    const std::basic_stacktrace<T1> &a1,
    std::basic_format_context<T2, char> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1>
using t29 = std::formatter<std::basic_string_view<char, T1>, char>;

template <typename T1>
std::formatter<std::basic_string_view<char, T1>, char> f68() {
  return std::formatter<std::basic_string_view<char, T1>, char>();
}

template <typename T1>
const std::basic_string_view<char>::value_type *
f69(std::formatter<std::basic_string_view<char, T1>, char> &a0,
    std::basic_format_parse_context<char> &a1) {
  return a0.parse(a1);
}

template <typename T1>
void f70(std::formatter<std::basic_string_view<char, T1>, char> &a0) {
  return a0.set_debug_format();
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, char>::iterator
f71(const std::formatter<std::basic_string_view<char, T1>, char> &a0,
    std::basic_string_view<char, T1> a1,
    std::basic_format_context<T2, char> &a2) {
  return a0.template format<T2>(
      std::forward<std::basic_string_view<char, T1>>(a1), a2);
}

template <typename T1>
using t30 = std::formatter<std::basic_string_view<char, T1>, wchar_t>;

template <typename T1>
using t31 = std::formatter<std::basic_string_view<wchar_t, T1>, wchar_t>;

template <typename T1>
std::formatter<std::basic_string_view<wchar_t, T1>, wchar_t> f72() {
  return std::formatter<std::basic_string_view<wchar_t, T1>, wchar_t>();
}

template <typename T1>
const std::basic_string_view<wchar_t>::value_type *
f73(std::formatter<std::basic_string_view<wchar_t, T1>, wchar_t> &a0,
    std::basic_format_parse_context<wchar_t> &a1) {
  return a0.parse(a1);
}

template <typename T1>
void f74(std::formatter<std::basic_string_view<wchar_t, T1>, wchar_t> &a0) {
  return a0.set_debug_format();
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, wchar_t>::iterator
f75(const std::formatter<std::basic_string_view<wchar_t, T1>, wchar_t> &a0,
    std::basic_string_view<wchar_t, T1> a1,
    std::basic_format_context<T2, wchar_t> &a2) {
  return a0.template format<T2>(
      std::forward<std::basic_string_view<wchar_t, T1>>(a1), a2);
}

template <typename T1> using t32 = std::formatter<std::nullptr_t, T1>;

template <typename T1> std::formatter<std::nullptr_t, T1> f76() {
  return std::formatter<std::nullptr_t, T1>();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f77(std::formatter<std::nullptr_t, T1> &a0,
    std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f78(const std::formatter<std::nullptr_t, T1> &a0, std::nullptr_t a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(a1, a2);
}

template <typename T1> using t33 = std::formatter<T1, T1>;

template <typename T1> std::formatter<T1, T1> f79() {
  return std::formatter<T1, T1>();
}

template <typename T1>
typename std::basic_format_parse_context<T1>::iterator
f80(std::formatter<T1, T1> &a0, std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1> void f81(std::formatter<T1, T1> &a0) {
  return a0.set_debug_format();
}

template <typename T1, typename T2>
typename std::basic_format_context<T2, T1>::iterator
f82(const std::formatter<T1, T1> &a0, T1 a1,
    std::basic_format_context<T2, T1> &a2) {
  return a0.template format<T2>(std::move(a1), a2);
}

template <typename T1, std::size_t T2> using t34 = std::formatter<T1[T2], T1>;

template <typename T1, typename T2>
using t35 = std::formatter<std::basic_string<char, T1, T2>, char>;

template <typename T1, typename T2>
std::formatter<std::basic_string<char, T1, T2>, char> f83() {
  return std::formatter<std::basic_string<char, T1, T2>, char>();
}

template <typename T1, typename T2>
const std::basic_string_view<char>::value_type *
f84(std::formatter<std::basic_string<char, T1, T2>, char> &a0,
    std::basic_format_parse_context<char> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
void f85(std::formatter<std::basic_string<char, T1, T2>, char> &a0) {
  return a0.set_debug_format();
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, char>::iterator
f86(const std::formatter<std::basic_string<char, T1, T2>, char> &a0,
    const std::basic_string<char, T1, T2> &a1,
    std::basic_format_context<T3, char> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
using t36 = std::formatter<std::basic_string<char, T1, T2>, wchar_t>;

template <typename T1, typename T2>
using t37 = std::formatter<std::basic_string<wchar_t, T1, T2>, wchar_t>;

template <typename T1, typename T2>
std::formatter<std::basic_string<wchar_t, T1, T2>, wchar_t> f87() {
  return std::formatter<std::basic_string<wchar_t, T1, T2>, wchar_t>();
}

template <typename T1, typename T2>
const std::basic_string_view<wchar_t>::value_type *
f88(std::formatter<std::basic_string<wchar_t, T1, T2>, wchar_t> &a0,
    std::basic_format_parse_context<wchar_t> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2>
void f89(std::formatter<std::basic_string<wchar_t, T1, T2>, wchar_t> &a0) {
  return a0.set_debug_format();
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, wchar_t>::iterator
f90(const std::formatter<std::basic_string<wchar_t, T1, T2>, wchar_t> &a0,
    const std::basic_string<wchar_t, T1, T2> &a1,
    std::basic_format_context<T3, wchar_t> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
using t38 = std::formatter<std::chrono::sys_time<T1>, T2>;

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f91(std::formatter<std::chrono::sys_time<T1>, T2> &a0,
    std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f92(const std::formatter<std::chrono::sys_time<T1>, T2> &a0,
    const std::chrono::sys_time<T1> &a1,
    std::basic_format_context<T3, T2> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
using t39 = std::formatter<std::chrono::gps_time<T1>, T2>;

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f93(std::formatter<std::chrono::gps_time<T1>, T2> &a0,
    std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f94(const std::formatter<std::chrono::gps_time<T1>, T2> &a0,
    const std::chrono::gps_time<T1> &a1,
    std::basic_format_context<T3, T2> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
using t40 = std::formatter<std::chrono::local_time<T1>, T2>;

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f95(std::formatter<std::chrono::local_time<T1>, T2> &a0,
    std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f96(const std::formatter<std::chrono::local_time<T1>, T2> &a0,
    const std::chrono::local_time<T1> &a1,
    std::basic_format_context<T3, T2> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
using t41 = std::formatter<std::chrono::tai_time<T1>, T2>;

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f97(std::formatter<std::chrono::tai_time<T1>, T2> &a0,
    std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f98(const std::formatter<std::chrono::tai_time<T1>, T2> &a0,
    const std::chrono::tai_time<T1> &a1,
    std::basic_format_context<T3, T2> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2>
using t42 = std::formatter<std::chrono::utc_time<T1>, T2>;

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f99(std::formatter<std::chrono::utc_time<T1>, T2> &a0,
    std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f100(const std::formatter<std::chrono::utc_time<T1>, T2> &a0,
     const std::chrono::utc_time<T1> &a1,
     std::basic_format_context<T3, T2> &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2> using t43 = std::formatter<T1, T2>;

template <typename T1, typename T2> std::formatter<T1, T2> f101() {
  return std::formatter<T1, T2>();
}

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f102(std::formatter<T1, T2> &a0, std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_context<T3, T2>::iterator
f103(const std::formatter<T1, T2> &a0, T1 a1,
     std::basic_format_context<T3, T2> &a2) {
  return a0.template format<T3>(std::move(a1), a2);
}

template <typename T1, typename T2>
  requires((std::format_kind<T1> != std::range_format::disabled) &&
           std::formattable<std::ranges::range_reference_t<T1>, T2>)
using t44 = std::formatter<T1, T2>;

template <typename T1, typename T2>
std::formatter<T1, T2> f104()
  requires((std::format_kind<T1> != std::range_format::disabled) &&
           std::formattable<std::ranges::range_reference_t<T1>, T2>)
{
  return std::formatter<T1, T2>();
}

template <typename T1, typename T2>
void f105(std::formatter<T1, T2> &a0, std::basic_string_view<T2> a1)
  requires((std::format_kind<T1> == std::range_format::sequence)) &&
          ((std::format_kind<T1> != std::range_format::disabled) &&
           std::formattable<std::ranges::range_reference_t<T1>, T2>)
{
  return a0.set_separator(std::forward<std::basic_string_view<T2>>(a1));
}

template <typename T1, typename T2>
void f106(std::formatter<T1, T2> &a0, std::basic_string_view<T2> a1,
          std::basic_string_view<T2> a2)
  requires((std::format_kind<T1> == std::range_format::sequence)) &&
          ((std::format_kind<T1> != std::range_format::disabled) &&
           std::formattable<std::ranges::range_reference_t<T1>, T2>)
{
  return a0.set_brackets(std::forward<std::basic_string_view<T2>>(a1),
                         std::forward<std::basic_string_view<T2>>(a2));
}

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f107(std::formatter<T1, T2> &a0, std::basic_format_parse_context<T2> &a1)
  requires((std::format_kind<T1> != std::range_format::disabled) &&
           std::formattable<std::ranges::range_reference_t<T1>, T2>)
{
  return a0.parse(a1);
}

template <typename T1, typename... T2>
using t45 = std::formatter<std::tuple<T2...>, T1>;

template <typename T1, typename... T2>
void f108(std::formatter<std::tuple<T2...>, T1> &a0,
          std::basic_string_view<T1> a1) {
  return a0.set_separator(std::forward<std::basic_string_view<T1>>(a1));
}

template <typename T1, typename... T2>
void f109(std::formatter<std::tuple<T2...>, T1> &a0,
          std::basic_string_view<T1> a1, std::basic_string_view<T1> a2) {
  return a0.set_brackets(std::forward<std::basic_string_view<T1>>(a1),
                         std::forward<std::basic_string_view<T1>>(a2));
}

template <typename T1, typename... T2>
typename std::basic_format_parse_context<T1>::iterator
f110(std::formatter<std::tuple<T2...>, T1> &a0,
     std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
using t46 =
    std::formatter<std::chrono::hh_mm_ss<std::chrono::duration<T1, T2>>, T3>;

template <typename T1, typename T2, typename T3>
typename std::basic_format_parse_context<T3>::iterator
f111(std::formatter<std::chrono::hh_mm_ss<std::chrono::duration<T1, T2>>, T3>
         &a0,
     std::basic_format_parse_context<T3> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_format_context<T4, T3>::iterator
f112(const std::formatter<std::chrono::hh_mm_ss<std::chrono::duration<T1, T2>>,
                          T3> &a0,
     const std::chrono::hh_mm_ss<std::chrono::duration<T1, T2>> &a1,
     std::basic_format_context<T4, T3> &a2) {
  return a0.template format<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3>
using t47 = std::formatter<std::chrono::duration<T1, T2>, T3>;

template <typename T1, typename T2, typename T3>
typename std::basic_format_parse_context<T3>::iterator
f113(std::formatter<std::chrono::duration<T1, T2>, T3> &a0,
     std::basic_format_parse_context<T3> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_format_context<T4, T3>::iterator
f114(const std::formatter<std::chrono::duration<T1, T2>, T3> &a0,
     const std::chrono::duration<T1, T2> &a1,
     std::basic_format_context<T4, T3> &a2) {
  return a0.template format<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3>
using t48 = std::formatter<std::chrono::zoned_time<T1, T2>, T3>;

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_format_context<T4, T3>::iterator
f115(const std::formatter<std::chrono::zoned_time<T1, T2>, T3> &a0,
     const std::chrono::zoned_time<T1, T2> &a1,
     std::basic_format_context<T4, T3> &a2) {
  return a0.template format<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3>
using t49 = std::formatter<std::pair<T2, T3>, T1>;

template <typename T1, typename T2, typename T3>
void f116(std::formatter<std::pair<T2, T3>, T1> &a0,
          std::basic_string_view<T1> a1) {
  return a0.set_separator(std::forward<std::basic_string_view<T1>>(a1));
}

template <typename T1, typename T2, typename T3>
void f117(std::formatter<std::pair<T2, T3>, T1> &a0,
          std::basic_string_view<T1> a1, std::basic_string_view<T1> a2) {
  return a0.set_brackets(std::forward<std::basic_string_view<T1>>(a1),
                         std::forward<std::basic_string_view<T1>>(a2));
}

template <typename T1, typename T2, typename T3>
typename std::basic_format_parse_context<T1>::iterator
f118(std::formatter<std::pair<T2, T3>, T1> &a0,
     std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
using t50 = std::formatter<std::queue<T2, T3>, T1>;

template <typename T1, typename T2, typename T3>
typename std::basic_format_parse_context<T1>::iterator
f119(std::formatter<std::queue<T2, T3>, T1> &a0,
     std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3>
using t51 = std::formatter<std::stack<T2, T3>, T1>;

template <typename T1, typename T2, typename T3>
typename std::basic_format_parse_context<T1>::iterator
f120(std::formatter<std::stack<T2, T3>, T1> &a0,
     std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
using t52 = std::formatter<std::priority_queue<T2, T3, T4>, T1>;

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_format_parse_context<T1>::iterator
f121(std::formatter<std::priority_queue<T2, T3, T4>, T1> &a0,
     std::basic_format_parse_context<T1> &a1) {
  return a0.parse(a1);
}
