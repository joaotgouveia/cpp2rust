#include <cstddef>
#include <filesystem>
#include <istream>
#include <ostream>
#include <string.h>
#include <string>
#include <string_view>
#include <type_traits>
#include <utility>

std::string f80(const std::filesystem::path &a0) { return a0.u8string(); }

std::string f81(const std::filesystem::path &a0) {
  return a0.generic_u8string();
}

template <typename T1, typename std::enable_if<
                           (!(std::is_same<T1, std::filesystem::path>::value)),
                           int>::type T2 = 0>
std::__enable_if_t<std::filesystem::__is_pathable<T1>::value,
                   std::filesystem::path &>
f82(std::filesystem::path &a0, const T1 &a1) {
  return a0.template operator= <T1>(a1);
}

template <typename T1, typename std::enable_if<
                           (!(std::is_same<T1, std::filesystem::path>::value)),
                           int>::type T2 = 0>
std::__enable_if_t<std::filesystem::__is_pathable<T1>::value,
                   std::filesystem::path &>
f83(std::filesystem::path &a0, const T1 &a1) {
  return a0.template operator/= <T1>(a1);
}

template <
    typename T1,
    std::__enable_if_t<std::filesystem::__can_convert_char<T1>::value, int> T2 =
        0,
    typename std::enable_if<
        (!(std::is_same<T1, const std::filesystem::path &>::value) &&
         !(std::is_same<T1,
                        const std::filesystem::path::string_type &>::value) &&
         !(std::is_same<T1, std::filesystem::path::__string_view>::value) &&
         !(std::is_same<T1,
                        const std::filesystem::path::value_type *>::value) &&
         !(std::is_same<T1, std::filesystem::path::value_type>::value)),
        int>::type T3 = 0>
std::filesystem::path &f84(std::filesystem::path &a0, T1 a1) {
  return a0.template operator+= <T1>(a1);
}

template <typename T1,
          typename std::enable_if<
              (!(std::is_same<T1, std::filesystem::path>::value) &&
               !(std::is_same<T1, std::filesystem::path::string_type>::value)),
              int>::type T2 = 0>
std::__enable_if_t<std::filesystem::__is_pathable<T1>::value,
                   std::filesystem::path &>
f85(std::filesystem::path &a0, const T1 &a1) {
  return a0.template operator+= <T1>(a1);
}

bool f86(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return operator>=(a0, a1);
}

bool f87(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return operator>(a0, a1);
}

bool f88(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return operator<=(a0, a1);
}

bool f89(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return operator<(a0, a1);
}

bool f90(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return operator!=(a0, a1);
}
