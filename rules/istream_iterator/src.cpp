#include "../common/init.h"
#include "../common/traits.h"
#include <iterator>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
using t1 = std::istream_iterator<T1, T2, T3, T4>;

template <typename T1, typename T2, typename T3, typename T4>
using t2 = typename std::istream_iterator<T1, T2, T3, T4>::char_type;

template <typename T1, typename T2, typename T3, typename T4>
using t3 = typename std::istream_iterator<T1, T2, T3, T4>::traits_type;

template <typename T1, typename T2, typename T3, typename T4>
using t4 = typename std::istream_iterator<T1, T2, T3, T4>::istream_type;

template <typename T1, typename T2, typename T3, typename T4>
std::istream_iterator<T1, T2, T3, T4> f1() {
  return std::istream_iterator<T1, T2, T3, T4>();
}

template <typename T1, typename T2, typename T3, typename T4>
std::istream_iterator<T1, T2, T3, T4>
f2(typename std::istream_iterator<T1, T2, T3, T4>::istream_type &a0) {
  return std::istream_iterator<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::istream_iterator<T1, T2, T3, T4>
f3(const std::istream_iterator<T1, T2, T3, T4> &a0) {
  return std::istream_iterator<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::istream_iterator<T1, T2, T3, T4> &
f4(std::istream_iterator<T1, T2, T3, T4> &a0,
   const std::istream_iterator<T1, T2, T3, T4> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
const T1 &f5(const std::istream_iterator<T1, T2, T3, T4> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, typename T3, typename T4>
const T1 *f6(const std::istream_iterator<T1, T2, T3, T4> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2, typename T3, typename T4>
std::istream_iterator<T1, T2, T3, T4> &
f7(std::istream_iterator<T1, T2, T3, T4> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2, typename T3, typename T4>
std::istream_iterator<T1, T2, T3, T4>
f8(std::istream_iterator<T1, T2, T3, T4> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f9(const std::istream_iterator<T1, T2, T3, T4> &a0,
        const std::istream_iterator<T1, T2, T3, T4> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3, typename T4>
using t5 =
    typename std::istream_iterator<T1, T2, T3, T4>::istream_type::char_type;

template <typename T1, typename T2, typename T3, typename T4>
using t6 =
    typename std::istream_iterator<T1, T2, T3, T4>::istream_type::int_type;

template <typename T1, typename T2, typename T3, typename T4>
using t7 =
    typename std::istream_iterator<T1, T2, T3, T4>::istream_type::pos_type;

template <typename T1, typename T2, typename T3, typename T4>
using t8 =
    typename std::istream_iterator<T1, T2, T3, T4>::istream_type::off_type;

template <typename T1, typename T2, typename T3, typename T4>
using t9 =
    typename std::istream_iterator<T1, T2, T3, T4>::istream_type::traits_type;

template <typename T1, typename T2, typename T3, typename T4>
using t10 =
    typename std::istream_iterator<T1, T2, T3, T4>::istream_type::sentry;

template <typename T1, typename T2, typename T3, typename T4>
bool f10(bool a0) {
  return std::istream_iterator<T1, T2, T3, T4>::istream_type::sync_with_stdio(
      a0);
}

template <typename T1, typename T2, typename T3, typename T4> int f11() {
  return std::istream_iterator<T1, T2, T3, T4>::istream_type::xalloc();
}
