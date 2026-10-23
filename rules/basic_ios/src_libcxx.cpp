#include <ios>
#include <ostream>
#include <streambuf>
#include <utility>

template <typename T1, typename T2>
std::basic_ios<T1, T2>
f38(std::basic_streambuf<typename std::basic_ios<T1, T2>::char_type,
                         typename std::basic_ios<T1, T2>::traits_type> *a0) {
  return std::basic_ios<T1, T2>(a0);
}

template <typename T1, typename T2>
std::basic_ostream<typename std::basic_ios<T1, T2>::char_type,
                   typename std::basic_ios<T1, T2>::traits_type> *
f39(const std::basic_ios<T1, T2> &a0) {
  return a0.tie();
}

template <typename T1, typename T2>
std::basic_ostream<typename std::basic_ios<T1, T2>::char_type,
                   typename std::basic_ios<T1, T2>::traits_type> *
f40(std::basic_ios<T1, T2> &a0,
    std::basic_ostream<typename std::basic_ios<T1, T2>::char_type,
                       typename std::basic_ios<T1, T2>::traits_type> *a1) {
  return a0.tie(a1);
}

template <typename T1, typename T2>
std::basic_streambuf<typename std::basic_ios<T1, T2>::char_type,
                     typename std::basic_ios<T1, T2>::traits_type> *
f41(const std::basic_ios<T1, T2> &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2>
std::basic_streambuf<typename std::basic_ios<T1, T2>::char_type,
                     typename std::basic_ios<T1, T2>::traits_type> *
f42(std::basic_ios<T1, T2> &a0,
    std::basic_streambuf<typename std::basic_ios<T1, T2>::char_type,
                         typename std::basic_ios<T1, T2>::traits_type> *a1) {
  return a0.rdbuf(a1);
}
