#include <algorithm>
#include <coroutine>
#include <cstddef>
#include <filesystem>
#include <functional>
#include <memory>
#include <optional>
#include <string>
#include <string_view>
#include <system_error>
#include <typeindex>
#include <utility>
#include <variant>
#include <vector>

using t68 =
    std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>;

std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>> f250() {
  return std::hash<
      std::basic_string_view<char8_t, std::char_traits<char8_t>>>();
}

std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
f251(const std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
         &a0) {
  return std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>(
      a0);
}

std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
f252(std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
         &&a0) {
  return std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>(
      std::move(a0));
}

std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>> &
f253(std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>> &a0,
     const std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
         &a1) {
  return a0.operator=(a1);
}

std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>> &
f254(std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>> &a0,
     std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
         &&a1) {
  return a0.operator=(std::move(a1));
}

std::size_t
f255(const std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
         &a0,
     const std::basic_string_view<char8_t, std::char_traits<char8_t>> a1) {
  return a0.operator()(std::move(a1));
}
