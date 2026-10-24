#include <algorithm>
#include <deque>
#include <utility>

template <typename T1, typename T2>
std::deque<T1, T2> f45(std::size_t a0, const T2 &a1) {
  return std::deque<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::deque<T1, T2> f46(std::size_t a0, const T1 &a1, const T2 &a2) {
  return std::deque<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
void f47(std::deque<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
std::size_t f48(const std::deque<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
std::size_t f49(const std::deque<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
void f50(std::deque<T1, T2> &a0, std::size_t a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f51(std::deque<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::reference f52(std::deque<T1, T2> &a0,
                                           std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_reference f53(const std::deque<T1, T2> &a0,
                                                 std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::reference f54(std::deque<T1, T2> &a0,
                                           std::size_t a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_reference f55(const std::deque<T1, T2> &a0,
                                                 std::size_t a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::reference f56(std::deque<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_reference f57(const std::deque<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::reference f58(std::deque<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_reference f59(const std::deque<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::iterator
f60(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
    std::size_t a2, const T1 &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
std::deque<T1, T2> f61(T3 a0, T3 a1, const T2 &a2) {
  return std::deque<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
void f62(std::deque<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
typename std::deque<T1, T2>::iterator
f63(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
    T3 a2, T3 a3) {
  return a0.template insert<T3>(a1, a2, a3);
}

template <typename T1, typename T2>
using t25 = typename std::deque<T1, T2>::iterator::iterator;

template <typename T1, typename T2>
using t26 = typename std::deque<T1, T2>::iterator::const_iterator;

template <typename T1, typename T2>
using t27 = typename std::deque<T1, T2>::iterator::size_type;

template <typename T1, typename T2>
using t28 = typename std::deque<T1, T2>::const_iterator::iterator;

template <typename T1, typename T2>
using t29 = typename std::deque<T1, T2>::const_iterator::const_iterator;

template <typename T1, typename T2>
using t30 = typename std::deque<T1, T2>::const_iterator::size_type;
