#include "../common/init.h"
#include "../common/traits.h"
#include <deque>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2>
std::deque<T1, T2> f60(typename std::deque<T1, T2>::size_type a0,
                       const typename std::deque<T1, T2>::allocator_type &a1) {
  return std::deque<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::reference
f61(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_reference
f62(const std::deque<T1, T2> &a0, typename std::deque<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::reference
f63(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_reference
f64(const std::deque<T1, T2> &a0, typename std::deque<T1, T2>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::reference f65(std::deque<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_reference f66(const std::deque<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::reference f67(std::deque<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_reference f68(const std::deque<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
using t29 = typename std::deque<T1, T2>::iterator::size_type;

template <typename T1, typename T2>
using t30 = typename std::deque<T1, T2>::const_iterator::size_type;
