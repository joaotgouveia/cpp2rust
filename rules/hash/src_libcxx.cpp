#include <algorithm>
#include <bitset>
#include <cstddef>
#include <functional>
#include <memory>
#include <string>
#include <string_view>
#include <system_error>
#include <typeindex>
#include <utility>
#include <vector>

using t53 = std::hash<std::basic_string_view<char, std::char_traits<char>>>;

std::hash<std::basic_string_view<char, std::char_traits<char>>> f233() {
  return std::hash<std::basic_string_view<char, std::char_traits<char>>>();
}

std::hash<std::basic_string_view<char, std::char_traits<char>>> f234(
    const std::hash<std::basic_string_view<char, std::char_traits<char>>> &a0) {
  return std::hash<std::basic_string_view<char, std::char_traits<char>>>(a0);
}

std::hash<std::basic_string_view<char, std::char_traits<char>>>
f235(std::hash<std::basic_string_view<char, std::char_traits<char>>> &&a0) {
  return std::hash<std::basic_string_view<char, std::char_traits<char>>>(
      std::move(a0));
}

std::hash<std::basic_string_view<char, std::char_traits<char>>> &f236(
    std::hash<std::basic_string_view<char, std::char_traits<char>>> &a0,
    const std::hash<std::basic_string_view<char, std::char_traits<char>>> &a1) {
  return a0.operator=(a1);
}

std::hash<std::basic_string_view<char, std::char_traits<char>>> &
f237(std::hash<std::basic_string_view<char, std::char_traits<char>>> &a0,
     std::hash<std::basic_string_view<char, std::char_traits<char>>> &&a1) {
  return a0.operator=(std::move(a1));
}

std::size_t
f238(const std::hash<std::basic_string_view<char, std::char_traits<char>>> &a0,
     const std::basic_string_view<char, std::char_traits<char>> a1) {
  return a0.operator()(std::move(a1));
}

using t54 =
    std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>>;

std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>> f239() {
  return std::hash<
      std::basic_string_view<wchar_t, std::char_traits<wchar_t>>>();
}

std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>>
f240(const std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>>
         &a0) {
  return std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>>(
      a0);
}

std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>>
f241(std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>>
         &&a0) {
  return std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>>(
      std::move(a0));
}

std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>> &
f242(std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>> &a0,
     const std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>>
         &a1) {
  return a0.operator=(a1);
}

std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>> &
f243(std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>> &a0,
     std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>>
         &&a1) {
  return a0.operator=(std::move(a1));
}

std::size_t
f244(const std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>>
         &a0,
     const std::basic_string_view<wchar_t, std::char_traits<wchar_t>> a1) {
  return a0.operator()(std::move(a1));
}

using t55 =
    std::hash<std::basic_string_view<char16_t, std::char_traits<char16_t>>>;

std::hash<std::basic_string_view<char16_t, std::char_traits<char16_t>>> f245() {
  return std::hash<
      std::basic_string_view<char16_t, std::char_traits<char16_t>>>();
}

std::hash<std::basic_string_view<char16_t, std::char_traits<char16_t>>>
f246(const std::hash<
     std::basic_string_view<char16_t, std::char_traits<char16_t>>> &a0) {
  return std::hash<
      std::basic_string_view<char16_t, std::char_traits<char16_t>>>(a0);
}

std::hash<std::basic_string_view<char16_t, std::char_traits<char16_t>>>
f247(std::hash<std::basic_string_view<char16_t, std::char_traits<char16_t>>>
         &&a0) {
  return std::hash<
      std::basic_string_view<char16_t, std::char_traits<char16_t>>>(
      std::move(a0));
}

std::hash<std::basic_string_view<char16_t, std::char_traits<char16_t>>> &f248(
    std::hash<std::basic_string_view<char16_t, std::char_traits<char16_t>>> &a0,
    const std::hash<
        std::basic_string_view<char16_t, std::char_traits<char16_t>>> &a1) {
  return a0.operator=(a1);
}

std::hash<std::basic_string_view<char16_t, std::char_traits<char16_t>>> &f249(
    std::hash<std::basic_string_view<char16_t, std::char_traits<char16_t>>> &a0,
    std::hash<std::basic_string_view<char16_t, std::char_traits<char16_t>>>
        &&a1) {
  return a0.operator=(std::move(a1));
}

