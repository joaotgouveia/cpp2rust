#include <algorithm>
#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f1(std::unordered_map<T1, T2, T3, T4, T5> &a0, T6 a1) {
  return std::erase_if(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f2(std::unordered_multimap<T1, T2, T3, T4, T5> &a0, T6 a1) {
  return std::erase_if(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f3(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
        const std::unordered_map<T1, T2, T3, T4, T5> &a1) {
  return std::operator==(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f4(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
        const std::unordered_multimap<T1, T2, T3, T4, T5> &a1) {
  return std::operator==(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
using t1 = std::pmr::unordered_map<T1, T2, T3, T4>;

template <typename T1, typename T2, typename T3, typename T4>
using t2 = std::pmr::unordered_multimap<T1, T2, T3, T4>;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f5(std::unordered_map<T1, T2, T3, T4, T5> &a0,
        std::unordered_map<T1, T2, T3, T4, T5> &a1) {
  return std::swap(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f6(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
        std::unordered_multimap<T1, T2, T3, T4, T5> &a1) {
  return std::swap(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t3 = std::unordered_map<T1, T2, T3, T4, T5>;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t4 = typename std::unordered_map<T1, T2, T3, T4, T5>::key_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t5 = typename std::unordered_map<T1, T2, T3, T4, T5>::value_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t6 = typename std::unordered_map<T1, T2, T3, T4, T5>::mapped_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t7 = typename std::unordered_map<T1, T2, T3, T4, T5>::hasher;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t8 = typename std::unordered_map<T1, T2, T3, T4, T5>::key_equal;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t9 = typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t10 = typename std::unordered_map<T1, T2, T3, T4, T5>::pointer;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t11 = typename std::unordered_map<T1, T2, T3, T4, T5>::const_pointer;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t12 = typename std::unordered_map<T1, T2, T3, T4, T5>::reference;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t13 = typename std::unordered_map<T1, T2, T3, T4, T5>::const_reference;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t14 = typename std::unordered_map<T1, T2, T3, T4, T5>::iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t15 = typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t16 = typename std::unordered_map<T1, T2, T3, T4, T5>::local_iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t17 =
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_local_iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t18 = typename std::unordered_map<T1, T2, T3, T4, T5>::size_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t19 = typename std::unordered_map<T1, T2, T3, T4, T5>::difference_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t20 = typename std::unordered_map<T1, T2, T3, T4, T5>::node_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t21 = typename std::unordered_map<T1, T2, T3, T4, T5>::insert_return_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5> f7() {
  return std::unordered_map<T1, T2, T3, T4, T5>();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f8(typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a0,
   const typename std::unordered_map<T1, T2, T3, T4, T5>::hasher &a1,
   const typename std::unordered_map<T1, T2, T3, T4, T5>::key_equal &a2,
   const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a3) {
  return std::unordered_map<T1, T2, T3, T4, T5>(
      std::forward<typename std::unordered_map<T1, T2, T3, T4, T5>::size_type>(
          a0),
      a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f9(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f10(std::unordered_map<T1, T2, T3, T4, T5> &&a0) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f11(const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a0) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f12(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a1) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f13(std::unordered_map<T1, T2, T3, T4, T5> &&a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a1) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f14(std::initializer_list<
        typename std::unordered_map<T1, T2, T3, T4, T5>::value_type>
        a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::hasher &a2,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_equal &a3,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a4) {
  return std::unordered_map<T1, T2, T3, T4, T5>(
      std::forward<std::initializer_list<
          typename std::unordered_map<T1, T2, T3, T4, T5>::value_type>>(a0),
      std::forward<typename std::unordered_map<T1, T2, T3, T4, T5>::size_type>(
          a1),
      a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f15(typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a1) {
  return std::unordered_map<T1, T2, T3, T4, T5>(
      std::forward<typename std::unordered_map<T1, T2, T3, T4, T5>::size_type>(
          a0),
      a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f16(typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::hasher &a1,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a2) {
  return std::unordered_map<T1, T2, T3, T4, T5>(
      std::forward<typename std::unordered_map<T1, T2, T3, T4, T5>::size_type>(
          a0),
      a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f17(std::initializer_list<
        typename std::unordered_map<T1, T2, T3, T4, T5>::value_type>
        a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a2) {
  return std::unordered_map<T1, T2, T3, T4, T5>(
      std::forward<std::initializer_list<
          typename std::unordered_map<T1, T2, T3, T4, T5>::value_type>>(a0),
      std::forward<typename std::unordered_map<T1, T2, T3, T4, T5>::size_type>(
          a1),
      a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f18(std::initializer_list<
        typename std::unordered_map<T1, T2, T3, T4, T5>::value_type>
        a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::hasher &a2,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a3) {
  return std::unordered_map<T1, T2, T3, T4, T5>(
      std::forward<std::initializer_list<
          typename std::unordered_map<T1, T2, T3, T4, T5>::value_type>>(a0),
      std::forward<typename std::unordered_map<T1, T2, T3, T4, T5>::size_type>(
          a1),
      a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5> &
f19(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const std::unordered_map<T1, T2, T3, T4, T5> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5> &
f20(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    std::unordered_map<T1, T2, T3, T4, T5> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5> &
f21(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    std::initializer_list<
        typename std::unordered_map<T1, T2, T3, T4, T5>::value_type>
        a1) {
  return a0.operator=(
      std::forward<std::initializer_list<
          typename std::unordered_map<T1, T2, T3, T4, T5>::value_type>>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type
f22(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f23(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f24(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f25(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f26(std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator
f27(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator
f28(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f29(std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator
f30(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator
f31(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::node_type
f32(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1) {
  return a0.extract(
      std::forward<
          typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::node_type
f33(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::insert_return_type
f34(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::node_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f35(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::unordered_map<T1, T2, T3, T4, T5>::node_type &&a2) {
  return a0.insert(
      std::forward<
          typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f36(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::value_type &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f37(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::value_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f38(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::value_type &a2) {
  return a0.insert(
      std::forward<
          typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator>(a1),
      a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f39(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::unordered_map<T1, T2, T3, T4, T5>::value_type &&a2) {
  return a0.insert(
      std::forward<
          typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f40(std::unordered_map<T1, T2, T3, T4, T5> &a0,
         std::initializer_list<
             typename std::unordered_map<T1, T2, T3, T4, T5>::value_type>
             a1) {
  return a0.insert(
      std::forward<std::initializer_list<
          typename std::unordered_map<T1, T2, T3, T4, T5>::value_type>>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f41(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1) {
  return a0.erase(
      std::forward<
          typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f42(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::iterator a1) {
  return a0.erase(
      std::forward<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator>(
          a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f43(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f44(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a2) {
  return a0.erase(
      std::forward<
          typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator>(a1),
      std::forward<
          typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f45(std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f46(std::unordered_map<T1, T2, T3, T4, T5> &a0,
         std::unordered_map<T1, T2, T3, T4, T5> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::hasher
f47(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.hash_function();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::key_equal
f48(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.key_eq();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f49(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator
f50(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f51(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f52(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
         const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator,
          typename std::unordered_map<T1, T2, T3, T4, T5>::iterator>
f53(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator,
          typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator>
f54(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::mapped_type &
f55(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::mapped_type &
f56(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &&a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::mapped_type &
f57(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
const typename std::unordered_map<T1, T2, T3, T4, T5>::mapped_type &
f58(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f59(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.bucket_count();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f60(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.max_bucket_count();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f61(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.bucket_size(
      std::forward<typename std::unordered_map<T1, T2, T3, T4, T5>::size_type>(
          a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f62(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.bucket(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::local_iterator
f63(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.begin(
      std::forward<typename std::unordered_map<T1, T2, T3, T4, T5>::size_type>(
          a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_local_iterator
f64(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.begin(
      std::forward<typename std::unordered_map<T1, T2, T3, T4, T5>::size_type>(
          a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_local_iterator
f65(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.cbegin(
      std::forward<typename std::unordered_map<T1, T2, T3, T4, T5>::size_type>(
          a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::local_iterator
f66(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.end(
      std::forward<typename std::unordered_map<T1, T2, T3, T4, T5>::size_type>(
          a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_local_iterator
f67(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.end(
      std::forward<typename std::unordered_map<T1, T2, T3, T4, T5>::size_type>(
          a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_local_iterator
f68(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.cend(
      std::forward<typename std::unordered_map<T1, T2, T3, T4, T5>::size_type>(
          a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
float f69(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.load_factor();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
float f70(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.max_load_factor();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f71(std::unordered_map<T1, T2, T3, T4, T5> &a0, float a1) {
  return a0.max_load_factor(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f72(std::unordered_map<T1, T2, T3, T4, T5> &a0,
         typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.rehash(
      std::forward<typename std::unordered_map<T1, T2, T3, T4, T5>::size_type>(
          a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f73(std::unordered_map<T1, T2, T3, T4, T5> &a0,
         typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.reserve(
      std::forward<typename std::unordered_map<T1, T2, T3, T4, T5>::size_type>(
          a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5>
f74(T6 a0, T6 a1, typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a2,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::hasher &a3,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_equal &a4,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a5) {
  return std::unordered_map<T1, T2, T3, T4, T5>(
      std::move(a0), std::move(a1),
      std::forward<typename std::unordered_map<T1, T2, T3, T4, T5>::size_type>(
          a2),
      a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5>
f75(T6 a0, T6 a1, typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a2,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a3) {
  return std::unordered_map<T1, T2, T3, T4, T5>(
      std::move(a0), std::move(a1),
      std::forward<typename std::unordered_map<T1, T2, T3, T4, T5>::size_type>(
          a2),
      a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5>
f76(T6 a0, T6 a1, typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a2,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::hasher &a3,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a4) {
  return std::unordered_map<T1, T2, T3, T4, T5>(
      std::move(a0), std::move(a1),
      std::forward<typename std::unordered_map<T1, T2, T3, T4, T5>::size_type>(
          a2),
      a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5>
f77(std::from_range_t a0, T6 &&a1,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a2,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::hasher &a3,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_equal &a4,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a5) {
  return std::unordered_map<T1, T2, T3, T4, T5>(
      std::move(a0), std::forward<T6>(a1),
      std::forward<typename std::unordered_map<T1, T2, T3, T4, T5>::size_type>(
          a2),
      a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5>
f78(std::from_range_t a0, T6 &&a1,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a2) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0),
                                                std::forward<T6>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5>
f79(std::from_range_t a0, T6 &&a1,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a2,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a3) {
  return std::unordered_map<T1, T2, T3, T4, T5>(
      std::move(a0), std::forward<T6>(a1),
      std::forward<typename std::unordered_map<T1, T2, T3, T4, T5>::size_type>(
          a2),
      a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5>
f80(std::from_range_t a0, T6 &&a1,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a2,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::hasher &a3,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a4) {
  return std::unordered_map<T1, T2, T3, T4, T5>(
      std::move(a0), std::forward<T6>(a1),
      std::forward<typename std::unordered_map<T1, T2, T3, T4, T5>::size_type>(
          a2),
      a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f81(std::unordered_map<T1, T2, T3, T4, T5> &a0, T6 &&...a1) {
  return a0.template emplace<T6...>(std::forward<T6>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f82(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
    T6 &&...a2) {
  return a0.template emplace_hint<T6...>(
      std::forward<
          typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator>(a1),
      std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f83(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1,
    T6 &&...a2) {
  return a0.template try_emplace<T6...>(a1, std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f84(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &&a1,
    T6 &&...a2) {
  return a0.template try_emplace<T6...>(std::move(a1), std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f85(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a2,
    T6 &&...a3) {
  return a0.template try_emplace<T6...>(
      std::forward<
          typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator>(a1),
      a2, std::forward<T6>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f86(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &&a2,
    T6 &&...a3) {
  return a0.template try_emplace<T6...>(
      std::forward<
          typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator>(a1),
      std::move(a2), std::forward<T6>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::__umap_hashtable<T1, T2, T3, T4, T5>::iterator, bool>
f87(std::unordered_map<T1, T2, T3, T4, T5> &a0, T6 &&a1)
  requires(
      !(std::is_same<T6, typename std::unordered_map<T1, T2, T3, T4,
                                                     T5>::node_type>::value) &&
      !(std::is_same<T6, const typename std::unordered_map<
                             T1, T2, T3, T4, T5>::value_type &>::value) &&
      !(std::is_same<T6, typename std::unordered_map<T1, T2, T3, T4,
                                                     T5>::value_type>::value) &&
      !(std::is_same<T6, std::initializer_list<typename std::unordered_map<
                             T1, T2, T3, T4, T5>::value_type>>::value))
{
  return a0.template insert<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::__umap_hashtable<T1, T2, T3, T4, T5>::iterator
f88(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1, T6 &&a2)
  requires(
      !(std::is_same<T6, typename std::unordered_map<T1, T2, T3, T4,
                                                     T5>::node_type>::value) &&
      !(std::is_same<T6, const typename std::unordered_map<
                             T1, T2, T3, T4, T5>::value_type &>::value) &&
      !(std::is_same<T6, typename std::unordered_map<T1, T2, T3, T4,
                                                     T5>::value_type>::value))
{
  return a0.template insert<T6>(
      std::forward<
          typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator>(a1),
      std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f89(std::unordered_map<T1, T2, T3, T4, T5> &a0, T6 a1, T6 a2) {
  return a0.template insert<T6>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f90(std::unordered_map<T1, T2, T3, T4, T5> &a0, T6 &&a1) {
  return a0.template insert_range<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f91(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1,
    T6 &&a2) {
  return a0.template insert_or_assign<T6>(a1, std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f92(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &&a1, T6 &&a2) {
  return a0.template insert_or_assign<T6>(std::move(a1), std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f93(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a2,
    T6 &&a3) {
  return a0.template insert_or_assign<T6>(
      std::forward<
          typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator>(a1),
      a2, std::forward<T6>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f94(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &&a2, T6 &&a3) {
  return a0.template insert_or_assign<T6>(
      std::forward<
          typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator>(a1),
      std::move(a2), std::forward<T6>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
void f95(std::unordered_map<T1, T2, T3, T4, T5> &a0,
         std::unordered_map<T1, T2, T6, T7, T5> &a1) {
  return a0.template merge<T6, T7>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
void f96(std::unordered_map<T1, T2, T3, T4, T5> &a0,
         std::unordered_map<T1, T2, T6, T7, T5> &&a1) {
  return a0.template merge<T6, T7>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
void f97(std::unordered_map<T1, T2, T3, T4, T5> &a0,
         std::unordered_multimap<T1, T2, T6, T7, T5> &a1) {
  return a0.template merge<T6, T7>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
void f98(std::unordered_map<T1, T2, T3, T4, T5> &a0,
         std::unordered_multimap<T1, T2, T6, T7, T5> &&a1) {
  return a0.template merge<T6, T7>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f99(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<T6, typename std::unordered_map<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f100(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<T6, typename std::unordered_map<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f101(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<T6, typename std::unordered_map<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template count<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f102(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<T6, typename std::unordered_map<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template contains<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f103(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<T6, typename std::unordered_map<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template equal_range<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f104(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<T6, typename std::unordered_map<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template equal_range<T6>(a1);
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
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator f105() {
  return typename std::unordered_map<T1, T2, T3, T4, T5>::iterator();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator::reference
f106(const typename std::unordered_map<T1, T2, T3, T4, T5>::iterator &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator::pointer
f107(const typename std::unordered_map<T1, T2, T3, T4, T5>::iterator &a0) {
  return a0.operator->();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator &
f108(typename std::unordered_map<T1, T2, T3, T4, T5>::iterator &a0) {
  return a0.operator++();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f109(typename std::unordered_map<T1, T2, T3, T4, T5>::iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f110(const typename std::unordered_map<T1, T2, T3, T4, T5>::iterator &a0,
          const typename std::unordered_map<T1, T2, T3, T4, T5>::iterator &a1) {
  return operator==(a0, a1);
}

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
typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator f111() {
  return typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
const std::pair<const T1, T2> &f112(
    const typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
const std::pair<const T1, T2> *f113(
    const typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator &a0) {
  return a0.operator->();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator &
f114(typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator &a0) {
  return a0.operator++();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator
f115(typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator &a0,
     int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f116(
    const typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator &a1) {
  return operator==(a0, a1);
}

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
typename std::unordered_map<T1, T2, T3, T4, T5>::local_iterator f117() {
  return typename std::unordered_map<T1, T2, T3, T4, T5>::local_iterator();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::local_iterator::reference f118(
    const typename std::unordered_map<T1, T2, T3, T4, T5>::local_iterator &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::local_iterator::pointer f119(
    const typename std::unordered_map<T1, T2, T3, T4, T5>::local_iterator &a0) {
  return a0.operator->();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::local_iterator &
f120(typename std::unordered_map<T1, T2, T3, T4, T5>::local_iterator &a0) {
  return a0.operator++();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::local_iterator
f121(typename std::unordered_map<T1, T2, T3, T4, T5>::local_iterator &a0,
     int a1) {
  return a0.operator++(a1);
}

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

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_local_iterator f122() {
  return
      typename std::unordered_map<T1, T2, T3, T4, T5>::const_local_iterator();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
const std::pair<const T1, T2> &
f123(const typename std::unordered_map<T1, T2, T3, T4, T5>::const_local_iterator
         &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
const std::pair<const T1, T2> *
f124(const typename std::unordered_map<T1, T2, T3, T4, T5>::const_local_iterator
         &a0) {
  return a0.operator->();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_local_iterator &f125(
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_local_iterator &a0) {
  return a0.operator++();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_local_iterator
f126(typename std::unordered_map<T1, T2, T3, T4, T5>::const_local_iterator &a0,
     int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t42 =
    typename std::unordered_map<T1, T2, T3, T4, T5>::node_type::key_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t43 =
    typename std::unordered_map<T1, T2, T3, T4, T5>::node_type::mapped_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::node_type f127() {
  return typename std::unordered_map<T1, T2, T3, T4, T5>::node_type();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::node_type
f128(typename std::unordered_map<T1, T2, T3, T4, T5>::node_type &&a0) {
  return
      typename std::unordered_map<T1, T2, T3, T4, T5>::node_type(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::node_type &
f129(typename std::unordered_map<T1, T2, T3, T4, T5>::node_type &a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::node_type &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T1 &f130(const typename std::unordered_map<T1, T2, T3, T4, T5>::node_type &a0) {
  return a0.key();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::node_type::mapped_type &
f131(const typename std::unordered_map<T1, T2, T3, T4, T5>::node_type &a0) {
  return a0.mapped();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f132(typename std::unordered_map<T1, T2, T3, T4, T5>::node_type &a0,
          typename std::unordered_map<T1, T2, T3, T4, T5>::node_type &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f133(typename std::unordered_map<T1, T2, T3, T4, T5>::node_type &a0,
          typename std::unordered_map<T1, T2, T3, T4, T5>::node_type &a1) {
  return swap(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::node_type::allocator_type
f134(const typename std::unordered_map<T1, T2, T3, T4, T5>::node_type &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f135(
    const typename std::unordered_map<T1, T2, T3, T4, T5>::node_type &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f136(
    const typename std::unordered_map<T1, T2, T3, T4, T5>::node_type &a0) {
  return a0.empty();
}
