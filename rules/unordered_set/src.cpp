#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <unordered_set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
bool f3(const std::unordered_multiset<T1, T2, T3, T4> &a0,
        const std::unordered_multiset<T1, T2, T3, T4> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f4(const std::unordered_set<T1, T2, T3, T4> &a0,
        const std::unordered_set<T1, T2, T3, T4> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3, typename T4>
void f5(std::unordered_multiset<T1, T2, T3, T4> &a0,
        std::unordered_multiset<T1, T2, T3, T4> &a1) {
  return std::swap<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f6(std::unordered_set<T1, T2, T3, T4> &a0,
        std::unordered_set<T1, T2, T3, T4> &a1) {
  return std::swap<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
using t1 = std::unordered_set<T1, T2, T3, T4>;

template <typename T1, typename T2, typename T3, typename T4>
using t2 = typename std::unordered_set<T1, T2, T3, T4>::key_type;

template <typename T1, typename T2, typename T3, typename T4>
using t3 = typename std::unordered_set<T1, T2, T3, T4>::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t4 = typename std::unordered_set<T1, T2, T3, T4>::hasher;

template <typename T1, typename T2, typename T3, typename T4>
using t5 = typename std::unordered_set<T1, T2, T3, T4>::key_equal;

template <typename T1, typename T2, typename T3, typename T4>
using t6 = typename std::unordered_set<T1, T2, T3, T4>::allocator_type;

template <typename T1, typename T2, typename T3, typename T4>
using t7 = typename std::unordered_set<T1, T2, T3, T4>::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t8 = typename std::unordered_set<T1, T2, T3, T4>::const_pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t9 = typename std::unordered_set<T1, T2, T3, T4>::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t10 = typename std::unordered_set<T1, T2, T3, T4>::const_reference;

template <typename T1, typename T2, typename T3, typename T4>
using t11 = typename std::unordered_set<T1, T2, T3, T4>::size_type;

template <typename T1, typename T2, typename T3, typename T4>
using t12 = typename std::unordered_set<T1, T2, T3, T4>::difference_type;

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
  return std::unordered_set<T1, T2, T3, T4>(a0, a1, a2, a3);
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
  return std::unordered_set<T1, T2, T3, T4>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f15(typename std::unordered_set<T1, T2, T3, T4>::size_type a0,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a1) {
  return std::unordered_set<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f16(typename std::unordered_set<T1, T2, T3, T4>::size_type a0,
    const typename std::unordered_set<T1, T2, T3, T4>::hasher &a1,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a2) {
  return std::unordered_set<T1, T2, T3, T4>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f17(std::initializer_list<
        typename std::unordered_set<T1, T2, T3, T4>::value_type>
        a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a2) {
  return std::unordered_set<T1, T2, T3, T4>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f18(std::initializer_list<
        typename std::unordered_set<T1, T2, T3, T4>::value_type>
        a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1,
    const typename std::unordered_set<T1, T2, T3, T4>::hasher &a2,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a3) {
  return std::unordered_set<T1, T2, T3, T4>(a0, a1, a2, a3);
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
  return a0.operator=(a1);
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
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f35(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator a1,
    typename std::unordered_set<T1, T2, T3, T4>::value_type &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
void f36(std::unordered_set<T1, T2, T3, T4> &a0,
         std::initializer_list<
             typename std::unordered_set<T1, T2, T3, T4>::value_type>
             a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f37(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::size_type
f38(std::unordered_set<T1, T2, T3, T4> &a0,
    const typename std::unordered_set<T1, T2, T3, T4>::key_type &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f39(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator a1,
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
void f40(std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3, typename T4>
void f41(std::unordered_set<T1, T2, T3, T4> &a0,
         std::unordered_set<T1, T2, T3, T4> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::hasher
f42(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.hash_function();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::key_equal
f43(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.key_eq();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f44(std::unordered_set<T1, T2, T3, T4> &a0,
    const typename std::unordered_set<T1, T2, T3, T4>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::const_iterator
f45(const std::unordered_set<T1, T2, T3, T4> &a0,
    const typename std::unordered_set<T1, T2, T3, T4>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::size_type
f46(const std::unordered_set<T1, T2, T3, T4> &a0,
    const typename std::unordered_set<T1, T2, T3, T4>::key_type &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::unordered_set<T1, T2, T3, T4>::iterator,
          typename std::unordered_set<T1, T2, T3, T4>::iterator>
f47(std::unordered_set<T1, T2, T3, T4> &a0,
    const typename std::unordered_set<T1, T2, T3, T4>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::unordered_set<T1, T2, T3, T4>::const_iterator,
          typename std::unordered_set<T1, T2, T3, T4>::const_iterator>
f48(const std::unordered_set<T1, T2, T3, T4> &a0,
    const typename std::unordered_set<T1, T2, T3, T4>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::size_type
f49(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.bucket_count();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::size_type
f50(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.max_bucket_count();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::size_type
f51(const std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.bucket_size(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::size_type
f52(const std::unordered_set<T1, T2, T3, T4> &a0,
    const typename std::unordered_set<T1, T2, T3, T4>::key_type &a1) {
  return a0.bucket(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::local_iterator
f53(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.begin(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::const_local_iterator
f54(const std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.begin(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::const_local_iterator
f55(const std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.cbegin(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::local_iterator
f56(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.end(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::const_local_iterator
f57(const std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.end(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::const_local_iterator
f58(const std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.cend(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
float f59(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.load_factor();
}

template <typename T1, typename T2, typename T3, typename T4>
float f60(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.max_load_factor();
}

template <typename T1, typename T2, typename T3, typename T4>
void f61(std::unordered_set<T1, T2, T3, T4> &a0, float a1) {
  return a0.max_load_factor(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f62(std::unordered_set<T1, T2, T3, T4> &a0,
         typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.rehash(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f63(std::unordered_set<T1, T2, T3, T4> &a0,
         typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.reserve(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_set<T1, T2, T3, T4>
f64(T5 a0, T5 a1, typename std::unordered_set<T1, T2, T3, T4>::size_type a2,
    const typename std::unordered_set<T1, T2, T3, T4>::hasher &a3,
    const typename std::unordered_set<T1, T2, T3, T4>::key_equal &a4,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a5) {
  return std::unordered_set<T1, T2, T3, T4>(a0, a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_set<T1, T2, T3, T4>
f65(T5 a0, T5 a1, typename std::unordered_set<T1, T2, T3, T4>::size_type a2,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a3) {
  return std::unordered_set<T1, T2, T3, T4>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_set<T1, T2, T3, T4>
f66(T5 a0, T5 a1, typename std::unordered_set<T1, T2, T3, T4>::size_type a2,
    const typename std::unordered_set<T1, T2, T3, T4>::hasher &a3,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a4) {
  return std::unordered_set<T1, T2, T3, T4>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
std::pair<typename std::unordered_set<T1, T2, T3, T4>::iterator, bool>
f67(std::unordered_set<T1, T2, T3, T4> &a0, T5 &&...a1) {
  return a0.template emplace<T5...>(std::forward<T5>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f68(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator a1,
    T5 &&...a2) {
  return a0.template emplace_hint<T5...>(a1, std::forward<T5>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f69(std::unordered_set<T1, T2, T3, T4> &a0, T5 a1, T5 a2) {
  return a0.template insert<T5>(a1, a2);
}
