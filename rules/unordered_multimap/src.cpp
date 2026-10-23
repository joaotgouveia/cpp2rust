#include <algorithm>
#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t1 = std::unordered_multimap<T1, T2, T3, T4, T5>;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t2 = typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t3 = typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t4 = typename std::unordered_multimap<T1, T2, T3, T4, T5>::mapped_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t5 = typename std::unordered_multimap<T1, T2, T3, T4, T5>::hasher;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t6 = typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_equal;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t7 = typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t8 = typename std::unordered_multimap<T1, T2, T3, T4, T5>::pointer;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t9 = typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_pointer;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t10 = typename std::unordered_multimap<T1, T2, T3, T4, T5>::reference;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t11 =
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_reference;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t12 = typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t13 =
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::difference_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5> f1() {
  return std::unordered_multimap<T1, T2, T3, T4, T5>();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f2(typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a0,
   const typename std::unordered_multimap<T1, T2, T3, T4, T5>::hasher &a1,
   const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_equal &a2,
   const typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
       &a3) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f3(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f4(std::unordered_multimap<T1, T2, T3, T4, T5> &&a0) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f5(const typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
       &a0) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f6(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
   const typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
       &a1) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f7(std::unordered_multimap<T1, T2, T3, T4, T5> &&a0,
   const typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
       &a1) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f8(std::initializer_list<
       typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type>
       a0,
   typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1,
   const typename std::unordered_multimap<T1, T2, T3, T4, T5>::hasher &a2,
   const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_equal &a3,
   const typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
       &a4) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f9(typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a0,
   const typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
       &a1) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f10(typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::hasher &a1,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
        &a2) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f11(std::initializer_list<
        typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type>
        a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
        &a2) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f12(std::initializer_list<
        typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type>
        a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::hasher &a2,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
        &a3) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5> &
f13(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    const std::unordered_multimap<T1, T2, T3, T4, T5> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5> &
f14(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    std::unordered_multimap<T1, T2, T3, T4, T5> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5> &
f15(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    std::initializer_list<
        typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type>
        a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
f16(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f17(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f18(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f19(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f20(std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator
f21(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator
f22(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f23(std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator
f24(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator
f25(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f26(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type
        &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f27(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f28(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type
        &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f29(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f30(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
         std::initializer_list<
             typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type>
             a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f31(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f32(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f33(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f34(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f35(std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f36(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
         std::unordered_multimap<T1, T2, T3, T4, T5> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::hasher
f37(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.hash_function();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_equal
f38(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.key_eq();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f39(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator
f40(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f41(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator,
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator>
f42(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator,
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator>
f43(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f44(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.bucket_count();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f45(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.max_bucket_count();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f46(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.bucket_size(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f47(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.bucket(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::local_iterator
f48(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.begin(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_local_iterator
f49(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.begin(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_local_iterator
f50(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.cbegin(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::local_iterator
f51(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.end(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_local_iterator
f52(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.end(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_local_iterator
f53(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.cend(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
float f54(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.load_factor();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
float f55(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.max_load_factor();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f56(std::unordered_multimap<T1, T2, T3, T4, T5> &a0, float a1) {
  return a0.max_load_factor(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f57(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
         typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.rehash(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f58(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
         typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.reserve(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_multimap<T1, T2, T3, T4, T5>
f59(T6 a0, T6 a1,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a2,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::hasher &a3,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_equal &a4,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
        &a5) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_multimap<T1, T2, T3, T4, T5>
f60(T6 a0, T6 a1,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a2,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
        &a3) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_multimap<T1, T2, T3, T4, T5>
f61(T6 a0, T6 a1,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a2,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::hasher &a3,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::allocator_type
        &a4) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f62(std::unordered_multimap<T1, T2, T3, T4, T5> &a0, T6 &&...a1) {
  return a0.template emplace<T6...>(std::forward<T6>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f63(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    T6 &&...a2) {
  return a0.template emplace_hint<T6...>(a1, std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f64(std::unordered_multimap<T1, T2, T3, T4, T5> &a0, T6 a1, T6 a2) {
  return a0.template insert<T6>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t14 = typename std::unordered_multimap<T1, T2, T3, T4,
                                             T5>::value_type::first_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t15 = typename std::unordered_multimap<T1, T2, T3, T4,
                                             T5>::value_type::second_type;
