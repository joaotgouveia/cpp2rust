#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f85(typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a0,
    const T3 &a1, const T4 &a2) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f86(typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a0,
    const T3 &a1, const T4 &a2, const T5 &a3) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5> f87(const T5 &a0) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f88(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T5 &a1) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f89(std::unordered_multimap<T1, T2, T3, T4, T5> &&a0, const T5 &a1) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f90(std::initializer_list<std::pair<const T1, T2>> a0) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f91(std::initializer_list<std::pair<const T1, T2>> a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1,
    const T3 &a2, const T4 &a3) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f92(std::initializer_list<std::pair<const T1, T2>> a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1,
    const T3 &a2, const T4 &a3, const T5 &a4) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f93(typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a0,
    const T5 &a1) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f94(typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a0,
    const T3 &a1, const T5 &a2) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f95(std::initializer_list<std::pair<const T1, T2>> a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1,
    const T5 &a2) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f96(std::initializer_list<std::pair<const T1, T2>> a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1,
    const T3 &a2, const T5 &a3) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5> &
f97(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    std::initializer_list<std::pair<const T1, T2>> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T5 f98(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f99(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    const std::pair<const T1, T2> &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f100(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
     typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
     const std::pair<const T1, T2> &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f101(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
          std::initializer_list<std::pair<const T1, T2>> a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f102(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
     std::pair<const T1, T2> &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f103(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
     typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
     std::pair<const T1, T2> &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f104(std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T3 f105(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.hash_function();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T4 f106(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.key_eq();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f107(std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator
f108(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f109(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator,
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator>
f110(std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator,
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator>
f111(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f112(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.bucket(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_multimap<T1, T2, T3, T4, T5> f113(T6 a0, T6 a1) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_multimap<T1, T2, T3, T4, T5>
f114(T6 a0, T6 a1,
     typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a2,
     const T3 &a3, const T4 &a4) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_multimap<T1, T2, T3, T4, T5>
f115(T6 a0, T6 a1,
     typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a2,
     const T3 &a3, const T4 &a4, const T5 &a5) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_multimap<T1, T2, T3, T4, T5>
f116(T6 a0, T6 a1,
     typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a2,
     const T5 &a3) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_multimap<T1, T2, T3, T4, T5>
f117(T6 a0, T6 a1,
     typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a2,
     const T3 &a3, const T5 &a4) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2, a3, a4);
}
