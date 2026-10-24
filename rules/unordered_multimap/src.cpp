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
using t12 = typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t13 =
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t14 =
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::local_iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t15 =
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_local_iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t16 = typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t17 =
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::difference_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5> f1() {
  return std::unordered_multimap<T1, T2, T3, T4, T5>();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f2(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f3(std::unordered_multimap<T1, T2, T3, T4, T5> &&a0) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5> &
f4(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
   const std::unordered_multimap<T1, T2, T3, T4, T5> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5> &
f5(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
   std::unordered_multimap<T1, T2, T3, T4, T5> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f6(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f7(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f8(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f9(std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator
f10(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator
f11(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f12(std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator
f13(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator
f14(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f15(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator a1) {
  return a0.erase(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f16(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator a1) {
  return a0.erase(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f17(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator a2) {
  return a0.erase(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f18(std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f19(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
         std::unordered_multimap<T1, T2, T3, T4, T5> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f20(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.bucket_count();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f21(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.max_bucket_count();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f22(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.bucket_size(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::local_iterator
f23(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.begin(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_local_iterator
f24(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.begin(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_local_iterator
f25(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.cbegin(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::local_iterator
f26(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.end(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_local_iterator
f27(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.end(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_local_iterator
f28(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.cend(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
float f29(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.load_factor();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
float f30(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0) {
  return a0.max_load_factor();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f31(std::unordered_multimap<T1, T2, T3, T4, T5> &a0, float a1) {
  return a0.max_load_factor(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f32(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
         typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.rehash(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f33(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
         typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.reserve(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f34(std::unordered_multimap<T1, T2, T3, T4, T5> &a0, T6 &&...a1) {
  return a0.template emplace<T6...>(std::forward<T6>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f35(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    T6 &&...a2) {
  return a0.template emplace_hint<T6...>(std::move(a1),
                                         std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f36(std::unordered_multimap<T1, T2, T3, T4, T5> &a0, T6 a1, T6 a2) {
  return a0.template insert<T6>(std::move(a1), std::move(a2));
}
