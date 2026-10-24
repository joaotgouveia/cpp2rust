#include <algorithm>
#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f52(typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::hasher &a1,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_equal &a2,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a3) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f53(const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a0) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f54(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a1) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f55(std::unordered_map<T1, T2, T3, T4, T5> &&a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a1) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f56(std::initializer_list<
        typename std::unordered_map<T1, T2, T3, T4, T5>::value_type>
        a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::hasher &a2,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_equal &a3,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a4) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                                a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f57(typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a1) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f58(typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::hasher &a1,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a2) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f59(std::initializer_list<
        typename std::unordered_map<T1, T2, T3, T4, T5>::value_type>
        a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a2) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                                a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f60(std::initializer_list<
        typename std::unordered_map<T1, T2, T3, T4, T5>::value_type>
        a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::hasher &a2,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a3) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                                a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5> &
f61(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    std::initializer_list<
        typename std::unordered_map<T1, T2, T3, T4, T5>::value_type>
        a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type
f62(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f63(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::value_type &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f64(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::value_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f65(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::value_type &a2) {
  return a0.insert(std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f66(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::unordered_map<T1, T2, T3, T4, T5>::value_type &&a2) {
  return a0.insert(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f67(std::unordered_map<T1, T2, T3, T4, T5> &a0,
         std::initializer_list<
             typename std::unordered_map<T1, T2, T3, T4, T5>::value_type>
             a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f68(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::hasher
f69(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.hash_function();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::key_equal
f70(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.key_eq();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f71(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator
f72(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f73(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator,
          typename std::unordered_map<T1, T2, T3, T4, T5>::iterator>
f74(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator,
          typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator>
f75(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::mapped_type &
f76(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::mapped_type &
f77(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &&a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::mapped_type &
f78(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
const typename std::unordered_map<T1, T2, T3, T4, T5>::mapped_type &
f79(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f80(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.bucket(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5>
f81(T6 a0, T6 a1, typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a2,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::hasher &a3,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_equal &a4,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a5) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                                std::move(a2), a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5>
f82(T6 a0, T6 a1, typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a2,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a3) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                                std::move(a2), a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5>
f83(T6 a0, T6 a1, typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a2,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::hasher &a3,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a4) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                                std::move(a2), a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t22 =
    typename std::unordered_map<T1, T2, T3, T4, T5>::iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t23 =
    typename std::unordered_map<T1, T2, T3, T4, T5>::iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t24 = typename std::unordered_map<T1, T2, T3, T4,
                                        T5>::iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t25 = typename std::unordered_map<T1, T2, T3, T4, T5>::iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t26 =
    typename std::unordered_map<T1, T2, T3, T4, T5>::iterator::reference;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t27 =
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t28 = typename std::unordered_map<T1, T2, T3, T4,
                                        T5>::const_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t29 = typename std::unordered_map<T1, T2, T3, T4,
                                        T5>::const_iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t30 =
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t31 =
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t32 =
    typename std::unordered_map<T1, T2, T3, T4, T5>::local_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t33 =
    typename std::unordered_map<T1, T2, T3, T4, T5>::local_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t34 =
    typename std::unordered_map<T1, T2, T3, T4, T5>::local_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t35 = typename std::unordered_map<T1, T2, T3, T4,
                                        T5>::local_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t36 = typename std::unordered_map<T1, T2, T3, T4,
                                        T5>::local_iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t37 = typename std::unordered_map<T1, T2, T3, T4,
                                        T5>::const_local_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t38 = typename std::unordered_map<T1, T2, T3, T4,
                                        T5>::const_local_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t39 = typename std::unordered_map<T1, T2, T3, T4,
                                        T5>::const_local_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t40 =
    typename std::unordered_map<T1, T2, T3, T4,
                                T5>::const_local_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t41 =
    typename std::unordered_map<T1, T2, T3, T4,
                                T5>::const_local_iterator::iterator_category;
