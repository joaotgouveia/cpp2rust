#include <algorithm>
#include <bitset>
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

using t70 =
    std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>;

std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>> f281() {
  return std::hash<
      std::basic_string_view<char8_t, std::char_traits<char8_t>>>();
}

std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
f282(const std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
         &a0) {
  return std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>(
      a0);
}

std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
f283(std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
         &&a0) {
  return std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>(
      std::move(a0));
}

std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>> &
f284(std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>> &a0,
     const std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
         &a1) {
  return a0.operator=(a1);
}

std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>> &
f285(std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>> &a0,
     std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
         &&a1) {
  return a0.operator=(std::move(a1));
}

std::size_t
f286(const std::hash<std::basic_string_view<char8_t, std::char_traits<char8_t>>>
         &a0,
     const std::basic_string_view<char8_t, std::char_traits<char8_t>> a1) {
  return a0.operator()(std::move(a1));
}

template <typename T1>
std::size_t f287(const std::hash<T1> &a0, T1 a1)
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
std::size_t f288(const std::hash<T1> &a0, T1 a1)
  requires(std::is_same<void,
                        typename std::enable_if<std::is_enum<T1>::value &&
                                                    std::__is_unqualified_v<T1>,
                                                void>::type>::value)
{
  return a0.operator()(std::move(a1));
}

template <typename T1>
std::size_t f289(const std::hash<T1> &a0, T1 a1)
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
std::size_t f290(const std::hash<T1> &a0, T1 a1)
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
std::size_t f291(const std::hash<T1> &a0, T1 a1)
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
std::size_t f292(const std::hash<T1> &a0, T1 a1)
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
std::size_t f293(const std::hash<T1> &a0, T1 a1)
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
std::size_t f294(const std::hash<T1> &a0, T1 a1)
  requires(std::is_same<
           void, typename std::enable_if<std::is_floating_point<T1>::value &&
                                             std::__is_unqualified_v<T1>,
                                         void>::type>::value)
{
  return a0.operator()(std::move(a1));
}
