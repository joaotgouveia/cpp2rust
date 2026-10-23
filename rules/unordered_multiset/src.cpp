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
f2(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return std::unordered_multiset<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_multiset<T1, T2, T3, T4>
f3(std::unordered_multiset<T1, T2, T3, T4> &&a0) {
  return std::unordered_multiset<T1, T2, T3, T4>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_multiset<T1, T2, T3, T4> &
f4(std::unordered_multiset<T1, T2, T3, T4> &a0,
   const std::unordered_multiset<T1, T2, T3, T4> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_multiset<T1, T2, T3, T4> &
f5(std::unordered_multiset<T1, T2, T3, T4> &a0,
   std::unordered_multiset<T1, T2, T3, T4> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
bool f6(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::size_type
f7(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::size_type
f8(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::iterator
f9(std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator
f10(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::iterator
f11(std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator
f12(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator
f13(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator
f14(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::iterator
f15(std::unordered_multiset<T1, T2, T3, T4> &a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::iterator
f16(std::unordered_multiset<T1, T2, T3, T4> &a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator a1,
    typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
void f17(std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3, typename T4>
void f18(std::unordered_multiset<T1, T2, T3, T4> &a0,
         std::unordered_multiset<T1, T2, T3, T4> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::size_type
f19(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.bucket_count();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::size_type
f20(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.max_bucket_count();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::size_type
f21(const std::unordered_multiset<T1, T2, T3, T4> &a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::size_type a1) {
  return a0.bucket_size(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::local_iterator
f22(std::unordered_multiset<T1, T2, T3, T4> &a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::size_type a1) {
  return a0.begin(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::const_local_iterator
f23(const std::unordered_multiset<T1, T2, T3, T4> &a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::size_type a1) {
  return a0.begin(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::const_local_iterator
f24(const std::unordered_multiset<T1, T2, T3, T4> &a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::size_type a1) {
  return a0.cbegin(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::local_iterator
f25(std::unordered_multiset<T1, T2, T3, T4> &a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::size_type a1) {
  return a0.end(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::const_local_iterator
f26(const std::unordered_multiset<T1, T2, T3, T4> &a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::size_type a1) {
  return a0.end(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::const_local_iterator
f27(const std::unordered_multiset<T1, T2, T3, T4> &a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::size_type a1) {
  return a0.cend(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
float f28(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.load_factor();
}

template <typename T1, typename T2, typename T3, typename T4>
float f29(const std::unordered_multiset<T1, T2, T3, T4> &a0) {
  return a0.max_load_factor();
}

template <typename T1, typename T2, typename T3, typename T4>
void f30(std::unordered_multiset<T1, T2, T3, T4> &a0, float a1) {
  return a0.max_load_factor(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f31(std::unordered_multiset<T1, T2, T3, T4> &a0,
         typename std::unordered_multiset<T1, T2, T3, T4>::size_type a1) {
  return a0.rehash(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f32(std::unordered_multiset<T1, T2, T3, T4> &a0,
         typename std::unordered_multiset<T1, T2, T3, T4>::size_type a1) {
  return a0.reserve(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
typename std::unordered_multiset<T1, T2, T3, T4>::iterator
f33(std::unordered_multiset<T1, T2, T3, T4> &a0, T5 &&...a1) {
  return a0.template emplace<T5...>(std::forward<T5>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
typename std::unordered_multiset<T1, T2, T3, T4>::iterator
f34(std::unordered_multiset<T1, T2, T3, T4> &a0,
    typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator a1,
    T5 &&...a2) {
  return a0.template emplace_hint<T5...>(a1, std::forward<T5>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f35(std::unordered_multiset<T1, T2, T3, T4> &a0, T5 a1, T5 a2) {
  return a0.template insert<T5>(a1, a2);
}
