#include "../common/init.h"
#include "../common/traits.h"
#include <ios>
#include <istream>
#include <ostream>
#include <streambuf>
#include <utility>

template <typename T1, typename T2>
std::basic_istream<T1, T2> &
f75(std::basic_istream<T1, T2> &a0,
    std::basic_ios<typename std::basic_istream<T1, T2>::char_type,
                   typename std::basic_istream<T1, T2>::traits_type> &(*a1)(
        std::basic_ios<typename std::basic_istream<T1, T2>::char_type,
                       typename std::basic_istream<T1, T2>::traits_type> &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &
f76(std::basic_istream<T1, T2> &a0,
    std::basic_streambuf<typename std::basic_istream<T1, T2>::char_type,
                         typename std::basic_istream<T1, T2>::traits_type>
        *a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &
f77(std::basic_istream<T1, T2> &a0,
    std::basic_streambuf<typename std::basic_istream<T1, T2>::char_type,
                         typename std::basic_istream<T1, T2>::traits_type>
        &a1) {
  return a0.get(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &
f78(std::basic_istream<T1, T2> &a0,
    std::basic_streambuf<typename std::basic_istream<T1, T2>::char_type,
                         typename std::basic_istream<T1, T2>::traits_type> &a1,
    typename std::basic_istream<T1, T2>::char_type a2) {
  return a0.get(a1, a2);
}

template <
    typename T1, typename T2,
    typename T3 = typename std::basic_istream<T1, T2>::char_type,
    typename std::enable_if<std::is_same<T1, char>::value, int>::type T4 = 0>
std::basic_istream<T1, T2> &
f79(std::basic_istream<T1, T2> &a0, std::streamsize a1,
    typename std::basic_istream<T1, T2>::char_type a2) {
  return a0.ignore(a1, a2);
}

template <typename T1, typename T2>
std::basic_ostream<typename std::basic_istream<T1, T2>::char_type,
                   typename std::basic_istream<T1, T2>::traits_type> *
f80(const std::basic_istream<T1, T2> &a0) {
  return a0.tie();
}

template <typename T1, typename T2>
std::basic_ostream<typename std::basic_istream<T1, T2>::char_type,
                   typename std::basic_istream<T1, T2>::traits_type> *
f81(std::basic_istream<T1, T2> &a0,
    std::basic_ostream<typename std::basic_istream<T1, T2>::char_type,
                       typename std::basic_istream<T1, T2>::traits_type> *a1) {
  return a0.tie(a1);
}

template <typename T1, typename T2>
std::basic_streambuf<typename std::basic_istream<T1, T2>::char_type,
                     typename std::basic_istream<T1, T2>::traits_type> *
f82(const std::basic_istream<T1, T2> &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2>
std::basic_streambuf<typename std::basic_istream<T1, T2>::char_type,
                     typename std::basic_istream<T1, T2>::traits_type> *
f83(std::basic_istream<T1, T2> &a0,
    std::basic_streambuf<typename std::basic_istream<T1, T2>::char_type,
                         typename std::basic_istream<T1, T2>::traits_type>
        *a1) {
  return a0.rdbuf(a1);
}
