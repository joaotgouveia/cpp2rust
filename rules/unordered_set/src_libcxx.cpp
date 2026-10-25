#include <initializer_list>
#include <unordered_set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f93(typename std::unordered_set<T1, T2, T3, T4>::size_type a0, const T2 &a1,
    const T3 &a2) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a0),
      a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f94(typename std::unordered_set<T1, T2, T3, T4>::size_type a0, const T4 &a1) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a0),
      a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f95(typename std::unordered_set<T1, T2, T3, T4>::size_type a0, const T2 &a1,
    const T4 &a2) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a0),
      a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f96(typename std::unordered_set<T1, T2, T3, T4>::size_type a0, const T2 &a1,
    const T3 &a2, const T4 &a3) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a0),
      a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4> f97(const T4 &a0) {
  return std::unordered_set<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f98(const std::unordered_set<T1, T2, T3, T4> &a0, const T4 &a1) {
  return std::unordered_set<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4> f99(std::unordered_set<T1, T2, T3, T4> &&a0,
                                       const T4 &a1) {
  return std::unordered_set<T1, T2, T3, T4>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4> f100(std::initializer_list<T1> a0) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::forward<std::initializer_list<T1>>(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f101(std::initializer_list<T1> a0,
     typename std::unordered_set<T1, T2, T3, T4>::size_type a1, const T2 &a2,
     const T3 &a3) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::forward<std::initializer_list<T1>>(a0),
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a1),
      a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f102(std::initializer_list<T1> a0,
     typename std::unordered_set<T1, T2, T3, T4>::size_type a1, const T2 &a2,
     const T3 &a3, const T4 &a4) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::forward<std::initializer_list<T1>>(a0),
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a1),
      a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f103(std::initializer_list<T1> a0,
     typename std::unordered_set<T1, T2, T3, T4>::size_type a1, const T4 &a2) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::forward<std::initializer_list<T1>>(a0),
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a1),
      a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f104(std::initializer_list<T1> a0,
     typename std::unordered_set<T1, T2, T3, T4>::size_type a1, const T2 &a2,
     const T4 &a3) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::forward<std::initializer_list<T1>>(a0),
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a1),
      a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4> &f105(std::unordered_set<T1, T2, T3, T4> &a0,
                                         std::initializer_list<T1> a1) {
  return a0.operator=(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
T4 f106(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::unordered_set<T1, T2, T3, T4>::iterator, bool>
f107(std::unordered_set<T1, T2, T3, T4> &a0, T1 &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f108(std::unordered_set<T1, T2, T3, T4> &a0,
     typename std::unordered_set<T1, T2, T3, T4>::const_iterator a1, T1 &&a2) {
  return a0.insert(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::const_iterator>(
          a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
void f109(std::unordered_set<T1, T2, T3, T4> &a0,
          std::initializer_list<T1> a1) {
  return a0.insert(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::unordered_set<T1, T2, T3, T4>::iterator, bool>
f110(std::unordered_set<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f111(std::unordered_set<T1, T2, T3, T4> &a0,
     typename std::unordered_set<T1, T2, T3, T4>::const_iterator a1,
     const T1 &a2) {
  return a0.insert(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::const_iterator>(
          a1),
      a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::size_type
f112(std::unordered_set<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
T2 f113(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.hash_function();
}

template <typename T1, typename T2, typename T3, typename T4>
T3 f114(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.key_eq();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f115(std::unordered_set<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::const_iterator
f116(const std::unordered_set<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::size_type
f117(const std::unordered_set<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::unordered_set<T1, T2, T3, T4>::iterator,
          typename std::unordered_set<T1, T2, T3, T4>::iterator>
f118(std::unordered_set<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::unordered_set<T1, T2, T3, T4>::const_iterator,
          typename std::unordered_set<T1, T2, T3, T4>::const_iterator>
f119(const std::unordered_set<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::size_type
f120(const std::unordered_set<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.bucket(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_set<T1, T2, T3, T4> f121(T5 a0, T5 a1) {
  return std::unordered_set<T1, T2, T3, T4>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_set<T1, T2, T3, T4>
f122(T5 a0, T5 a1, typename std::unordered_set<T1, T2, T3, T4>::size_type a2,
     const T2 &a3, const T3 &a4) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::move(a0), std::move(a1),
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a2),
      a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_set<T1, T2, T3, T4>
f123(T5 a0, T5 a1, typename std::unordered_set<T1, T2, T3, T4>::size_type a2,
     const T2 &a3, const T3 &a4, const T4 &a5) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::move(a0), std::move(a1),
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a2),
      a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_set<T1, T2, T3, T4>
f124(T5 a0, T5 a1, typename std::unordered_set<T1, T2, T3, T4>::size_type a2,
     const T4 &a3) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::move(a0), std::move(a1),
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a2),
      a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_set<T1, T2, T3, T4>
f125(T5 a0, T5 a1, typename std::unordered_set<T1, T2, T3, T4>::size_type a2,
     const T2 &a3, const T4 &a4) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::move(a0), std::move(a1),
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a2),
      a3, a4);
}
