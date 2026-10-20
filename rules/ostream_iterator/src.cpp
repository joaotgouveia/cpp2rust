#include "../common/init.h"
#include "../common/traits.h"
#include <iterator>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::ostream_iterator<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = typename std::ostream_iterator<T1, T2, T3>::char_type;

template <typename T1, typename T2, typename T3>
using t3 = typename std::ostream_iterator<T1, T2, T3>::traits_type;

template <typename T1, typename T2, typename T3>
using t4 = typename std::ostream_iterator<T1, T2, T3>::ostream_type;

template <typename T1, typename T2, typename T3>
std::ostream_iterator<T1, T2, T3>
f1(typename std::ostream_iterator<T1, T2, T3>::ostream_type &a0) {
  return std::ostream_iterator<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::ostream_iterator<T1, T2, T3>
f2(typename std::ostream_iterator<T1, T2, T3>::ostream_type &a0, const T2 *a1) {
  return std::ostream_iterator<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::ostream_iterator<T1, T2, T3>
f3(const std::ostream_iterator<T1, T2, T3> &a0) {
  return std::ostream_iterator<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::ostream_iterator<T1, T2, T3> &
f4(std::ostream_iterator<T1, T2, T3> &a0,
   const std::ostream_iterator<T1, T2, T3> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::ostream_iterator<T1, T2, T3> &f5(std::ostream_iterator<T1, T2, T3> &a0,
                                      const T1 &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::ostream_iterator<T1, T2, T3> &f6(std::ostream_iterator<T1, T2, T3> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, typename T3>
std::ostream_iterator<T1, T2, T3> &f7(std::ostream_iterator<T1, T2, T3> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2, typename T3>
std::ostream_iterator<T1, T2, T3> &f8(std::ostream_iterator<T1, T2, T3> &a0,
                                      int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2, typename T3>
using t5 = typename std::ostream_iterator<T1, T2, T3>::ostream_type::char_type;

template <typename T1, typename T2, typename T3>
using t6 = typename std::ostream_iterator<T1, T2, T3>::ostream_type::int_type;

template <typename T1, typename T2, typename T3>
using t7 = typename std::ostream_iterator<T1, T2, T3>::ostream_type::pos_type;

template <typename T1, typename T2, typename T3>
using t8 = typename std::ostream_iterator<T1, T2, T3>::ostream_type::off_type;

template <typename T1, typename T2, typename T3>
using t9 =
    typename std::ostream_iterator<T1, T2, T3>::ostream_type::traits_type;

template <typename T1, typename T2, typename T3>
using t10 = typename std::ostream_iterator<T1, T2, T3>::ostream_type::sentry;

template <typename T1, typename T2, typename T3> bool f9(bool a0) {
  return
      typename std::ostream_iterator<T1, T2, T3>::ostream_type::sync_with_stdio(
          a0);
}

template <typename T1, typename T2, typename T3> int f10() {
  return typename std::ostream_iterator<T1, T2, T3>::ostream_type::xalloc();
}
