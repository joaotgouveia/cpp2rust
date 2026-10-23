#include <algorithm>
#include <list>
#include <utility>

template <typename T1, typename T2>
std::list<T1, T2> f66(std::size_t a0, const T2 &a1) {
  return std::list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::list<T1, T2> f67(std::size_t a0, const T1 &a1, const T2 &a2) {
  return std::list<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
void f68(std::list<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
std::size_t f69(const std::list<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
std::size_t f70(const std::list<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
void f71(std::list<T1, T2> &a0, std::size_t a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f72(std::list<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2>
typename std::list<T1, T2>::reference f73(std::list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_reference f74(const std::list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::reference f75(std::list<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_reference f76(const std::list<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator
f77(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
    std::size_t a2, const T1 &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
std::list<T1, T2> f78(T3 a0, T3 a1, const T2 &a2) {
  return std::list<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
void f79(std::list<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
typename std::list<T1, T2>::iterator
f80(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1, T3 a2,
    T3 a3) {
  return a0.template insert<T3>(a1, a2, a3);
}
