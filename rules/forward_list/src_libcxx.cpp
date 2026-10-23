#include <forward_list>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2>
std::forward_list<T1, T2>
f83(typename std::forward_list<T1, T2>::size_type a0) {
  return std::forward_list<T1, T2>(a0);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f84(typename std::forward_list<T1, T2>::size_type a0,
                              const T2 &a1) {
  return std::forward_list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f85(typename std::forward_list<T1, T2>::size_type a0,
                              const T1 &a1) {
  return std::forward_list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f86(std::initializer_list<T1> a0) {
  return std::forward_list<T1, T2>(a0);
}

template <typename T1, typename T2>
void f87(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::size_type a1, const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::size_type
f88(const std::forward_list<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f89(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1,
    typename std::forward_list<T1, T2>::size_type a2, const T1 &a3) {
  return a0.insert_after(a1, a2, a3);
}

template <typename T1, typename T2>
void f90(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::size_type a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f91(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::size_type a1, const T1 &a2) {
  return a0.resize(a1, a2);
}
