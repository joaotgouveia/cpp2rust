#include <ios>
#include <iosfwd>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
std::istreambuf_iterator<T1, T2>
f11(const std::istreambuf_iterator<T1, T2> &a0) {
  return std::istreambuf_iterator<T1, T2>(a0);
}

template <typename T1, typename T2>
std::istreambuf_iterator<T1, T2> &
f12(std::istreambuf_iterator<T1, T2> &a0,
    const std::istreambuf_iterator<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::istreambuf_iterator<T1, T2> f13(std::istreambuf_iterator<T1, T2> &a0,
                                     int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
using t7 = typename std::istreambuf_iterator<T1, T2>::streambuf_type::char_type;

template <typename T1, typename T2>
using t8 =
    typename std::istreambuf_iterator<T1, T2>::streambuf_type::traits_type;

template <typename T1, typename T2>
using t9 = typename std::istreambuf_iterator<T1, T2>::streambuf_type::int_type;

template <typename T1, typename T2>
using t10 = typename std::istreambuf_iterator<T1, T2>::streambuf_type::pos_type;

template <typename T1, typename T2>
using t11 = typename std::istreambuf_iterator<T1, T2>::streambuf_type::off_type;

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *
f14(const typename std::istreambuf_iterator<T1, T2>::istream_type &a0) {
  return a0.tie();
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *
f15(typename std::istreambuf_iterator<T1, T2>::istream_type &a0,
    std::basic_ostream<T1, T2> *a1) {
  return a0.tie(a1);
}

template <typename T1, typename T2>
std::basic_streambuf<T1, T2> *
f16(const typename std::istreambuf_iterator<T1, T2>::istream_type &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2>
std::basic_streambuf<T1, T2> *
f17(typename std::istreambuf_iterator<T1, T2>::istream_type &a0,
    std::basic_streambuf<T1, T2> *a1) {
  return a0.rdbuf(a1);
}
