#include <algorithm>
#include <format>
#include <ios>
#include <iosfwd>
#include <utility>

template <typename... T1>
std::string f7(const std::locale &a0, std::format_string<T1...> a1,
               T1 &&...a2) {
  return std::format<T1...>(a0, a1, std::forward<T1>(a2)...);
}

template <typename... T1>
std::wstring f8(const std::locale &a0, std::wformat_string<T1...> a1,
                T1 &&...a2) {
  return std::format<T1...>(a0, a1, std::forward<T1>(a2)...);
}

template <typename... T1>
std::string f9(std::format_string<T1...> a0, T1 &&...a1) {
  return std::format<T1...>(a0, std::forward<T1>(a1)...);
}

template <typename... T1>
std::wstring f10(std::wformat_string<T1...> a0, T1 &&...a1) {
  return std::format<T1...>(a0, std::forward<T1>(a1)...);
}

using t2 = std::format_args;

using t3 = std::format_context;

using t4 = std::format_error;

std::format_error f11(const std::string &a0) { return std::format_error(a0); }

std::format_error f12(const char *a0) { return std::format_error(a0); }

std::format_error f13(const std::format_error &a0) {
  return std::format_error(a0);
}

std::format_error f14(std::format_error &&a0) {
  return std::format_error(std::move(a0));
}

std::format_error &f15(std::format_error &a0, const std::format_error &a1) {
  return a0.operator=(a1);
}

std::format_error &f16(std::format_error &a0, std::format_error &&a1) {
  return a0.operator=(std::move(a1));
}

const char *f17(const std::format_error &a0) { return a0.what(); }

using t5 = std::format_parse_context;

template <typename... T1> using t6 = std::format_string<T1...>;

template <typename T1, typename... T2>
T1 f18(T1 a0, const std::locale &a1, std::format_string<T2...> a2, T2 &&...a3) {
  return std::format_to<T1, T2...>(a0, a1, a2, std::forward<T2>(a3)...);
}

template <typename T1, typename... T2>
T1 f19(T1 a0, const std::locale &a1, std::wformat_string<T2...> a2,
       T2 &&...a3) {
  return std::format_to<T1, T2...>(a0, a1, a2, std::forward<T2>(a3)...);
}

template <typename T1, typename... T2>
T1 f20(T1 a0, std::format_string<T2...> a1, T2 &&...a2) {
  return std::format_to<T1, T2...>(a0, a1, std::forward<T2>(a2)...);
}

template <typename T1, typename... T2>
T1 f21(T1 a0, std::wformat_string<T2...> a1, T2 &&...a2) {
  return std::format_to<T1, T2...>(a0, a1, std::forward<T2>(a2)...);
}

template <typename T1, typename... T2>
std::format_to_n_result<T1> f22(T1 a0, std::iter_difference_t<T1> a1,
                                const std::locale &a2,
                                std::format_string<T2...> a3, T2 &&...a4) {
  return std::format_to_n<T1, T2...>(a0, a1, a2, a3, std::forward<T2>(a4)...);
}

template <typename T1, typename... T2>
std::format_to_n_result<T1> f23(T1 a0, std::iter_difference_t<T1> a1,
                                const std::locale &a2,
                                std::wformat_string<T2...> a3, T2 &&...a4) {
  return std::format_to_n<T1, T2...>(a0, a1, a2, a3, std::forward<T2>(a4)...);
}

template <typename T1, typename... T2>
std::format_to_n_result<T1> f24(T1 a0, std::iter_difference_t<T1> a1,
                                std::format_string<T2...> a2, T2 &&...a3) {
  return std::format_to_n<T1, T2...>(a0, a1, a2, std::forward<T2>(a3)...);
}

template <typename T1, typename... T2>
std::format_to_n_result<T1> f25(T1 a0, std::iter_difference_t<T1> a1,
                                std::wformat_string<T2...> a2, T2 &&...a3) {
  return std::format_to_n<T1, T2...>(a0, a1, a2, std::forward<T2>(a3)...);
}

template <typename... T1>
std::size_t f26(const std::locale &a0, std::format_string<T1...> a1,
                T1 &&...a2) {
  return std::formatted_size<T1...>(a0, a1, std::forward<T1>(a2)...);
}

template <typename... T1>
std::size_t f27(const std::locale &a0, std::wformat_string<T1...> a1,
                T1 &&...a2) {
  return std::formatted_size<T1...>(a0, a1, std::forward<T1>(a2)...);
}

template <typename... T1>
std::size_t f28(std::format_string<T1...> a0, T1 &&...a1) {
  return std::formatted_size<T1...>(a0, std::forward<T1>(a1)...);
}

template <typename... T1>
std::size_t f29(std::wformat_string<T1...> a0, T1 &&...a1) {
  return std::formatted_size<T1...>(a0, std::forward<T1>(a1)...);
}

template <typename T1, typename... T2> auto f30(T2 &...a0) {
  return std::make_format_args<T1, T2...>(a0...);
}

template <typename... T1> auto f31(T1 &...a0) {
  return std::make_wformat_args<T1...>(a0...);
}

std::string f32(std::string_view a0, std::format_args a1) {
  return std::vformat(std::move(a0), std::move(a1));
}

std::wstring f33(std::wstring_view a0, std::wformat_args a1) {
  return std::vformat(std::move(a0), std::move(a1));
}

std::string f34(const std::locale &a0, std::string_view a1,
                std::format_args a2) {
  return std::vformat(a0, std::move(a1), std::move(a2));
}

std::wstring f35(const std::locale &a0, std::wstring_view a1,
                 std::wformat_args a2) {
  return std::vformat(a0, std::move(a1), std::move(a2));
}

template <typename T1> T1 f36(T1 a0, std::string_view a1, std::format_args a2) {
  return std::vformat_to<T1>(a0, std::move(a1), std::move(a2));
}

template <typename T1>
T1 f37(T1 a0, std::wstring_view a1, std::wformat_args a2) {
  return std::vformat_to<T1>(a0, std::move(a1), std::move(a2));
}

template <typename T1>
T1 f38(T1 a0, const std::locale &a1, std::string_view a2, std::format_args a3) {
  return std::vformat_to<T1>(a0, a1, std::move(a2), std::move(a3));
}

template <typename T1>
T1 f39(T1 a0, const std::locale &a1, std::wstring_view a2,
       std::wformat_args a3) {
  return std::vformat_to<T1>(a0, a1, std::move(a2), std::move(a3));
}

template <typename T1, typename T2>
decltype(auto) f40(T1 &&a0, std::basic_format_arg<T2> a1) {
  return std::visit_format_arg<T1, T2>(std::forward<T1>(a0), a1);
}

using t7 = std::wformat_args;

using t8 = std::wformat_context;

using t9 = std::wformat_parse_context;

template <typename... T1> using t10 = std::wformat_string<T1...>;
