#include <initializer_list>
#include <list>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
std::list<T1, T2> f74(typename std::list<T1, T2>::size_type a0,
                      const typename std::list<T1, T2>::value_type &a1,
                      const typename std::list<T1, T2>::allocator_type &a2) {
  return std::list<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
typename std::list<T1, T2>::reference f75(std::list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_reference f76(const std::list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::reference f77(std::list<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_reference f78(const std::list<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
void f79(std::list<T1, T2> &a0, const T1 &a1) {
  return a0.remove(a1);
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T3>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
std::list<T1, T2> f80(T3 a0, T3 a1,
                      const typename std::list<T1, T2>::allocator_type &a2) {
  return std::list<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T3>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
void f81(std::list<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T3>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
typename std::list<T1, T2>::iterator
f82(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1, T3 a2,
    T3 a3) {
  return a0.template insert<T3>(a1, a2, a3);
}
