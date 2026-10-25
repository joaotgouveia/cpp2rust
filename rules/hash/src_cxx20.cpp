#include <algorithm>
#include <atomic>
#include <barrier>
#include <bitset>
#include <chrono>
#include <coroutine>
#include <cstddef>
#include <filesystem>
#include <format>
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

template <typename T1>
using t16 =
    std::hash<std::basic_string<char8_t, std::char_traits<char8_t>, T1>>;

template <typename T1>
std::size_t
f31(const std::hash<std::basic_string<char8_t, std::char_traits<char8_t>, T1>>
        &a0,
    const std::basic_string<char8_t, std::char_traits<char8_t>, T1> &a1) {
  return a0.operator()(a1);
}

template <typename T1> using t17 = std::hash<std::coroutine_handle<T1>>;

template <typename T1>
std::size_t f32(const std::hash<std::coroutine_handle<T1>> &a0,
                const std::coroutine_handle<T1> &a1) {
  return a0.operator()(a1);
}
