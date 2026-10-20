#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <unordered_set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
using t1 = std::unordered_multiset<T1, T2, T3, T4>;

template <typename T1, typename T2, typename T3, typename T4>
using t2 = typename std::unordered_multiset<T1, T2, T3, T4>::key_type;

template <typename T1, typename T2, typename T3, typename T4>
using t3 = typename std::unordered_multiset<T1, T2, T3, T4>::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t4 = typename std::unordered_multiset<T1, T2, T3, T4>::hasher;

template <typename T1, typename T2, typename T3, typename T4>
using t5 = typename std::unordered_multiset<T1, T2, T3, T4>::key_equal;

template <typename T1, typename T2, typename T3, typename T4>
using t6 = typename std::unordered_multiset<T1, T2, T3, T4>::allocator_type;

template <typename T1, typename T2, typename T3, typename T4>
using t7 = typename std::unordered_multiset<T1, T2, T3, T4>::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t8 = typename std::unordered_multiset<T1, T2, T3, T4>::const_pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t9 = typename std::unordered_multiset<T1, T2, T3, T4>::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t10 = typename std::unordered_multiset<T1, T2, T3, T4>::const_reference;

template <typename T1, typename T2, typename T3, typename T4>
using t11 = typename std::unordered_multiset<T1, T2, T3, T4>::size_type;

