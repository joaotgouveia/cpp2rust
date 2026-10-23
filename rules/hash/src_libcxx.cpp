#include <algorithm>
#include <cstddef>
#include <functional>
#include <memory>
#include <string>
#include <string_view>
#include <system_error>
#include <typeindex>
#include <utility>
#include <vector>

using t51 = std::hash<std::basic_string_view<char, std::char_traits<char>>>;

std::hash<std::basic_string_view<char, std::char_traits<char>>> f209() {
  return std::hash<std::basic_string_view<char, std::char_traits<char>>>();
}

std::hash<std::basic_string_view<char, std::char_traits<char>>> f210(
    const std::hash<std::basic_string_view<char, std::char_traits<char>>> &a0) {
  return std::hash<std::basic_string_view<char, std::char_traits<char>>>(a0);
}

std::hash<std::basic_string_view<char, std::char_traits<char>>>
f211(std::hash<std::basic_string_view<char, std::char_traits<char>>> &&a0) {
  return std::hash<std::basic_string_view<char, std::char_traits<char>>>(
      std::move(a0));
}

std::hash<std::basic_string_view<char, std::char_traits<char>>> &f212(
    std::hash<std::basic_string_view<char, std::char_traits<char>>> &a0,
    const std::hash<std::basic_string_view<char, std::char_traits<char>>> &a1) {
  return a0.operator=(a1);
}

std::hash<std::basic_string_view<char, std::char_traits<char>>> &
f213(std::hash<std::basic_string_view<char, std::char_traits<char>>> &a0,
     std::hash<std::basic_string_view<char, std::char_traits<char>>> &&a1) {
  return a0.operator=(std::move(a1));
}

std::size_t
f214(const std::hash<std::basic_string_view<char, std::char_traits<char>>> &a0,
     const std::basic_string_view<char, std::char_traits<char>> a1) {
  return a0.operator()(std::move(a1));
}

using t52 =
    std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>>;

std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>> f215() {
  return std::hash<
      std::basic_string_view<wchar_t, std::char_traits<wchar_t>>>();
}

std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>>
f216(const std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>>
         &a0) {
  return std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>>(
      a0);
}

std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>>
f217(std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>>
         &&a0) {
  return std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>>(
      std::move(a0));
}

std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>> &
f218(std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>> &a0,
     const std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>>
         &a1) {
  return a0.operator=(a1);
}

std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>> &
f219(std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>> &a0,
     std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>>
         &&a1) {
  return a0.operator=(std::move(a1));
}

std::size_t
f220(const std::hash<std::basic_string_view<wchar_t, std::char_traits<wchar_t>>>
         &a0,
     const std::basic_string_view<wchar_t, std::char_traits<wchar_t>> a1) {
  return a0.operator()(std::move(a1));
}

using t53 =
    std::hash<std::basic_string_view<char16_t, std::char_traits<char16_t>>>;

std::hash<std::basic_string_view<char16_t, std::char_traits<char16_t>>> f221() {
  return std::hash<
      std::basic_string_view<char16_t, std::char_traits<char16_t>>>();
}

std::hash<std::basic_string_view<char16_t, std::char_traits<char16_t>>>
f222(const std::hash<
     std::basic_string_view<char16_t, std::char_traits<char16_t>>> &a0) {
  return std::hash<
      std::basic_string_view<char16_t, std::char_traits<char16_t>>>(a0);
}

std::hash<std::basic_string_view<char16_t, std::char_traits<char16_t>>>
f223(std::hash<std::basic_string_view<char16_t, std::char_traits<char16_t>>>
         &&a0) {
  return std::hash<
      std::basic_string_view<char16_t, std::char_traits<char16_t>>>(
      std::move(a0));
}

std::hash<std::basic_string_view<char16_t, std::char_traits<char16_t>>> &f224(
    std::hash<std::basic_string_view<char16_t, std::char_traits<char16_t>>> &a0,
    const std::hash<
        std::basic_string_view<char16_t, std::char_traits<char16_t>>> &a1) {
  return a0.operator=(a1);
}

std::hash<std::basic_string_view<char16_t, std::char_traits<char16_t>>> &f225(
    std::hash<std::basic_string_view<char16_t, std::char_traits<char16_t>>> &a0,
    std::hash<std::basic_string_view<char16_t, std::char_traits<char16_t>>>
        &&a1) {
  return a0.operator=(std::move(a1));
}

std::size_t
f226(const std::hash<
         std::basic_string_view<char16_t, std::char_traits<char16_t>>> &a0,
     const std::basic_string_view<char16_t, std::char_traits<char16_t>> a1) {
  return a0.operator()(std::move(a1));
}

