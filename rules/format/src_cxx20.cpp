#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <format>
#include <ios>
#include <iosfwd>
#include <utility>

template <typename... T1>
std::string f1(const std::locale &a0, std::format_string<T1...> a1,
               T1 &&...a2) {
  return std::format<T1...>(a0, a1, std::forward<T1>(a2)...);
}

template <typename... T1>
std::wstring f2(const std::locale &a0, std::wformat_string<T1...> a1,
                T1 &&...a2) {
  return std::format<T1...>(a0, a1, std::forward<T1>(a2)...);
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

using t2 = std::format_context;

using t3 = std::format_error;

std::format_error f5(const std::string &a0) { return std::format_error(a0); }

std::format_error f6(const char *a0) { return std::format_error(a0); }

std::format_error f7(const std::format_error &a0) {
  return std::format_error(a0);
}

std::format_error f8(std::format_error &&a0) {
  return std::format_error(std::move(a0));
}

std::format_error &f9(std::format_error &a0, const std::format_error &a1) {
  return a0.operator=(a1);
}

std::format_error &f10(std::format_error &a0, std::format_error &&a1) {
  return a0.operator=(std::move(a1));
}

const char *f11(const std::format_error &a0) { return a0.what(); }

using t4 = std::format_parse_context;

template <typename... T1> using t5 = std::format_string<T1...>;

template <typename T1, typename... T2>
T1 f12(T1 a0, const std::locale &a1, std::format_string<T2...> a2, T2 &&...a3) {
  return std::format_to<T1, T2...>(a0, a1, a2, std::forward<T2>(a3)...);
}

template <typename T1, typename... T2>
T1 f13(T1 a0, const std::locale &a1, std::wformat_string<T2...> a2,
       T2 &&...a3) {
  return std::format_to<T1, T2...>(a0, a1, a2, std::forward<T2>(a3)...);
}

template <typename T1, typename... T2>
T1 f14(T1 a0, std::format_string<T2...> a1, T2 &&...a2) {
  return std::format_to<T1, T2...>(a0, a1, std::forward<T2>(a2)...);
}

template <typename T1, typename... T2>
T1 f15(T1 a0, std::wformat_string<T2...> a1, T2 &&...a2) {
  return std::format_to<T1, T2...>(a0, a1, std::forward<T2>(a2)...);
}

template <typename T1, typename... T2>
std::format_to_n_result<T1> f16(T1 a0, std::iter_difference_t<T1> a1,
                                const std::locale &a2,
                                std::format_string<T2...> a3, T2 &&...a4) {
  return std::format_to_n<T1, T2...>(a0, a1, a2, a3, std::forward<T2>(a4)...);
}

template <typename T1, typename... T2>
std::format_to_n_result<T1> f17(T1 a0, std::iter_difference_t<T1> a1,
                                const std::locale &a2,
                                std::wformat_string<T2...> a3, T2 &&...a4) {
  return std::format_to_n<T1, T2...>(a0, a1, a2, a3, std::forward<T2>(a4)...);
}

template <typename T1, typename... T2>
std::format_to_n_result<T1> f18(T1 a0, std::iter_difference_t<T1> a1,
                                std::format_string<T2...> a2, T2 &&...a3) {
  return std::format_to_n<T1, T2...>(a0, a1, a2, std::forward<T2>(a3)...);
}

template <typename T1, typename... T2>
std::format_to_n_result<T1> f19(T1 a0, std::iter_difference_t<T1> a1,
                                std::wformat_string<T2...> a2, T2 &&...a3) {
  return std::format_to_n<T1, T2...>(a0, a1, a2, std::forward<T2>(a3)...);
}

template <typename... T1>
std::size_t f20(const std::locale &a0, std::format_string<T1...> a1,
                T1 &&...a2) {
  return std::formatted_size<T1...>(a0, a1, std::forward<T1>(a2)...);
}

template <typename... T1>
std::size_t f21(const std::locale &a0, std::wformat_string<T1...> a1,
                T1 &&...a2) {
  return std::formatted_size<T1...>(a0, a1, std::forward<T1>(a2)...);
}

template <typename... T1>
std::size_t f22(std::format_string<T1...> a0, T1 &&...a1) {
  return std::formatted_size<T1...>(a0, std::forward<T1>(a1)...);
}

template <typename... T1>
std::size_t f23(std::wformat_string<T1...> a0, T1 &&...a1) {
  return std::formatted_size<T1...>(a0, std::forward<T1>(a1)...);
}

template <typename T1, typename... T2> auto f24(T2 &...a0) {
  return std::make_format_args<T1, T2...>(a0...);
}

template <typename... T1> auto f25(T1 &...a0) {
  return std::make_wformat_args<T1...>(a0...);
}

std::string f26(std::string_view a0, std::format_args a1) {
  return std::vformat(std::move(a0), std::move(a1));
}

std::wstring f27(std::wstring_view a0, std::wformat_args a1) {
  return std::vformat(std::move(a0), std::move(a1));
}

std::string f28(const std::locale &a0, std::string_view a1,
                std::format_args a2) {
  return std::vformat(a0, std::move(a1), std::move(a2));
}

std::wstring f29(const std::locale &a0, std::wstring_view a1,
                 std::wformat_args a2) {
  return std::vformat(a0, std::move(a1), std::move(a2));
}

template <typename T1> T1 f30(T1 a0, std::string_view a1, std::format_args a2) {
  return std::vformat_to<T1>(a0, std::move(a1), std::move(a2));
}

template <typename T1>
T1 f31(T1 a0, std::wstring_view a1, std::wformat_args a2) {
  return std::vformat_to<T1>(a0, std::move(a1), std::move(a2));
}

template <typename T1>
T1 f32(T1 a0, const std::locale &a1, std::string_view a2, std::format_args a3) {
  return std::vformat_to<T1>(a0, a1, std::move(a2), std::move(a3));
}

template <typename T1>
T1 f33(T1 a0, const std::locale &a1, std::wstring_view a2,
       std::wformat_args a3) {
  return std::vformat_to<T1>(a0, a1, std::move(a2), std::move(a3));
}

template <typename T1, typename T2>
decltype(auto) f34(T1 &&a0, std::basic_format_arg<T2> a1) {
  return std::visit_format_arg<T1, T2>(std::forward<T1>(a0), a1);
}

using t6 = std::wformat_args;

using t7 = std::wformat_context;

using t8 = std::wformat_parse_context;

template <typename... T1> using t9 = std::wformat_string<T1...>;