template <typename T1, typename T2, typename T3, typename T4>
using t12 = typename std::unordered_multiset<T1, T2, T3, T4>::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_multiset<T1, T2, T3, T4> f1() {
  return std::unordered_multiset<T1, T2, T3, T4>();
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_multiset<T1, T2, T3, T4>
f2(typename std::unordered_multiset<T1, T2, T3, T4>::size_type a0,
   const typename std::unordered_multiset<T1, T2, T3, T4>::hasher &a1,
   const typename std::unordered_multiset<T1, T2, T3, T4>::key_equal &a2,
   const typename std::unordered_multiset<T1, T2, T3, T4>::allocator_type &a3) {
  return std::unordered_multiset<T1, T2, T3, T4>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_multiset<T1, T2, T3, T4>
f3(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return std::unordered_multiset<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_multiset<T1, T2, T3, T4>
f4(std::unordered_multiset<T1, T2, T3, T4> &&a0) {
  return std::unordered_multiset<T1, T2, T3, T4>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_multiset<T1, T2, T3, T4>
f5(std::initializer_list<
       typename std::unordered_multiset<T1, T2, T3, T4>::value_type>
       a0,
   typename std::unordered_multiset<T1, T2, T3, T4>::size_type a1,
   const typename std::unordered_multiset<T1, T2, T3, T4>::hasher &a2,
   const typename std::unordered_multiset<T1, T2, T3, T4>::key_equal &a3,
   const typename std::unordered_multiset<T1, T2, T3, T4>::allocator_type &a4) {
  return std::unordered_multiset<T1, T2, T3, T4>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_multiset<T1, T2, T3, T4> &
f6(std::unordered_multiset<T1, T2, T3, T4> &a0,
   const std::unordered_multiset<T1, T2, T3, T4> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_multiset<T1, T2, T3, T4> &
f7(std::unordered_multiset<T1, T2, T3, T4> &a0,
   std::unordered_multiset<T1, T2, T3, T4> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_multiset<T1, T2, T3, T4>
f8(const typename std::unordered_multiset<T1, T2, T3, T4>::allocator_type &a0) {
  return std::unordered_multiset<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_multiset<T1, T2, T3, T4>
f9(const std::unordered_multiset<T1, T2, T3, T4> &a0,
   const typename std::unordered_multiset<T1, T2, T3, T4>::allocator_type &a1) {
  return std::unordered_multiset<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_multiset<T1, T2, T3, T4>
f10(std::unordered_multiset<T1, T2, T3, T4> &&a0,
    const typename std::unordered_multiset<T1, T2, T3, T4>::allocator_type
        &a1) {
  return std::unordered_multiset<T1, T2, T3, T4>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_multiset<T1, T2, T3, T4>
f11(typename std::unordered_multiset<T1, T2, T3, T4>::size_type a0,
    const typename std::unordered_multiset<T1, T2, T3, T4>::allocator_type
        &a1) {
  return std::unordered_multiset<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_multiset<T1, T2, T3, T4>
f12(typename std::unordered_multiset<T1, T2, T3, T4>::size_type a0,
    const typename std::unordered_multiset<T1, T2, T3, T4>::hasher &a1,
    const typename std::unordered_multiset<T1, T2, T3, T4>::allocator_type
        &a2) {
  return std::unordered_multiset<T1, T2, T3, T4>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_multiset<T1, T2, T3, T4>
f13(std::initializer_list<
        typename std::unordered_multiset<T1, T2, T3, T4>::value_type>
        a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::size_type a1,
    const typename std::unordered_multiset<T1, T2, T3, T4>::allocator_type
        &a2) {
  return std::unordered_multiset<T1, T2, T3, T4>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_multiset<T1, T2, T3, T4>
f14(std::initializer_list<
        typename std::unordered_multiset<T1, T2, T3, T4>::value_type>
        a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::size_type a1,
    const typename std::unordered_multiset<T1, T2, T3, T4>::hasher &a2,
    const typename std::unordered_multiset<T1, T2, T3, T4>::allocator_type
        &a3) {
  return std::unordered_multiset<T1, T2, T3, T4>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_multiset<T1, T2, T3, T4> &
f15(std::unordered_multiset<T1, T2, T3, T4> &a0,
    std::initializer_list<
        typename std::unordered_multiset<T1, T2, T3, T4>::value_type>
        a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::allocator_type
f16(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3, typename T4>
bool f17(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::size_type
f18(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::size_type
f19(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::iterator
f20(std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator
f21(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::iterator
f22(std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator
f23(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator
f24(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator
f25(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::iterator
f26(std::unordered_multiset<T1, T2, T3, T4> &a0,
    const typename std::unordered_multiset<T1, T2, T3, T4>::value_type &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::iterator
f27(std::unordered_multiset<T1, T2, T3, T4> &a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::value_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::iterator
f28(std::unordered_multiset<T1, T2, T3, T4> &a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator a1,
    const typename std::unordered_multiset<T1, T2, T3, T4>::value_type &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::iterator
f29(std::unordered_multiset<T1, T2, T3, T4> &a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator a1,
    typename std::unordered_multiset<T1, T2, T3, T4>::value_type &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
void f30(std::unordered_multiset<T1, T2, T3, T4> &a0,
         std::initializer_list<
             typename std::unordered_multiset<T1, T2, T3, T4>::value_type>
             a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::iterator
f31(std::unordered_multiset<T1, T2, T3, T4> &a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::iterator
f32(std::unordered_multiset<T1, T2, T3, T4> &a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::size_type
f33(std::unordered_multiset<T1, T2, T3, T4> &a0,
    const typename std::unordered_multiset<T1, T2, T3, T4>::key_type &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::iterator
f34(std::unordered_multiset<T1, T2, T3, T4> &a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator a1,
    typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
void f35(std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3, typename T4>
void f36(std::unordered_multiset<T1, T2, T3, T4> &a0,
         std::unordered_multiset<T1, T2, T3, T4> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::hasher
f37(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.hash_function();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::key_equal
f38(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.key_eq();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::iterator
f39(std::unordered_multiset<T1, T2, T3, T4> &a0,
    const typename std::unordered_multiset<T1, T2, T3, T4>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator
f40(const std::unordered_multiset<T1, T2, T3, T4> &a0,
    const typename std::unordered_multiset<T1, T2, T3, T4>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::size_type
f41(const std::unordered_multiset<T1, T2, T3, T4> &a0,
    const typename std::unordered_multiset<T1, T2, T3, T4>::key_type &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::unordered_multiset<T1, T2, T3, T4>::iterator,
          typename std::unordered_multiset<T1, T2, T3, T4>::iterator>
f42(std::unordered_multiset<T1, T2, T3, T4> &a0,
    const typename std::unordered_multiset<T1, T2, T3, T4>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator,
          typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator>
f43(const std::unordered_multiset<T1, T2, T3, T4> &a0,
    const typename std::unordered_multiset<T1, T2, T3, T4>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::size_type
f44(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.bucket_count();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::size_type
f45(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.max_bucket_count();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::size_type
f46(const std::unordered_multiset<T1, T2, T3, T4> &a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::size_type a1) {
  return a0.bucket_size(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::size_type
f47(const std::unordered_multiset<T1, T2, T3, T4> &a0,
    const typename std::unordered_multiset<T1, T2, T3, T4>::key_type &a1) {
  return a0.bucket(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::local_iterator
f48(std::unordered_multiset<T1, T2, T3, T4> &a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::size_type a1) {
  return a0.begin(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::const_local_iterator
f49(const std::unordered_multiset<T1, T2, T3, T4> &a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::size_type a1) {
  return a0.begin(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::const_local_iterator
f50(const std::unordered_multiset<T1, T2, T3, T4> &a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::size_type a1) {
  return a0.cbegin(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::local_iterator
f51(std::unordered_multiset<T1, T2, T3, T4> &a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::size_type a1) {
  return a0.end(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::const_local_iterator
f52(const std::unordered_multiset<T1, T2, T3, T4> &a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::size_type a1) {
  return a0.end(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::const_local_iterator
f53(const std::unordered_multiset<T1, T2, T3, T4> &a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::size_type a1) {
  return a0.cend(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
float f54(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.load_factor();
}

template <typename T1, typename T2, typename T3, typename T4>
float f55(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.max_load_factor();
}

template <typename T1, typename T2, typename T3, typename T4>
void f56(std::unordered_multiset<T1, T2, T3, T4> &a0, float a1) {
  return a0.max_load_factor(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f57(std::unordered_multiset<T1, T2, T3, T4> &a0,
         typename std::unordered_multiset<T1, T2, T3, T4>::size_type a1) {
  return a0.rehash(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f58(std::unordered_multiset<T1, T2, T3, T4> &a0,
         typename std::unordered_multiset<T1, T2, T3, T4>::size_type a1) {
  return a0.reserve(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multiset<T1, T2, T3, T4>
f59(T5 a0, T5 a1,
    typename std::unordered_multiset<T1, T2, T3, T4>::size_type a2,
    const typename std::unordered_multiset<T1, T2, T3, T4>::hasher &a3,
    const typename std::unordered_multiset<T1, T2, T3, T4>::key_equal &a4,
    const typename std::unordered_multiset<T1, T2, T3, T4>::allocator_type
        &a5) {
  return std::unordered_multiset<T1, T2, T3, T4>(a0, a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multiset<T1, T2, T3, T4>
f60(T5 a0, T5 a1,
    typename std::unordered_multiset<T1, T2, T3, T4>::size_type a2,
    const typename std::unordered_multiset<T1, T2, T3, T4>::allocator_type
        &a3) {
  return std::unordered_multiset<T1, T2, T3, T4>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multiset<T1, T2, T3, T4>
f61(T5 a0, T5 a1,
    typename std::unordered_multiset<T1, T2, T3, T4>::size_type a2,
    const typename std::unordered_multiset<T1, T2, T3, T4>::hasher &a3,
    const typename std::unordered_multiset<T1, T2, T3, T4>::allocator_type
        &a4) {
  return std::unordered_multiset<T1, T2, T3, T4>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
typename std::unordered_multiset<T1, T2, T3, T4>::iterator
f62(std::unordered_multiset<T1, T2, T3, T4> &a0, T5 &&...a1) {
  return a0.template emplace<T5...>(std::forward<T5>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
typename std::unordered_multiset<T1, T2, T3, T4>::iterator
f63(std::unordered_multiset<T1, T2, T3, T4> &a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator a1,
    T5 &&...a2) {
  return a0.template emplace_hint<T5...>(a1, std::forward<T5>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f64(std::unordered_multiset<T1, T2, T3, T4> &a0, T5 a1, T5 a2) {
  return a0.template insert<T5>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
using t13 =
    typename std::unordered_multiset<T1, T2, T3, T4>::iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t14 =
    typename std::unordered_multiset<T1, T2, T3, T4>::iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t15 = typename std::unordered_multiset<T1, T2, T3,
                                             T4>::iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t16 = typename std::unordered_multiset<T1, T2, T3, T4>::iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t17 =
    typename std::unordered_multiset<T1, T2, T3, T4>::iterator::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t18 = typename std::unordered_multiset<T1, T2, T3,
                                             T4>::const_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t19 =
    typename std::unordered_multiset<T1, T2, T3,
                                     T4>::const_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t20 =
    typename std::unordered_multiset<T1, T2, T3,
                                     T4>::const_iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t21 =
    typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t22 =
    typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t23 = typename std::unordered_multiset<T1, T2, T3,
                                             T4>::local_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t24 =
    typename std::unordered_multiset<T1, T2, T3, T4>::local_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t25 =
    typename std::unordered_multiset<T1, T2, T3, T4>::local_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t26 =
    typename std::unordered_multiset<T1, T2, T3,
                                     T4>::local_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t27 =
    typename std::unordered_multiset<T1, T2, T3,
                                     T4>::local_iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t28 =
    typename std::unordered_multiset<T1, T2, T3,
                                     T4>::const_local_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t29 = typename std::unordered_multiset<T1, T2, T3,
                                             T4>::const_local_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t30 =
    typename std::unordered_multiset<T1, T2, T3,
                                     T4>::const_local_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t31 =
    typename std::unordered_multiset<T1, T2, T3,
                                     T4>::const_local_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t32 = typename std::unordered_multiset<
    T1, T2, T3, T4>::const_local_iterator::iterator_category;
