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

std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>> f252() {
  return std::hash<
      std::basic_string_view<char8_t, std::char_traits<char8_t>>>();
}

std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
f253(const std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
         &a0) {
  return std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>(
      a0);
}

std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
f254(std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
         &&a0) {
  return std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>(
      std::move(a0));
}

std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>> &
f255(std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>> &a0,
     const std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
         &a1) {
  return a0.operator=(a1);
}

std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>> &
f256(std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>> &a0,
     std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
         &&a1) {
  return a0.operator=(std::move(a1));
}

std::size_t
f257(const std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
         &a0,
     const std::basic_string_view<char8_t, std::char_traits<char8_t>> a1) {
  return a0.operator()(std::move(a1));
}

template <typename T1>
std::size_t f258(const std::hash<T1> &a0, T1 a1)
  requires(
      std::is_same<
          void, typename std::enable_if<std::is_integral<T1>::value &&
                                            std::__is_unqualified_v<T1> &&
                                            (sizeof(T1) <= sizeof(std::size_t)),
                                        void>::type>::value)
{
  return a0.operator()(a1);
}

template <typename T1>
std::size_t f259(const std::hash<T1> &a0, T1 a1)
  requires(std::is_same<void,
                        typename std::enable_if<std::is_enum<T1>::value &&
                                                    std::__is_unqualified_v<T1>,
                                                void>::type>::value)
{
  return a0.operator()(a1);
}

template <typename T1>
std::size_t f260(const std::hash<T1> &a0, T1 a1)
  requires(std::is_same<
           void, typename std::enable_if<std::is_floating_point<T1>::value &&
                                             std::__is_unqualified_v<T1>,
                                         void>::type>::value)
{
  return a0.operator()(a1);
}