using t54 =
    std::hash<std::basic_string_view<char32_t, std::char_traits<char32_t>>>;

std::hash<std::basic_string_view<char32_t, std::char_traits<char32_t>>> f227() {
  return std::hash<
      std::basic_string_view<char32_t, std::char_traits<char32_t>>>();
}

std::hash<std::basic_string_view<char32_t, std::char_traits<char32_t>>>
f228(const std::hash<
     std::basic_string_view<char32_t, std::char_traits<char32_t>>> &a0) {
  return std::hash<
      std::basic_string_view<char32_t, std::char_traits<char32_t>>>(a0);
}

std::hash<std::basic_string_view<char32_t, std::char_traits<char32_t>>>
f229(std::hash<std::basic_string_view<char32_t, std::char_traits<char32_t>>>
         &&a0) {
  return std::hash<
      std::basic_string_view<char32_t, std::char_traits<char32_t>>>(
      std::move(a0));
}

std::hash<std::basic_string_view<char32_t, std::char_traits<char32_t>>> &f230(
    std::hash<std::basic_string_view<char32_t, std::char_traits<char32_t>>> &a0,
    const std::hash<
        std::basic_string_view<char32_t, std::char_traits<char32_t>>> &a1) {
  return a0.operator=(a1);
}

std::hash<std::basic_string_view<char32_t, std::char_traits<char32_t>>> &f231(
    std::hash<std::basic_string_view<char32_t, std::char_traits<char32_t>>> &a0,
    std::hash<std::basic_string_view<char32_t, std::char_traits<char32_t>>>
        &&a1) {
  return a0.operator=(std::move(a1));
}

std::size_t
f232(const std::hash<
         std::basic_string_view<char32_t, std::char_traits<char32_t>>> &a0,
     const std::basic_string_view<char32_t, std::char_traits<char32_t>> a1) {
  return a0.operator()(std::move(a1));
}

using t55 = std::hash<std::error_condition>;

std::size_t f233(const std::hash<std::error_condition> &a0,
                 const std::error_condition &a1) {
  return a0.operator()(a1);
}

std::hash<std::error_condition> f234() {
  return std::hash<std::error_condition>();
}

std::hash<std::error_condition>
f235(const std::hash<std::error_condition> &a0) {
  return std::hash<std::error_condition>(a0);
}

std::hash<std::error_condition> f236(std::hash<std::error_condition> &&a0) {
  return std::hash<std::error_condition>(std::move(a0));
}

std::hash<std::error_condition> &
f237(std::hash<std::error_condition> &a0,
     const std::hash<std::error_condition> &a1) {
  return a0.operator=(a1);
}

std::hash<std::error_condition> &f238(std::hash<std::error_condition> &a0,
                                      std::hash<std::error_condition> &&a1) {
  return a0.operator=(std::move(a1));
}

std::size_t f239(const std::hash<std::type_index> &a0, std::type_index a1) {
  return a0.operator()(std::move(a1));
}

using t56 = std::hash<std::nullptr_t>;

std::size_t f240(const std::hash<std::nullptr_t> &a0, std::nullptr_t a1) {
  return a0.operator()(a1);
}

std::hash<std::nullptr_t> f241() { return std::hash<std::nullptr_t>(); }

std::hash<std::nullptr_t> f242(const std::hash<std::nullptr_t> &a0) {
  return std::hash<std::nullptr_t>(a0);
}

std::hash<std::nullptr_t> f243(std::hash<std::nullptr_t> &&a0) {
  return std::hash<std::nullptr_t>(std::move(a0));
}

std::hash<std::nullptr_t> &f244(std::hash<std::nullptr_t> &a0,
                                const std::hash<std::nullptr_t> &a1) {
  return a0.operator=(a1);
}

std::hash<std::nullptr_t> &f245(std::hash<std::nullptr_t> &a0,
                                std::hash<std::nullptr_t> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
using t57 = std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>, typename std::unique_ptr<T1, T2>::pointer>>;

template <typename T1, typename T2>
std::size_t
f246(const std::hash<std::__enable_hash_helper<
         std::unique_ptr<T1, T2>, typename std::unique_ptr<T1, T2>::pointer>>
         &a0,
     const std::unique_ptr<T1, T2> &a1) {
  return a0.operator()(a1);
}

template <typename T1> std::size_t f247(const std::hash<T1> &a0, T1 a1) {
  return a0.operator()(a1);
}
