#include <cstddef>
#include <format>
#include <ios>
#include <iterator>
#include <string.h>
#include <string_view>
#include <utility>

template <typename... T1>
std::string f1(std::locale a0, std::format_string<T1...> a1, T1 &&...a2) {
  return std::format<T1...>(std::move(a0), a1, std::forward<T1>(a2)...);
}

template <typename... T1>
std::wstring f2(std::locale a0, std::wformat_string<T1...> a1, T1 &&...a2) {
  return std::format<T1...>(std::move(a0), a1, std::forward<T1>(a2)...);
}

template <typename... T1>
std::string f3(std::format_string<T1...> a0, T1 &&...a1) {
  return std::format<T1...>(a0, std::forward<T1>(a1)...);
}

template <typename... T1>
std::wstring f4(std::wformat_string<T1...> a0, T1 &&...a1) {
  return std::format<T1...>(a0, std::forward<T1>(a1)...);
}

using t1 = std::format_args;

template <typename... T1> using t2 = std::format_string<T1...>;

template <typename T1, typename... T2>
T1 f5(T1 a0, std::locale a1, std::format_string<T2...> a2, T2 &&...a3)
  requires(std::output_iterator<T1, const char &>)
{
  return std::format_to<T1, T2...>(a0, std::move(a1), a2,
                                   std::forward<T2>(a3)...);
}

template <typename T1, typename... T2>
T1 f6(T1 a0, std::locale a1, std::wformat_string<T2...> a2, T2 &&...a3)
  requires(std::output_iterator<T1, const wchar_t &>)
{
  return std::format_to<T1, T2...>(a0, std::move(a1), a2,
                                   std::forward<T2>(a3)...);
}

template <typename T1, typename... T2>
T1 f7(T1 a0, std::format_string<T2...> a1, T2 &&...a2)
  requires(std::output_iterator<T1, const char &>)
{
  return std::format_to<T1, T2...>(a0, a1, std::forward<T2>(a2)...);
}

template <typename T1, typename... T2>
T1 f8(T1 a0, std::wformat_string<T2...> a1, T2 &&...a2)
  requires(std::output_iterator<T1, const wchar_t &>)
{
  return std::format_to<T1, T2...>(a0, a1, std::forward<T2>(a2)...);
}

template <typename T1, typename... T2>
std::format_to_n_result<T1> f9(T1 a0, std::iter_difference_t<T1> a1,
                               std::locale a2, std::format_string<T2...> a3,
                               T2 &&...a4)
  requires(std::output_iterator<T1, const char &>)
{
  return std::format_to_n<T1, T2...>(a0, a1, std::move(a2), a3,
                                     std::forward<T2>(a4)...);
}

template <typename T1, typename... T2>
std::format_to_n_result<T1> f10(T1 a0, std::iter_difference_t<T1> a1,
                                std::locale a2, std::wformat_string<T2...> a3,
                                T2 &&...a4)
  requires(std::output_iterator<T1, const wchar_t &>)
{
  return std::format_to_n<T1, T2...>(a0, a1, std::move(a2), a3,
                                     std::forward<T2>(a4)...);
}

template <typename T1, typename... T2>
std::format_to_n_result<T1> f11(T1 a0, std::iter_difference_t<T1> a1,
                                std::format_string<T2...> a2, T2 &&...a3)
  requires(std::output_iterator<T1, const char &>)
{
  return std::format_to_n<T1, T2...>(a0, a1, a2, std::forward<T2>(a3)...);
}

template <typename T1, typename... T2>
std::format_to_n_result<T1> f12(T1 a0, std::iter_difference_t<T1> a1,
                                std::wformat_string<T2...> a2, T2 &&...a3)
  requires(std::output_iterator<T1, const wchar_t &>)
{
  return std::format_to_n<T1, T2...>(a0, a1, a2, std::forward<T2>(a3)...);
}

template <typename... T1>
std::size_t f13(std::locale a0, std::format_string<T1...> a1, T1 &&...a2) {
  return std::formatted_size<T1...>(std::move(a0), a1, std::forward<T1>(a2)...);
}

template <typename... T1>
std::size_t f14(std::locale a0, std::wformat_string<T1...> a1, T1 &&...a2) {
  return std::formatted_size<T1...>(std::move(a0), a1, std::forward<T1>(a2)...);
}

template <typename... T1>
std::size_t f15(std::format_string<T1...> a0, T1 &&...a1) {
  return std::formatted_size<T1...>(a0, std::forward<T1>(a1)...);
}

template <typename... T1>
std::size_t f16(std::wformat_string<T1...> a0, T1 &&...a1) {
  return std::formatted_size<T1...>(a0, std::forward<T1>(a1)...);
}

template <typename T1, typename... T2>
std::__format_arg_store<T1, T2...> f17(T2 &...a0) {
  return std::make_format_args<T1, T2...>(a0...);
}

template <typename... T1>
std::__format_arg_store<std::wformat_context, T1...> f18(T1 &...a0) {
  return std::make_wformat_args<T1...>(a0...);
}

template <typename T1 = void>
std::string f19(std::string_view a0, std::format_args a1) {
  return std::vformat(std::move(a0), std::move(a1));
}

template <typename T1 = void>
std::wstring f20(std::wstring_view a0, std::wformat_args a1) {
  return std::vformat(std::move(a0), std::move(a1));
}

template <typename T1 = void>
std::string f21(std::locale a0, std::string_view a1, std::format_args a2) {
  return std::vformat(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1 = void>
std::wstring f22(std::locale a0, std::wstring_view a1, std::wformat_args a2) {
  return std::vformat(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1>
T1 f23(T1 a0, std::string_view a1, std::format_args a2)
  requires(std::output_iterator<T1, const char &>)
{
  return std::vformat_to<T1>(a0, std::move(a1), std::move(a2));
}

template <typename T1>
T1 f24(T1 a0, std::wstring_view a1, std::wformat_args a2)
  requires(std::output_iterator<T1, const wchar_t &>)
{
  return std::vformat_to<T1>(a0, std::move(a1), std::move(a2));
}

template <typename T1>
T1 f25(T1 a0, std::locale a1, std::string_view a2, std::format_args a3)
  requires(std::output_iterator<T1, const char &>)
{
  return std::vformat_to<T1>(a0, std::move(a1), std::move(a2), std::move(a3));
}

template <typename T1>
T1 f26(T1 a0, std::locale a1, std::wstring_view a2, std::wformat_args a3)
  requires(std::output_iterator<T1, const wchar_t &>)
{
  return std::vformat_to<T1>(a0, std::move(a1), std::move(a2), std::move(a3));
}

using t3 = std::wformat_args;

template <typename... T1> using t4 = std::wformat_string<T1...>;