std::size_t
f250(const std::hash<
         std::basic_string_view<char16_t, std::char_traits<char16_t>>> &a0,
     const std::basic_string_view<char16_t, std::char_traits<char16_t>> a1) {
  return a0.operator()(std::move(a1));
}

using t56 =
    std::hash<std::basic_string_view<char32_t, std::char_traits<char32_t>>>;

std::hash<std::basic_string_view<char32_t, std::char_traits<char32_t>>> f251() {
  return std::hash<
      std::basic_string_view<char32_t, std::char_traits<char32_t>>>();
}

std::hash<std::basic_string_view<char32_t, std::char_traits<char32_t>>>
f252(const std::hash<
     std::basic_string_view<char32_t, std::char_traits<char32_t>>> &a0) {
  return std::hash<
      std::basic_string_view<char32_t, std::char_traits<char32_t>>>(a0);
}

std::hash<std::basic_string_view<char32_t, std::char_traits<char32_t>>>
f253(std::hash<std::basic_string_view<char32_t, std::char_traits<char32_t>>>
         &&a0) {
  return std::hash<
      std::basic_string_view<char32_t, std::char_traits<char32_t>>>(
      std::move(a0));
}

std::hash<std::basic_string_view<char32_t, std::char_traits<char32_t>>> &f254(
    std::hash<std::basic_string_view<char32_t, std::char_traits<char32_t>>> &a0,
    const std::hash<
        std::basic_string_view<char32_t, std::char_traits<char32_t>>> &a1) {
  return a0.operator=(a1);
}

std::hash<std::basic_string_view<char32_t, std::char_traits<char32_t>>> &f255(
    std::hash<std::basic_string_view<char32_t, std::char_traits<char32_t>>> &a0,
    std::hash<std::basic_string_view<char32_t, std::char_traits<char32_t>>>
        &&a1) {
  return a0.operator=(std::move(a1));
}

std::size_t
f256(const std::hash<
         std::basic_string_view<char32_t, std::char_traits<char32_t>>> &a0,
     const std::basic_string_view<char32_t, std::char_traits<char32_t>> a1) {
  return a0.operator()(std::move(a1));
}

using t57 = std::hash<std::error_condition>;

std::size_t f257(const std::hash<std::error_condition> &a0,
                 const std::error_condition &a1) {
  return a0.operator()(a1);
}

std::hash<std::error_condition> f258() {
  return std::hash<std::error_condition>();
}

std::hash<std::error_condition>
f259(const std::hash<std::error_condition> &a0) {
  return std::hash<std::error_condition>(a0);
}

std::hash<std::error_condition> f260(std::hash<std::error_condition> &&a0) {
  return std::hash<std::error_condition>(std::move(a0));
}

std::hash<std::error_condition> &
f261(std::hash<std::error_condition> &a0,
     const std::hash<std::error_condition> &a1) {
  return a0.operator=(a1);
}

std::hash<std::error_condition> &f262(std::hash<std::error_condition> &a0,
                                      std::hash<std::error_condition> &&a1) {
  return a0.operator=(std::move(a1));
}

std::size_t f263(const std::hash<std::type_index> &a0, std::type_index a1) {
  return a0.operator()(std::move(a1));
}

using t58 = std::hash<std::nullptr_t>;

std::size_t f264(const std::hash<std::nullptr_t> &a0, std::nullptr_t a1) {
  return a0.operator()(a1);
}

std::hash<std::nullptr_t> f265() { return std::hash<std::nullptr_t>(); }

std::hash<std::nullptr_t> f266(const std::hash<std::nullptr_t> &a0) {
  return std::hash<std::nullptr_t>(a0);
}

std::hash<std::nullptr_t> f267(std::hash<std::nullptr_t> &&a0) {
  return std::hash<std::nullptr_t>(std::move(a0));
}

std::hash<std::nullptr_t> &f268(std::hash<std::nullptr_t> &a0,
                                const std::hash<std::nullptr_t> &a1) {
  return a0.operator=(a1);
}

std::hash<std::nullptr_t> &f269(std::hash<std::nullptr_t> &a0,
                                std::hash<std::nullptr_t> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
using t59 = std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>, typename std::unique_ptr<T1, T2>::pointer>>;

template <typename T1, typename T2>
std::size_t
f270(const std::hash<std::__enable_hash_helper<
         std::unique_ptr<T1, T2>, typename std::unique_ptr<T1, T2>::pointer>>
         &a0,
     const std::unique_ptr<T1, T2> &a1) {
  return a0.operator()(a1);
}
