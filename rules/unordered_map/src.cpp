#include <algorithm>
#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f3(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
        const std::unordered_map<T1, T2, T3, T4, T5> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f4(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
        const std::unordered_multimap<T1, T2, T3, T4, T5> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f5(std::unordered_map<T1, T2, T3, T4, T5> &a0,
        std::unordered_map<T1, T2, T3, T4, T5> &a1) {
  return std::swap<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f6(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
        std::unordered_multimap<T1, T2, T3, T4, T5> &a1) {
  return std::swap<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t1 = std::unordered_map<T1, T2, T3, T4, T5>;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t2 = typename std::unordered_map<T1, T2, T3, T4, T5>::key_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t3 = typename std::unordered_map<T1, T2, T3, T4, T5>::value_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t4 = typename std::unordered_map<T1, T2, T3, T4, T5>::mapped_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t5 = typename std::unordered_map<T1, T2, T3, T4, T5>::hasher;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t6 = typename std::unordered_map<T1, T2, T3, T4, T5>::key_equal;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t7 = typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t8 = typename std::unordered_map<T1, T2, T3, T4, T5>::pointer;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t9 = typename std::unordered_map<T1, T2, T3, T4, T5>::const_pointer;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t10 = typename std::unordered_map<T1, T2, T3, T4, T5>::reference;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t11 = typename std::unordered_map<T1, T2, T3, T4, T5>::const_reference;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t12 = typename std::unordered_map<T1, T2, T3, T4, T5>::size_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t13 = typename std::unordered_map<T1, T2, T3, T4, T5>::difference_type;

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
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
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
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f15(typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a1) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f16(typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::hasher &a1,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a2) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f17(std::initializer_list<
        typename std::unordered_map<T1, T2, T3, T4, T5>::value_type>
        a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a2) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f18(std::initializer_list<
        typename std::unordered_map<T1, T2, T3, T4, T5>::value_type>
        a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::hasher &a2,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a3) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
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
  return a0.operator=(a1);
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
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f32(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::value_type &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f33(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::value_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f34(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::value_type &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f35(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::unordered_map<T1, T2, T3, T4, T5>::value_type &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f36(std::unordered_map<T1, T2, T3, T4, T5> &a0,
         std::initializer_list<
             typename std::unordered_map<T1, T2, T3, T4, T5>::value_type>
             a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f37(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f38(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f39(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f40(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f41(std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f42(std::unordered_map<T1, T2, T3, T4, T5> &a0,
         std::unordered_map<T1, T2, T3, T4, T5> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::hasher
f43(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.hash_function();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::key_equal
f44(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.key_eq();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f45(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator
f46(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f47(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator,
          typename std::unordered_map<T1, T2, T3, T4, T5>::iterator>
f48(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator,
          typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator>
f49(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::mapped_type &
f50(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::mapped_type &
f51(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &&a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::mapped_type &
f52(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
const typename std::unordered_map<T1, T2, T3, T4, T5>::mapped_type &
f53(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f54(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.bucket_count();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f55(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.max_bucket_count();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f56(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.bucket_size(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f57(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.bucket(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::local_iterator
f58(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.begin(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_local_iterator
f59(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.begin(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_local_iterator
f60(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.cbegin(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::local_iterator
f61(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.end(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_local_iterator
f62(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.end(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_local_iterator
f63(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.cend(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
float f64(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.load_factor();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
float f65(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.max_load_factor();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f66(std::unordered_map<T1, T2, T3, T4, T5> &a0, float a1) {
  return a0.max_load_factor(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f67(std::unordered_map<T1, T2, T3, T4, T5> &a0,
         typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.rehash(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f68(std::unordered_map<T1, T2, T3, T4, T5> &a0,
         typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.reserve(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5>
f69(T6 a0, T6 a1, typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a2,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::hasher &a3,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_equal &a4,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a5) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5>
f70(T6 a0, T6 a1, typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a2,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a3) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5>
f71(T6 a0, T6 a1, typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a2,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::hasher &a3,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type &a4) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f72(std::unordered_map<T1, T2, T3, T4, T5> &a0, T6 &&...a1) {
  return a0.template emplace<T6...>(std::forward<T6>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f73(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
    T6 &&...a2) {
  return a0.template emplace_hint<T6...>(a1, std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f74(std::unordered_map<T1, T2, T3, T4, T5> &a0, T6 a1, T6 a2) {
  return a0.template insert<T6>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t14 =
    typename std::unordered_map<T1, T2, T3, T4, T5>::value_type::first_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t15 =
    typename std::unordered_map<T1, T2, T3, T4, T5>::value_type::second_type;
