#include <algorithm>
#include <initializer_list>
#include <unordered_set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multiset<T1, T2, T3, T4>::size_type
f1(std::unordered_multiset<T1, T2, T3, T4> &a0, T5 a1) {
  return std::erase_if(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_set<T1, T2, T3, T4>::size_type
f2(std::unordered_set<T1, T2, T3, T4> &a0, T5 a1) {
  return std::erase_if(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
bool f3(const std::unordered_multiset<T1, T2, T3, T4> &a0,
        const std::unordered_multiset<T1, T2, T3, T4> &a1) {
  return std::operator==(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f4(const std::unordered_set<T1, T2, T3, T4> &a0,
        const std::unordered_set<T1, T2, T3, T4> &a1) {
  return std::operator==(a0, a1);
}

template <typename T1, typename T2, typename T3>
using t1 = std::pmr::unordered_multiset<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = std::pmr::unordered_set<T1, T2, T3>;

template <typename T1, typename T2, typename T3, typename T4>
void f5(std::unordered_multiset<T1, T2, T3, T4> &a0,
        std::unordered_multiset<T1, T2, T3, T4> &a1) {
  return std::swap(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f6(std::unordered_set<T1, T2, T3, T4> &a0,
        std::unordered_set<T1, T2, T3, T4> &a1) {
  return std::swap(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
using t3 = std::unordered_set<T1, T2, T3, T4>;

template <typename T1, typename T2, typename T3, typename T4>
using t4 = typename std::unordered_set<T1, T2, T3, T4>::key_type;

template <typename T1, typename T2, typename T3, typename T4>
using t5 = typename std::unordered_set<T1, T2, T3, T4>::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t6 = typename std::unordered_set<T1, T2, T3, T4>::hasher;

template <typename T1, typename T2, typename T3, typename T4>
using t7 = typename std::unordered_set<T1, T2, T3, T4>::key_equal;

template <typename T1, typename T2, typename T3, typename T4>
using t8 = typename std::unordered_set<T1, T2, T3, T4>::allocator_type;

template <typename T1, typename T2, typename T3, typename T4>
using t9 = typename std::unordered_set<T1, T2, T3, T4>::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t10 = typename std::unordered_set<T1, T2, T3, T4>::const_pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t11 = typename std::unordered_set<T1, T2, T3, T4>::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t12 = typename std::unordered_set<T1, T2, T3, T4>::const_reference;

template <typename T1, typename T2, typename T3, typename T4>
using t13 = typename std::unordered_set<T1, T2, T3, T4>::iterator;

template <typename T1, typename T2, typename T3, typename T4>
using t14 = typename std::unordered_set<T1, T2, T3, T4>::const_iterator;

template <typename T1, typename T2, typename T3, typename T4>
using t15 = typename std::unordered_set<T1, T2, T3, T4>::local_iterator;

template <typename T1, typename T2, typename T3, typename T4>
using t16 = typename std::unordered_set<T1, T2, T3, T4>::const_local_iterator;

template <typename T1, typename T2, typename T3, typename T4>
using t17 = typename std::unordered_set<T1, T2, T3, T4>::size_type;

template <typename T1, typename T2, typename T3, typename T4>
using t18 = typename std::unordered_set<T1, T2, T3, T4>::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t19 = typename std::unordered_set<T1, T2, T3, T4>::node_type;

template <typename T1, typename T2, typename T3, typename T4>
using t20 = typename std::unordered_set<T1, T2, T3, T4>::insert_return_type;

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4> f7() {
  return std::unordered_set<T1, T2, T3, T4>();
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f8(typename std::unordered_set<T1, T2, T3, T4>::size_type a0,
   const typename std::unordered_set<T1, T2, T3, T4>::hasher &a1,
   const typename std::unordered_set<T1, T2, T3, T4>::key_equal &a2,
   const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a3) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a0),
      a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f9(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return std::unordered_set<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f10(std::unordered_set<T1, T2, T3, T4> &&a0) {
  return std::unordered_set<T1, T2, T3, T4>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f11(const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a0) {
  return std::unordered_set<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f12(const std::unordered_set<T1, T2, T3, T4> &a0,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a1) {
  return std::unordered_set<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f13(std::unordered_set<T1, T2, T3, T4> &&a0,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a1) {
  return std::unordered_set<T1, T2, T3, T4>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f14(std::initializer_list<
        typename std::unordered_set<T1, T2, T3, T4>::value_type>
        a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1,
    const typename std::unordered_set<T1, T2, T3, T4>::hasher &a2,
    const typename std::unordered_set<T1, T2, T3, T4>::key_equal &a3,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a4) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::forward<std::initializer_list<
          typename std::unordered_set<T1, T2, T3, T4>::value_type>>(a0),
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a1),
      a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f15(typename std::unordered_set<T1, T2, T3, T4>::size_type a0,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a1) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a0),
      a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f16(typename std::unordered_set<T1, T2, T3, T4>::size_type a0,
    const typename std::unordered_set<T1, T2, T3, T4>::hasher &a1,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a2) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a0),
      a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f17(std::initializer_list<
        typename std::unordered_set<T1, T2, T3, T4>::value_type>
        a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a2) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::forward<std::initializer_list<
          typename std::unordered_set<T1, T2, T3, T4>::value_type>>(a0),
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a1),
      a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f18(std::initializer_list<
        typename std::unordered_set<T1, T2, T3, T4>::value_type>
        a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1,
    const typename std::unordered_set<T1, T2, T3, T4>::hasher &a2,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a3) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::forward<std::initializer_list<
          typename std::unordered_set<T1, T2, T3, T4>::value_type>>(a0),
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a1),
      a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4> &
f19(std::unordered_set<T1, T2, T3, T4> &a0,
    const std::unordered_set<T1, T2, T3, T4> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4> &
f20(std::unordered_set<T1, T2, T3, T4> &a0,
    std::unordered_set<T1, T2, T3, T4> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4> &
f21(std::unordered_set<T1, T2, T3, T4> &a0,
    std::initializer_list<
        typename std::unordered_set<T1, T2, T3, T4>::value_type>
        a1) {
  return a0.operator=(
      std::forward<std::initializer_list<
          typename std::unordered_set<T1, T2, T3, T4>::value_type>>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::allocator_type
f22(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3, typename T4>
bool f23(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::size_type
f24(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::size_type
f25(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f26(std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::const_iterator
f27(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f28(std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::const_iterator
f29(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::const_iterator
f30(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::const_iterator
f31(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::unordered_set<T1, T2, T3, T4>::iterator, bool>
f32(std::unordered_set<T1, T2, T3, T4> &a0,
    const typename std::unordered_set<T1, T2, T3, T4>::value_type &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::unordered_set<T1, T2, T3, T4>::iterator, bool>
f33(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::value_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f34(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator a1,
    const typename std::unordered_set<T1, T2, T3, T4>::value_type &a2) {
  return a0.insert(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::const_iterator>(
          a1),
      a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f35(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator a1,
    typename std::unordered_set<T1, T2, T3, T4>::value_type &&a2) {
  return a0.insert(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::const_iterator>(
          a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
void f36(std::unordered_set<T1, T2, T3, T4> &a0,
         std::initializer_list<
             typename std::unordered_set<T1, T2, T3, T4>::value_type>
             a1) {
  return a0.insert(
      std::forward<std::initializer_list<
          typename std::unordered_set<T1, T2, T3, T4>::value_type>>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::node_type
f37(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator a1) {
  return a0.extract(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::const_iterator>(
          a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::node_type
f38(std::unordered_set<T1, T2, T3, T4> &a0,
    const typename std::unordered_set<T1, T2, T3, T4>::key_type &a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::insert_return_type
f39(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::node_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f40(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator a1,
    typename std::unordered_set<T1, T2, T3, T4>::node_type &&a2) {
  return a0.insert(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::const_iterator>(
          a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f41(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator a1) {
  return a0.erase(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::const_iterator>(
          a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f42(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::iterator a1) {
  return a0.erase(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::iterator>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::size_type
f43(std::unordered_set<T1, T2, T3, T4> &a0,
    const typename std::unordered_set<T1, T2, T3, T4>::key_type &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f44(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator a1,
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator a2) {
  return a0.erase(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::const_iterator>(
          a1),
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::const_iterator>(
          a2));
}

template <typename T1, typename T2, typename T3, typename T4>
void f45(std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3, typename T4>
void f46(std::unordered_set<T1, T2, T3, T4> &a0,
         std::unordered_set<T1, T2, T3, T4> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::hasher
f47(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.hash_function();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::key_equal
f48(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.key_eq();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f49(std::unordered_set<T1, T2, T3, T4> &a0,
    const typename std::unordered_set<T1, T2, T3, T4>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::const_iterator
f50(const std::unordered_set<T1, T2, T3, T4> &a0,
    const typename std::unordered_set<T1, T2, T3, T4>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::size_type
f51(const std::unordered_set<T1, T2, T3, T4> &a0,
    const typename std::unordered_set<T1, T2, T3, T4>::key_type &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f52(const std::unordered_set<T1, T2, T3, T4> &a0,
         const typename std::unordered_set<T1, T2, T3, T4>::key_type &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::unordered_set<T1, T2, T3, T4>::iterator,
          typename std::unordered_set<T1, T2, T3, T4>::iterator>
f53(std::unordered_set<T1, T2, T3, T4> &a0,
    const typename std::unordered_set<T1, T2, T3, T4>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::unordered_set<T1, T2, T3, T4>::const_iterator,
          typename std::unordered_set<T1, T2, T3, T4>::const_iterator>
f54(const std::unordered_set<T1, T2, T3, T4> &a0,
    const typename std::unordered_set<T1, T2, T3, T4>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::size_type
f55(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.bucket_count();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::size_type
f56(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.max_bucket_count();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::size_type
f57(const std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.bucket_size(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::size_type
f58(const std::unordered_set<T1, T2, T3, T4> &a0,
    const typename std::unordered_set<T1, T2, T3, T4>::key_type &a1) {
  return a0.bucket(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::local_iterator
f59(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.begin(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::const_local_iterator
f60(const std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.begin(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::const_local_iterator
f61(const std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.cbegin(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::local_iterator
f62(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.end(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::const_local_iterator
f63(const std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.end(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::const_local_iterator
f64(const std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.cend(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
float f65(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.load_factor();
}

template <typename T1, typename T2, typename T3, typename T4>
float f66(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.max_load_factor();
}

template <typename T1, typename T2, typename T3, typename T4>
void f67(std::unordered_set<T1, T2, T3, T4> &a0, float a1) {
  return a0.max_load_factor(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f68(std::unordered_set<T1, T2, T3, T4> &a0,
         typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.rehash(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
void f69(std::unordered_set<T1, T2, T3, T4> &a0,
         typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.reserve(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_set<T1, T2, T3, T4>
f70(T5 a0, T5 a1, typename std::unordered_set<T1, T2, T3, T4>::size_type a2,
    const typename std::unordered_set<T1, T2, T3, T4>::hasher &a3,
    const typename std::unordered_set<T1, T2, T3, T4>::key_equal &a4,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a5) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::move(a0), std::move(a1),
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a2),
      a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_set<T1, T2, T3, T4>
f71(T5 a0, T5 a1, typename std::unordered_set<T1, T2, T3, T4>::size_type a2,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a3) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::move(a0), std::move(a1),
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a2),
      a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_set<T1, T2, T3, T4>
f72(T5 a0, T5 a1, typename std::unordered_set<T1, T2, T3, T4>::size_type a2,
    const typename std::unordered_set<T1, T2, T3, T4>::hasher &a3,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a4) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::move(a0), std::move(a1),
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a2),
      a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_set<T1, T2, T3, T4>
f73(std::from_range_t a0, T5 &&a1,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a2,
    const typename std::unordered_set<T1, T2, T3, T4>::hasher &a3,
    const typename std::unordered_set<T1, T2, T3, T4>::key_equal &a4,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a5) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::move(a0), std::forward<T5>(a1),
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a2),
      a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_set<T1, T2, T3, T4>
f74(std::from_range_t a0, T5 &&a1,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a2) {
  return std::unordered_set<T1, T2, T3, T4>(std::move(a0), std::forward<T5>(a1),
                                            a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_set<T1, T2, T3, T4>
f75(std::from_range_t a0, T5 &&a1,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a2,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a3) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::move(a0), std::forward<T5>(a1),
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a2),
      a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_set<T1, T2, T3, T4>
f76(std::from_range_t a0, T5 &&a1,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a2,
    const typename std::unordered_set<T1, T2, T3, T4>::hasher &a3,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a4) {
  return std::unordered_set<T1, T2, T3, T4>(
      std::move(a0), std::forward<T5>(a1),
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::size_type>(a2),
      a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
std::pair<typename std::unordered_set<T1, T2, T3, T4>::iterator, bool>
f77(std::unordered_set<T1, T2, T3, T4> &a0, T5 &&...a1) {
  return a0.template emplace<T5...>(std::forward<T5>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f78(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator a1,
    T5 &&...a2) {
  return a0.template emplace_hint<T5...>(
      std::forward<typename std::unordered_set<T1, T2, T3, T4>::const_iterator>(
          a1),
      std::forward<T5>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f79(std::unordered_set<T1, T2, T3, T4> &a0, T5 a1, T5 a2) {
  return a0.template insert<T5>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f80(std::unordered_set<T1, T2, T3, T4> &a0, T5 &&a1) {
  return a0.template insert_range<T5>(std::forward<T5>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f81(std::unordered_set<T1, T2, T3, T4> &a0,
         std::unordered_set<T1, T5, T6, T4> &a1) {
  return a0.template merge<T5, T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f82(std::unordered_set<T1, T2, T3, T4> &a0,
         std::unordered_set<T1, T5, T6, T4> &&a1) {
  return a0.template merge<T5, T6>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f83(std::unordered_set<T1, T2, T3, T4> &a0,
         std::unordered_multiset<T1, T5, T6, T4> &a1) {
  return a0.template merge<T5, T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f84(std::unordered_set<T1, T2, T3, T4> &a0,
         std::unordered_multiset<T1, T5, T6, T4> &&a1) {
  return a0.template merge<T5, T6>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f85(std::unordered_set<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::unordered_set<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template find<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f86(const std::unordered_set<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::unordered_set<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template find<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f87(const std::unordered_set<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::unordered_set<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template count<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f88(const std::unordered_set<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::unordered_set<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template contains<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f89(std::unordered_set<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::unordered_set<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template equal_range<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f90(const std::unordered_set<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::unordered_set<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template equal_range<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
using t21 = typename std::unordered_set<T1, T2, T3, T4>::iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t22 =
    typename std::unordered_set<T1, T2, T3, T4>::iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t23 =
    typename std::unordered_set<T1, T2, T3, T4>::iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t24 = typename std::unordered_set<T1, T2, T3, T4>::iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t25 = typename std::unordered_set<T1, T2, T3, T4>::iterator::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t26 =
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t27 = typename std::unordered_set<T1, T2, T3,
                                        T4>::const_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t28 = typename std::unordered_set<T1, T2, T3,
                                        T4>::const_iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t29 =
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t30 =
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t31 =
    typename std::unordered_set<T1, T2, T3, T4>::local_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t32 =
    typename std::unordered_set<T1, T2, T3, T4>::local_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t33 =
    typename std::unordered_set<T1, T2, T3, T4>::local_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t34 = typename std::unordered_set<T1, T2, T3,
                                        T4>::local_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t35 = typename std::unordered_set<T1, T2, T3,
                                        T4>::local_iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t36 = typename std::unordered_set<T1, T2, T3,
                                        T4>::const_local_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t37 =
    typename std::unordered_set<T1, T2, T3, T4>::const_local_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t38 = typename std::unordered_set<T1, T2, T3,
                                        T4>::const_local_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t39 =
    typename std::unordered_set<T1, T2, T3,
                                T4>::const_local_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t40 =
    typename std::unordered_set<T1, T2, T3,
                                T4>::const_local_iterator::iterator_category;
