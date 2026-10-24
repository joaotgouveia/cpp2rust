#include <algorithm>
#include <filesystem>
#include <ios>
#include <istream>
#include <string>
#include <system_error>
#include <utility>

std::string f95(const std::filesystem::path &a0) { return a0.u8string(); }

std::string f96(const std::filesystem::path &a0) {
  return a0.generic_u8string();
}

template <typename T1, typename std::enable_if<
                           (!(std::is_same<T1, std::filesystem::path>::value)),
                           int>::type T2 = 0>
std::filesystem::__detail::_Path<T1> &f97(std::filesystem::path &a0,
                                          const T1 &a1) {
  return a0.template operator= <T1>(a1);
}

template <typename T1, typename std::enable_if<
                           (!(std::is_same<T1, std::filesystem::path>::value)),
                           int>::type T2 = 0>
std::filesystem::__detail::_Path<T1> &f98(std::filesystem::path &a0,
                                          const T1 &a1) {
  return a0.template operator/= <T1>(a1);
}

template <typename T1,
          typename std::enable_if<
              (!(std::is_same<T1, std::filesystem::path>::value) &&
               !(std::is_same<T1, std::filesystem::path::string_type>::value)),
              int>::type T2 = 0>
std::filesystem::__detail::_Path<T1> &f99(std::filesystem::path &a0,
                                          const T1 &a1) {
  return a0.template operator+= <T1>(a1);
}

template <
    typename T1,
    typename std::enable_if<
        (!(std::is_same<T1, const std::filesystem::path &>::value) &&
         !(std::is_same<T1,
                        const std::filesystem::path::string_type &>::value) &&
         !(std::is_same<T1,
                        const std::filesystem::path::value_type *>::value) &&
         !(std::is_same<T1, std::filesystem::path::value_type>::value) &&
         !(std::is_same<T1, std::basic_string_view<
                                std::filesystem::path::value_type>>::value)),
        int>::type T2 = 0>
std::filesystem::__detail::_Path2<T1 *> &f100(std::filesystem::path &a0,
                                              T1 a1) {
  return a0.template operator+= <T1>(std::move(a1));
}

bool f101(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return operator>=(a0, a1);
}

bool f102(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return operator>(a0, a1);
}

bool f103(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return operator<=(a0, a1);
}

bool f104(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return operator<(a0, a1);
}

bool f105(const std::filesystem::path &a0, const std::filesystem::path &a1) {
  return operator!=(a0, a1);
}
