#include <algorithm>
#include <initializer_list>
#include <unordered_set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
bool f3(const std::unordered_multiset<T1, T2, T3, T4> &a0,
        const std::unordered_multiset<T1, T2, T3, T4> &a1) {
  return std::operator== <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f4(const std::unordered_set<T1, T2, T3, T4> &a0,
        const std::unordered_set<T1, T2, T3, T4> &a1) {
  return std::operator== <T1, T2, T3, T4>(a0, a1);
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
using t11 = typename std::unordered_set<T1, T2, T3, T4>::iterator;

template <typename T1, typename T2, typename T3, typename T4>
using t12 = typename std::unordered_set<T1, T2, T3, T4>::const_iterator;

template <typename T1, typename T2, typename T3, typename T4>
using t13 = typename std::unordered_set<T1, T2, T3, T4>::local_iterator;

template <typename T1, typename T2, typename T3, typename T4>
using t14 = typename std::unordered_set<T1, T2, T3, T4>::const_local_iterator;

template <typename T1, typename T2, typename T3, typename T4>
using t15 = typename std::unordered_set<T1, T2, T3, T4>::size_type;

template <typename T1, typename T2, typename T3, typename T4>
using t16 = typename std::unordered_set<T1, T2, T3, T4>::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4> f7() {
  return std::unordered_set<T1, T2, T3, T4>();
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f8(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return std::unordered_set<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4> f9(std::unordered_set<T1, T2, T3, T4> &&a0) {
  return std::unordered_set<T1, T2, T3, T4>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4> &
f10(std::unordered_set<T1, T2, T3, T4> &a0,
    const std::unordered_set<T1, T2, T3, T4> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4> &
f11(std::unordered_set<T1, T2, T3, T4> &a0,
    std::unordered_set<T1, T2, T3, T4> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
bool f12(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::size_type
f13(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::size_type
f14(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f15(std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::const_iterator
f16(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f17(std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::const_iterator
f18(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::const_iterator
f19(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::const_iterator
f20(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f21(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f22(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator a1,
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
void f23(std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3, typename T4>
void f24(std::unordered_set<T1, T2, T3, T4> &a0,
         std::unordered_set<T1, T2, T3, T4> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::size_type
f25(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.bucket_count();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::size_type
f26(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.max_bucket_count();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::size_type
f27(const std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.bucket_size(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::local_iterator
f28(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.begin(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::const_local_iterator
f29(const std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.begin(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::const_local_iterator
f30(const std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.cbegin(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::local_iterator
f31(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.end(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::const_local_iterator
f32(const std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.end(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::const_local_iterator
f33(const std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.cend(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
float f34(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.load_factor();
}

template <typename T1, typename T2, typename T3, typename T4>
float f35(const std::unordered_set<T1, T2, T3, T4> &a0) {
  return a0.max_load_factor();
}

template <typename T1, typename T2, typename T3, typename T4>
void f36(std::unordered_set<T1, T2, T3, T4> &a0, float a1) {
  return a0.max_load_factor(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f37(std::unordered_set<T1, T2, T3, T4> &a0,
         typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.rehash(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f38(std::unordered_set<T1, T2, T3, T4> &a0,
         typename std::unordered_set<T1, T2, T3, T4>::size_type a1) {
  return a0.reserve(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
std::pair<typename std::unordered_set<T1, T2, T3, T4>::iterator, bool>
f39(std::unordered_set<T1, T2, T3, T4> &a0, T5 &&...a1) {
  return a0.template emplace<T5...>(std::forward<T5>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f40(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator a1,
    T5 &&...a2) {
  return a0.template emplace_hint<T5...>(a1, std::forward<T5>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f41(std::unordered_set<T1, T2, T3, T4> &a0, T5 a1, T5 a2) {
  return a0.template insert<T5>(a1, a2);
}
