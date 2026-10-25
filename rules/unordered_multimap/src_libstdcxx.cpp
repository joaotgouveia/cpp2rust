#include <algorithm>
#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f44(typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::hasher &a1,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_equal &a2,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
        &a3) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type>(a0),
      a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f45(const typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
        &a0) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f46(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
        &a1) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f47(std::unordered_multimap<T1, T2, T3, T4, T5> &&a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
        &a1) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f48(std::initializer_list<
        typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type>
        a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::hasher &a2,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_equal &a3,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
        &a4) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::forward<std::initializer_list<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type>>(
          a0),
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type>(a1),
      a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f49(typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
        &a1) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type>(a0),
      a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f50(typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::hasher &a1,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
        &a2) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type>(a0),
      a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f51(std::initializer_list<
        typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type>
        a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
        &a2) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::forward<std::initializer_list<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type>>(
          a0),
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type>(a1),
      a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f52(std::initializer_list<
        typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type>
        a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::hasher &a2,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
        &a3) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::forward<std::initializer_list<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type>>(
          a0),
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type>(a1),
      a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5> &
f53(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    std::initializer_list<
        typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type>
        a1) {
  return a0.operator=(
      std::forward<std::initializer_list<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type>>(
          a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
f54(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f55(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type
        &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f56(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f57(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type
        &a2) {
  return a0.insert(
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator>(
          a1),
      a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f58(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type &&a2) {
  return a0.insert(
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator>(
          a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f59(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
         std::initializer_list<
             typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type>
             a1) {
  return a0.insert(
      std::forward<std::initializer_list<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type>>(
          a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f60(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::hasher
f61(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.hash_function();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_equal
f62(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.key_eq();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f63(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator
f64(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f65(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator,
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator>
f66(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator,
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator>
f67(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f68(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.bucket(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_multimap<T1, T2, T3, T4, T5>
f69(T6 a0, T6 a1,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a2,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::hasher &a3,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_equal &a4,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
        &a5) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::move(a0), std::move(a1),
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type>(a2),
      a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_multimap<T1, T2, T3, T4, T5>
f70(T6 a0, T6 a1,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a2,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
        &a3) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::move(a0), std::move(a1),
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type>(a2),
      a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_multimap<T1, T2, T3, T4, T5>
f71(T6 a0, T6 a1,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a2,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::hasher &a3,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
        &a4) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::move(a0), std::move(a1),
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type>(a2),
      a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t19 =
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t20 = typename std::unordered_multimap<T1, T2, T3, T4,
                                             T5>::iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t21 = typename std::unordered_multimap<T1, T2, T3, T4,
                                             T5>::iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t22 =
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t23 =
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator::reference;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t24 = typename std::unordered_multimap<T1, T2, T3, T4,
                                             T5>::const_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t25 =
    typename std::unordered_multimap<T1, T2, T3, T4,
                                     T5>::const_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t26 =
    typename std::unordered_multimap<T1, T2, T3, T4,
                                     T5>::const_iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t27 = typename std::unordered_multimap<T1, T2, T3, T4,
                                             T5>::const_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t28 = typename std::unordered_multimap<T1, T2, T3, T4,
                                             T5>::const_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t29 = typename std::unordered_multimap<T1, T2, T3, T4,
                                             T5>::local_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t30 = typename std::unordered_multimap<T1, T2, T3, T4,
                                             T5>::local_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t31 = typename std::unordered_multimap<T1, T2, T3, T4,
                                             T5>::local_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t32 =
    typename std::unordered_multimap<T1, T2, T3, T4,
                                     T5>::local_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t33 =
    typename std::unordered_multimap<T1, T2, T3, T4,
                                     T5>::local_iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t34 =
    typename std::unordered_multimap<T1, T2, T3, T4,
                                     T5>::const_local_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t35 = typename std::unordered_multimap<T1, T2, T3, T4,
                                             T5>::const_local_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t36 =
    typename std::unordered_multimap<T1, T2, T3, T4,
                                     T5>::const_local_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t37 =
    typename std::unordered_multimap<T1, T2, T3, T4,
                                     T5>::const_local_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t38 = typename std::unordered_multimap<
    T1, T2, T3, T4, T5>::const_local_iterator::iterator_category;
