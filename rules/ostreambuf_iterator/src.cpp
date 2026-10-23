#include <ios>
#include <iosfwd>
#include <iterator>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2> using t1 = std::ostreambuf_iterator<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::ostreambuf_iterator<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::ostreambuf_iterator<T1, T2>::traits_type;

template <typename T1, typename T2>
using t4 = typename std::ostreambuf_iterator<T1, T2>::streambuf_type;

template <typename T1, typename T2>
using t5 = typename std::ostreambuf_iterator<T1, T2>::ostream_type;

template <typename T1, typename T2>
std::ostreambuf_iterator<T1, T2>
f1(typename std::ostreambuf_iterator<T1, T2>::ostream_type &a0) {
  return std::ostreambuf_iterator<T1, T2>(a0);
}

template <typename T1, typename T2>
std::ostreambuf_iterator<T1, T2>
f2(typename std::ostreambuf_iterator<T1, T2>::streambuf_type *a0) {
  return std::ostreambuf_iterator<T1, T2>(a0);
}

template <typename T1, typename T2>
std::ostreambuf_iterator<T1, T2> &f3(std::ostreambuf_iterator<T1, T2> &a0,
                                     T1 a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::ostreambuf_iterator<T1, T2> &f4(std::ostreambuf_iterator<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
std::ostreambuf_iterator<T1, T2> &f5(std::ostreambuf_iterator<T1, T2> &a0,
                                     int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
std::ostreambuf_iterator<T1, T2> &f6(std::ostreambuf_iterator<T1, T2> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2>
bool f7(const std::ostreambuf_iterator<T1, T2> &a0) {
  return a0.failed();
}

template <typename T1, typename T2> bool f8(bool a0) {
  return std::ostreambuf_iterator<T1, T2>::ostream_type::sync_with_stdio(a0);
}

template <typename T1, typename T2> int f9() {
  return std::ostreambuf_iterator<T1, T2>::ostream_type::xalloc();
}
