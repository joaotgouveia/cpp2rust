#include <forward_list>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2>
std::forward_list<T1, T2>
f83(const typename std::forward_list<T1, T2>::allocator_type &a0) {
  return std::forward_list<T1, T2>(a0);
}

template <typename T1, typename T2>
std::forward_list<T1, T2>
f84(typename std::forward_list<T1, T2>::size_type a0) {
  return std::forward_list<T1, T2>(a0);
}

template <typename T1, typename T2>
std::forward_list<T1, T2>
f85(typename std::forward_list<T1, T2>::size_type a0,
    const typename std::forward_list<T1, T2>::allocator_type &a1) {
  return std::forward_list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2>
f86(typename std::forward_list<T1, T2>::size_type a0,
    const typename std::forward_list<T1, T2>::value_type &a1) {
  return std::forward_list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2>
f87(const std::forward_list<T1, T2> &a0,
    const typename std::forward_list<T1, T2>::allocator_type &a1) {
  return std::forward_list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2>
f88(std::forward_list<T1, T2> &&a0,
    const typename std::forward_list<T1, T2>::allocator_type &a1) {
  return std::forward_list<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2>
f89(std::initializer_list<typename std::forward_list<T1, T2>::value_type> a0) {
  return std::forward_list<T1, T2>(a0);
}

template <typename T1, typename T2>
std::forward_list<T1, T2>
f90(std::initializer_list<typename std::forward_list<T1, T2>::value_type> a0,
    const typename std::forward_list<T1, T2>::allocator_type &a1) {
  return std::forward_list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> &
f91(std::forward_list<T1, T2> &a0,
    std::initializer_list<typename std::forward_list<T1, T2>::value_type> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
void f92(
    std::forward_list<T1, T2> &a0,
    std::initializer_list<typename std::forward_list<T1, T2>::value_type> a1) {
  return a0.assign(a1);
}

template <typename T1, typename T2>
void f93(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::size_type a1,
         const typename std::forward_list<T1, T2>::value_type &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
void f94(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::value_type &&a1) {
  return a0.push_front(std::move(a1));
}

template <typename T1, typename T2>
void f95(std::forward_list<T1, T2> &a0,
         const typename std::forward_list<T1, T2>::value_type &a1) {
  return a0.push_front(a1);
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f96(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1,
    typename std::forward_list<T1, T2>::value_type &&a2) {
  return a0.insert_after(a1, std::move(a2));
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f97(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1,
    std::initializer_list<typename std::forward_list<T1, T2>::value_type> a2) {
  return a0.insert_after(a1, a2);
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f98(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1,
    const typename std::forward_list<T1, T2>::value_type &a2) {
  return a0.insert_after(a1, a2);
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f99(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1,
    typename std::forward_list<T1, T2>::size_type a2,
    const typename std::forward_list<T1, T2>::value_type &a3) {
  return a0.insert_after(a1, a2, a3);
}
