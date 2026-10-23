#include <bitset>
#include <ios>
#include <memory>
#include <ostream>
#include <streambuf>
#include <string_view>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &
f62(std::basic_ostream<T1, T2> &a0,
    std::basic_ios<typename std::basic_ostream<T1, T2>::char_type,
                   typename std::basic_ostream<T1, T2>::traits_type> &(*a1)(
        std::basic_ios<typename std::basic_ostream<T1, T2>::char_type,
                       typename std::basic_ostream<T1, T2>::traits_type> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &
f63(std::basic_ostream<T1, T2> &a0,
    std::basic_streambuf<typename std::basic_ostream<T1, T2>::char_type,
                         typename std::basic_ostream<T1, T2>::traits_type>
        *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ostream<typename std::basic_ostream<T1, T2>::char_type,
                   typename std::basic_ostream<T1, T2>::traits_type> *
f64(const std::basic_ostream<T1, T2> &a0) {
  return a0.tie();
}

template <typename T1, typename T2>
std::basic_ostream<typename std::basic_ostream<T1, T2>::char_type,
                   typename std::basic_ostream<T1, T2>::traits_type> *
f65(std::basic_ostream<T1, T2> &a0,
    std::basic_ostream<typename std::basic_ostream<T1, T2>::char_type,
                       typename std::basic_ostream<T1, T2>::traits_type> *a1) {
  return a0.tie(a1);
}

template <typename T1, typename T2>
std::basic_streambuf<typename std::basic_ostream<T1, T2>::char_type,
                     typename std::basic_ostream<T1, T2>::traits_type> *
f66(const std::basic_ostream<T1, T2> &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2>
std::basic_streambuf<typename std::basic_ostream<T1, T2>::char_type,
                     typename std::basic_ostream<T1, T2>::traits_type> *
f67(std::basic_ostream<T1, T2> &a0,
    std::basic_streambuf<typename std::basic_ostream<T1, T2>::char_type,
                         typename std::basic_ostream<T1, T2>::traits_type>
        *a1) {
  return a0.rdbuf(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f68(std::basic_ostream<T1, T2> &a0) {
  return std::endl<T1, T2>(a0);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f69(std::basic_ostream<T1, T2> &a0) {
  return std::ends<T1, T2>(a0);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f70(std::basic_ostream<T1, T2> &a0) {
  return std::flush<T1, T2>(a0);
}

template <typename T1>
std::basic_ostream<char, T1> &f71(std::basic_ostream<char, T1> &a0,
                                  const char *a1) {
  return a0 << a1;
}

template <typename T1>
std::basic_ostream<char, T1> &f72(std::basic_ostream<char, T1> &a0,
                                  const unsigned char *a1) {
  return a0 << a1;
}

template <typename T1>
std::basic_ostream<char, T1> &f73(std::basic_ostream<char, T1> &a0,
                                  const signed char *a1) {
  return a0 << a1;
}

template <typename T1>
std::basic_ostream<char, T1> &f74(std::basic_ostream<char, T1> &a0, char a1) {
  return a0 << a1;
}

template <typename T1>
std::basic_ostream<char, T1> &f75(std::basic_ostream<char, T1> &a0,
                                  unsigned char a1) {
  return a0 << a1;
}

template <typename T1>
std::basic_ostream<char, T1> &f76(std::basic_ostream<char, T1> &a0,
                                  signed char a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f77(std::basic_ostream<T1, T2> &a0,
                                const std::error_code &a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f78(std::basic_ostream<T1, T2> &a0,
                                const char *a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f79(std::basic_ostream<T1, T2> &a0, const T1 *a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f80(std::basic_ostream<T1, T2> &a0,
                                std::basic_string_view<T1, T2> a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f81(std::basic_ostream<T1, T2> &a0, char a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f82(std::basic_ostream<T1, T2> &a0, T1 a1) {
  return a0 << a1;
}

template <typename T1, typename T2, std::size_t T3>
std::basic_ostream<T1, T2> &f83(std::basic_ostream<T1, T2> &a0,
                                const std::bitset<T3> &a1) {
  return a0 << a1;
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f84(std::basic_ostream<T1, T2> &a0,
                                const std::shared_ptr<T3> &a1) {
  return a0 << a1;
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f85(std::basic_ostream<T1, T2> &a0,
                                const std::basic_string<T1, T2, T3> &a1) {
  return a0 << a1;
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<true, int>::type T5 = 0>
std::basic_ostream<T1, T2> &f86(std::basic_ostream<T1, T2> &a0,
                                const std::unique_ptr<T3, T4> &a1) {
  return a0 << a1;
}
