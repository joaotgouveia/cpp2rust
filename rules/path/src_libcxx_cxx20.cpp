#include <compare>
#include <cstddef>
#include <filesystem>
#include <istream>
#include <ostream>
#include <string.h>
#include <string>
#include <string_view>
#include <type_traits>
#include <utility>

std::u8string f91(const std::filesystem::path &a0) { return a0.u8string(); }

std::u8string f92(const std::filesystem::path &a0) {
  return a0.generic_u8string();
}

template <typename T1>
std::__enable_if_t<std::filesystem::__is_pathable<T1>::value,
                   std::filesystem::path &>
f93(std::filesystem::path &a0, const T1 &a1)
  requires(!(std::is_same<T1, std::filesystem::path>::value))
{
  return a0.template operator= <T1>(a1);
}

template <typename T1>
std::__enable_if_t<std::filesystem::__is_pathable<T1>::value,
                   std::filesystem::path &>
f94(std::filesystem::path &a0, const T1 &a1)
  requires(!(std::is_same<T1, std::filesystem::path>::value))
{
  return a0.template operator/= <T1>(a1);
}

template <
    typename T1,
    std::__enable_if_t<std::filesystem::__can_convert_char<T1>::value, int> T2 =
        0>
std::filesystem::path &f95(std::filesystem::path &a0, T1 a1)
  requires(
      !(std::is_same<T1, const std::filesystem::path &>::value) &&
      !(std::is_same<T1, const std::filesystem::path::string_type &>::value) &&
      !(std::is_same<T1, std::filesystem::path::__string_view>::value) &&
      !(std::is_same<T1, const std::filesystem::path::value_type *>::value) &&
      !(std::is_same<T1, std::filesystem::path::value_type>::value))
{
  return a0.template operator+= <T1>(a1);
}

template <typename T1>
std::__enable_if_t<std::filesystem::__is_pathable<T1>::value,
                   std::filesystem::path &>
f96(std::filesystem::path &a0, const T1 &a1)
  requires(!(std::is_same<T1, std::filesystem::path>::value) &&
           !(std::is_same<T1, std::filesystem::path::string_type>::value))
{
  return a0.template operator+= <T1>(a1);
}

std::strong_ordering f97(const std::filesystem::path &a0,
                         const std::filesystem::path &a1) {
  return operator<=>(a0, a1);
}
