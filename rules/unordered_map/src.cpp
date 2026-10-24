#include <algorithm>
#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f3(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
        const std::unordered_map<T1, T2, T3, T4, T5> &a1) {
  return std::operator== <T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f4(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
        const std::unordered_multimap<T1, T2, T3, T4, T5> &a1) {
  return std::operator== <T1, T2, T3, T4, T5>(a0, a1);
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
using t12 = typename std::unordered_map<T1, T2, T3, T4, T5>::iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t13 = typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t14 = typename std::unordered_map<T1, T2, T3, T4, T5>::local_iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t15 =
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_local_iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t16 = typename std::unordered_map<T1, T2, T3, T4, T5>::size_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t17 = typename std::unordered_map<T1, T2, T3, T4, T5>::difference_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5> f7() {
  return std::unordered_map<T1, T2, T3, T4, T5>();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f8(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f9(std::unordered_map<T1, T2, T3, T4, T5> &&a0) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5> &
f10(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const std::unordered_map<T1, T2, T3, T4, T5> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5> &
f11(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    std::unordered_map<T1, T2, T3, T4, T5> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f12(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f13(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f14(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f15(std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator
f16(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator
f17(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f18(std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator
f19(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator
f20(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f21(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1) {
  return a0.erase(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f22(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::iterator a1) {
  return a0.erase(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f23(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a2) {
  return a0.erase(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f24(std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f25(std::unordered_map<T1, T2, T3, T4, T5> &a0,
         std::unordered_map<T1, T2, T3, T4, T5> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f26(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.bucket_count();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f27(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.max_bucket_count();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f28(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.bucket_size(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::local_iterator
f29(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.begin(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_local_iterator
f30(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.begin(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_local_iterator
f31(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.cbegin(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::local_iterator
f32(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.end(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_local_iterator
f33(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.end(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_local_iterator
f34(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.cend(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
float f35(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.load_factor();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
float f36(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.max_load_factor();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f37(std::unordered_map<T1, T2, T3, T4, T5> &a0, float a1) {
  return a0.max_load_factor(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f38(std::unordered_map<T1, T2, T3, T4, T5> &a0,
         typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.rehash(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f39(std::unordered_map<T1, T2, T3, T4, T5> &a0,
         typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1) {
  return a0.reserve(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f40(std::unordered_map<T1, T2, T3, T4, T5> &a0, T6 &&...a1) {
  return a0.template emplace<T6...>(std::forward<T6>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f41(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
    T6 &&...a2) {
  return a0.template emplace_hint<T6...>(std::move(a1),
                                         std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f42(std::unordered_map<T1, T2, T3, T4, T5> &a0, T6 a1, T6 a2) {
  return a0.template insert<T6>(std::move(a1), std::move(a2));
}
