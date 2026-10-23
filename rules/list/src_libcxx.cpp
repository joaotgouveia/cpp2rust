#include <initializer_list>
#include <list>
#include <utility>

template <typename T1, typename T2>
bool f93(const std::list<T1, T2> &a0, const std::list<T1, T2> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
void f94(std::list<T1, T2> &a0, std::list<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::list<T1, T2> f95(typename std::list<T1, T2>::size_type a0) {
  return std::list<T1, T2>(a0);
}

template <typename T1, typename T2>
std::list<T1, T2> f96(typename std::list<T1, T2>::size_type a0,
                      const typename std::list<T1, T2>::value_type &a1) {
  return std::list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::list<T1, T2>
f97(std::initializer_list<typename std::list<T1, T2>::value_type> a0) {
  return std::list<T1, T2>(a0);
}

template <typename T1, typename T2>
typename std::list<T1, T2>::value_type &f98(std::list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
const typename std::list<T1, T2>::value_type &f99(const std::list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::value_type &f100(std::list<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
const typename std::list<T1, T2>::value_type &
f101(const std::list<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
using t23 = typename std::list<T1, T2>::iterator::iterator_category;

template <typename T1, typename T2>
using t24 = typename std::list<T1, T2>::iterator::value_type;

template <typename T1, typename T2>
using t25 = typename std::list<T1, T2>::iterator::reference;

template <typename T1, typename T2>
using t26 = typename std::list<T1, T2>::iterator::pointer;

template <typename T1, typename T2>
using t27 = typename std::list<T1, T2>::iterator::difference_type;

template <typename T1, typename T2>
using t28 = typename std::list<T1, T2>::const_iterator::iterator_category;

template <typename T1, typename T2>
using t29 = typename std::list<T1, T2>::const_iterator::value_type;

template <typename T1, typename T2>
using t30 = typename std::list<T1, T2>::const_iterator::reference;

template <typename T1, typename T2>
using t31 = typename std::list<T1, T2>::const_iterator::pointer;

template <typename T1, typename T2>
using t32 = typename std::list<T1, T2>::const_iterator::difference_type;

template <typename T1, typename T2>
using t33 = typename std::list<T1, T2>::reverse_iterator::iterator_category;

template <typename T1, typename T2>
using t34 = typename std::list<T1, T2>::reverse_iterator::value_type;

template <typename T1, typename T2>
using t35 =
    typename std::list<T1, T2>::const_reverse_iterator::iterator_category;

template <typename T1, typename T2>
using t36 = typename std::list<T1, T2>::const_reverse_iterator::value_type;
