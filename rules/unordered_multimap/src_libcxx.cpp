#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f91(typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a0,
    const T3 &a1, const T4 &a2) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type>(a0),
      a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f92(typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a0,
    const T3 &a1, const T4 &a2, const T5 &a3) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type>(a0),
      a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5> f93(const T5 &a0) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f94(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T5 &a1) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f95(std::unordered_multimap<T1, T2, T3, T4, T5> &&a0, const T5 &a1) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f96(std::initializer_list<std::pair<const T1, T2>> a0) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::forward<std::initializer_list<std::pair<const T1, T2>>>(a0));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f97(std::initializer_list<std::pair<const T1, T2>> a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1,
    const T3 &a2, const T4 &a3) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::forward<std::initializer_list<std::pair<const T1, T2>>>(a0),
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type>(a1),
      a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f98(std::initializer_list<std::pair<const T1, T2>> a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1,
    const T3 &a2, const T4 &a3, const T5 &a4) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::forward<std::initializer_list<std::pair<const T1, T2>>>(a0),
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type>(a1),
      a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f99(typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a0,
    const T5 &a1) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type>(a0),
      a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f100(typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a0,
     const T3 &a1, const T5 &a2) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type>(a0),
      a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f101(std::initializer_list<std::pair<const T1, T2>> a0,
     typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1,
     const T5 &a2) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::forward<std::initializer_list<std::pair<const T1, T2>>>(a0),
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type>(a1),
      a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f102(std::initializer_list<std::pair<const T1, T2>> a0,
     typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1,
     const T3 &a2, const T5 &a3) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::forward<std::initializer_list<std::pair<const T1, T2>>>(a0),
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type>(a1),
      a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5> &
f103(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
     std::initializer_list<std::pair<const T1, T2>> a1) {
  return a0.operator=(
      std::forward<std::initializer_list<std::pair<const T1, T2>>>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T5 f104(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f105(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
     const std::pair<const T1, T2> &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f106(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
     typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
     const std::pair<const T1, T2> &a2) {
  return a0.insert(
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator>(
          a1),
      a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f107(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
          std::initializer_list<std::pair<const T1, T2>> a1) {
  return a0.insert(
      std::forward<std::initializer_list<std::pair<const T1, T2>>>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f108(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
     std::pair<const T1, T2> &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f109(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
     typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
     std::pair<const T1, T2> &&a2) {
  return a0.insert(
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator>(
          a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f110(std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T3 f111(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.hash_function();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T4 f112(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.key_eq();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f113(std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator
f114(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f115(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator,
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator>
f116(std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator,
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator>
f117(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f118(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.bucket(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_multimap<T1, T2, T3, T4, T5> f119(T6 a0, T6 a1) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(std::move(a0),
                                                     std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_multimap<T1, T2, T3, T4, T5>
f120(T6 a0, T6 a1,
     typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a2,
     const T3 &a3, const T4 &a4) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::move(a0), std::move(a1),
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type>(a2),
      a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_multimap<T1, T2, T3, T4, T5>
f121(T6 a0, T6 a1,
     typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a2,
     const T3 &a3, const T4 &a4, const T5 &a5) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::move(a0), std::move(a1),
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type>(a2),
      a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_multimap<T1, T2, T3, T4, T5>
f122(T6 a0, T6 a1,
     typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a2,
     const T5 &a3) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::move(a0), std::move(a1),
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type>(a2),
      a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_multimap<T1, T2, T3, T4, T5>
f123(T6 a0, T6 a1,
     typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a2,
     const T3 &a3, const T5 &a4) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::move(a0), std::move(a1),
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type>(a2),
      a3, a4);
}
