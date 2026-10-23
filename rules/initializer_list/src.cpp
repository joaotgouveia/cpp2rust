#include <cstddef>
#include <initializer_list>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> const T1 *f1(std::initializer_list<T1> a0) {
  return std::begin<T1>(a0);
}

template <typename T1> const T1 *f2(std::initializer_list<T1> a0) {
  return std::end<T1>(a0);
}

template <typename T1> using t1 = std::initializer_list<T1>;

template <typename T1>
using t2 = typename std::initializer_list<T1>::value_type;

template <typename T1> using t3 = typename std::initializer_list<T1>::reference;

template <typename T1>
using t4 = typename std::initializer_list<T1>::const_reference;

template <typename T1> using t5 = typename std::initializer_list<T1>::size_type;

template <typename T1> using t6 = typename std::initializer_list<T1>::iterator;

template <typename T1>
using t7 = typename std::initializer_list<T1>::const_iterator;

template <typename T1> std::initializer_list<T1> f3() {
  return std::initializer_list<T1>();
}

template <typename T1> const T1 *f4(const std::initializer_list<T1> &a0) {
  return a0.begin();
}

template <typename T1> const T1 *f5(const std::initializer_list<T1> &a0) {
  return a0.end();
}
