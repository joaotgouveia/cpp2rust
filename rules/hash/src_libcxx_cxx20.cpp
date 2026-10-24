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

std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>> f257() {
  return std::hash<
      std::basic_string_view<char8_t, std::char_traits<char8_t>>>();
}

std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
f258(const std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
         &a0) {
  return std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>(
      a0);
}

std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
f259(std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
         &&a0) {
  return std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>(
      std::move(a0));
}

std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>> &
f260(std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>> &a0,
     const std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
         &a1) {
  return a0.operator=(a1);
}

std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>> &
f261(std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>> &a0,
     std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
         &&a1) {
  return a0.operator=(std::move(a1));
}

std::size_t
f262(const std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
         &a0,
     const std::basic_string_view<char8_t, std::char_traits<char8_t>> a1) {
  return a0.operator()(std::move(a1));
}

template <typename T1>
std::size_t f263(const std::hash<T1> &a0, T1 a1)
  requires(
      std::is_same<
          void, typename std::enable_if<std::is_integral<T1>::value &&
                                            std::__is_unqualified_v<T1> &&
                                            (sizeof(T1) <= sizeof(std::size_t)),
                                        void>::type>::value)
{
  return a0.operator()(std::move(a1));
}

template <typename T1>
std::size_t f264(const std::hash<T1> &a0, T1 a1)
  requires(std::is_same<void,
                        typename std::enable_if<std::is_enum<T1>::value &&
                                                    std::__is_unqualified_v<T1>,
                                                void>::type>::value)
{
  return a0.operator()(std::move(a1));
}

template <typename T1>
std::size_t f265(const std::hash<T1> &a0, T1 a1)
  requires(sizeof(T1) / sizeof(std::size_t) == 2UL) &&
          (std::is_same<void, typename std::enable_if<
                                  std::is_integral<T1>::value &&
                                      std::__is_unqualified_v<T1> &&
                                      (sizeof(T1) > sizeof(std::size_t)),
                                  void>::type>::value)
{
  return a0.operator()(std::move(a1));
}

template <typename T1>
std::size_t f266(const std::hash<T1> &a0, T1 a1)
  requires(sizeof(T1) / sizeof(std::size_t) == 0UL) &&
          (std::is_same<void, typename std::enable_if<
                                  std::is_integral<T1>::value &&
                                      std::__is_unqualified_v<T1> &&
                                      (sizeof(T1) > sizeof(std::size_t)),
                                  void>::type>::value)
{
  return a0.operator()(std::move(a1));
}

template <typename T1>
std::size_t f267(const std::hash<T1> &a0, T1 a1)
  requires(sizeof(T1) / sizeof(std::size_t) == 1UL) &&
          (std::is_same<void, typename std::enable_if<
                                  std::is_integral<T1>::value &&
                                      std::__is_unqualified_v<T1> &&
                                      (sizeof(T1) > sizeof(std::size_t)),
                                  void>::type>::value)
{
  return a0.operator()(std::move(a1));
}

template <typename T1>
std::size_t f268(const std::hash<T1> &a0, T1 a1)
  requires(sizeof(T1) / sizeof(std::size_t) == 3UL) &&
          (std::is_same<void, typename std::enable_if<
                                  std::is_integral<T1>::value &&
                                      std::__is_unqualified_v<T1> &&
                                      (sizeof(T1) > sizeof(std::size_t)),
                                  void>::type>::value)
{
  return a0.operator()(std::move(a1));
}

template <typename T1>
std::size_t f269(const std::hash<T1> &a0, T1 a1)
  requires(sizeof(T1) / sizeof(std::size_t) == 4UL) &&
          (std::is_same<void, typename std::enable_if<
                                  std::is_integral<T1>::value &&
                                      std::__is_unqualified_v<T1> &&
                                      (sizeof(T1) > sizeof(std::size_t)),
                                  void>::type>::value)
{
  return a0.operator()(std::move(a1));
}

template <typename T1>
std::size_t f270(const std::hash<T1> &a0, T1 a1)
  requires(std::is_same<
           void, typename std::enable_if<std::is_floating_point<T1>::value &&
                                             std::__is_unqualified_v<T1>,
                                         void>::type>::value)
{
  return a0.operator()(std::move(a1));
}
