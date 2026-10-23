#include <algorithm>
#include <deque>
#include <utility>

template <typename T1, typename T2>
std::deque<T1, T2> f50(std::size_t a0, const T2 &a1) {
  return std::deque<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::deque<T1, T2> f51(std::size_t a0, const T1 &a1, const T2 &a2) {
  return std::deque<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
void f52(std::deque<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
std::size_t f53(const std::deque<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
std::size_t f54(const std::deque<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
void f55(std::deque<T1, T2> &a0, std::size_t a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f56(std::deque<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::reference f57(std::deque<T1, T2> &a0,
                                           std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_reference f58(const std::deque<T1, T2> &a0,
                                                 std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::reference f59(std::deque<T1, T2> &a0,
                                           std::size_t a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_reference f60(const std::deque<T1, T2> &a0,
                                                 std::size_t a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::reference f61(std::deque<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_reference f62(const std::deque<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::reference f63(std::deque<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_reference f64(const std::deque<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::iterator
f65(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
    std::size_t a2, const T1 &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
std::deque<T1, T2> f66(T3 a0, T3 a1, const T2 &a2) {
  return std::deque<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
void f67(std::deque<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
typename std::deque<T1, T2>::iterator
f68(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
    T3 a2, T3 a3) {
  return a0.template insert<T3>(a1, a2, a3);
}

template <typename T1, typename T2>
using t21 = typename std::deque<T1, T2>::iterator::size_type;

template <typename T1, typename T2>
using t22 = typename std::deque<T1, T2>::const_iterator::size_type;
