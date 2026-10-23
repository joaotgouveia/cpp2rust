#include <algorithm>
#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::multiset<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = typename std::multiset<T1, T2, T3>::key_type;

template <typename T1, typename T2, typename T3>
using t3 = typename std::multiset<T1, T2, T3>::value_type;

template <typename T1, typename T2, typename T3>
using t4 = typename std::multiset<T1, T2, T3>::key_compare;

template <typename T1, typename T2, typename T3>
using t5 = typename std::multiset<T1, T2, T3>::value_compare;

template <typename T1, typename T2, typename T3>
using t6 = typename std::multiset<T1, T2, T3>::allocator_type;

template <typename T1, typename T2, typename T3>
using t7 = typename std::multiset<T1, T2, T3>::pointer;

template <typename T1, typename T2, typename T3>
using t8 = typename std::multiset<T1, T2, T3>::const_pointer;

template <typename T1, typename T2, typename T3>
using t9 = typename std::multiset<T1, T2, T3>::reference;

template <typename T1, typename T2, typename T3>
using t10 = typename std::multiset<T1, T2, T3>::const_reference;

template <typename T1, typename T2, typename T3>
using t11 = typename std::multiset<T1, T2, T3>::size_type;

template <typename T1, typename T2, typename T3>
using t12 = typename std::multiset<T1, T2, T3>::difference_type;

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3> f1() {
  return std::multiset<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3> f2(const std::multiset<T1, T2, T3> &a0) {
  return std::multiset<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3> f3(std::multiset<T1, T2, T3> &&a0) {
  return std::multiset<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3>
f4(const typename std::multiset<T1, T2, T3>::allocator_type &a0) {
  return std::multiset<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3>
f5(const std::multiset<T1, T2, T3> &a0,
   const typename std::multiset<T1, T2, T3>::allocator_type &a1) {
  return std::multiset<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3>
f6(std::multiset<T1, T2, T3> &&a0,
   const typename std::multiset<T1, T2, T3>::allocator_type &a1) {
  return std::multiset<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3>
f7(std::initializer_list<typename std::multiset<T1, T2, T3>::value_type> a0,
   const typename std::multiset<T1, T2, T3>::allocator_type &a1) {
  return std::multiset<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3> &f8(std::multiset<T1, T2, T3> &a0,
                              const std::multiset<T1, T2, T3> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3> &f9(std::multiset<T1, T2, T3> &a0,
                              std::multiset<T1, T2, T3> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3> &
f10(std::multiset<T1, T2, T3> &a0,
    std::initializer_list<typename std::multiset<T1, T2, T3>::value_type> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::key_compare
f11(const std::multiset<T1, T2, T3> &a0) {
  return a0.key_comp();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::value_compare
f12(const std::multiset<T1, T2, T3> &a0) {
  return a0.value_comp();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::allocator_type
f13(const std::multiset<T1, T2, T3> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3>
bool f14(const std::multiset<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::size_type
f15(const std::multiset<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::size_type
f16(const std::multiset<T1, T2, T3> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3>
void f17(std::multiset<T1, T2, T3> &a0, std::multiset<T1, T2, T3> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f18(std::multiset<T1, T2, T3> &a0,
    const typename std::multiset<T1, T2, T3>::value_type &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f19(std::multiset<T1, T2, T3> &a0,
    typename std::multiset<T1, T2, T3>::value_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f20(std::multiset<T1, T2, T3> &a0,
    typename std::multiset<T1, T2, T3>::const_iterator a1,
    const typename std::multiset<T1, T2, T3>::value_type &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f21(std::multiset<T1, T2, T3> &a0,
    typename std::multiset<T1, T2, T3>::const_iterator a1,
    typename std::multiset<T1, T2, T3>::value_type &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f22(
    std::multiset<T1, T2, T3> &a0,
    std::initializer_list<typename std::multiset<T1, T2, T3>::value_type> a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f23(std::multiset<T1, T2, T3> &a0,
    typename std::multiset<T1, T2, T3>::const_iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::size_type
f24(std::multiset<T1, T2, T3> &a0,
    const typename std::multiset<T1, T2, T3>::key_type &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f25(std::multiset<T1, T2, T3> &a0,
    typename std::multiset<T1, T2, T3>::const_iterator a1,
    typename std::multiset<T1, T2, T3>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2, typename T3>
void f26(std::multiset<T1, T2, T3> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::size_type
f27(const std::multiset<T1, T2, T3> &a0,
    const typename std::multiset<T1, T2, T3>::key_type &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f28(std::multiset<T1, T2, T3> &a0,
    const typename std::multiset<T1, T2, T3>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::const_iterator
f29(const std::multiset<T1, T2, T3> &a0,
    const typename std::multiset<T1, T2, T3>::key_type &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f30(std::multiset<T1, T2, T3> &a0,
    const typename std::multiset<T1, T2, T3>::key_type &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::const_iterator
f31(const std::multiset<T1, T2, T3> &a0,
    const typename std::multiset<T1, T2, T3>::key_type &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f32(std::multiset<T1, T2, T3> &a0,
    const typename std::multiset<T1, T2, T3>::key_type &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::const_iterator
f33(const std::multiset<T1, T2, T3> &a0,
    const typename std::multiset<T1, T2, T3>::key_type &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::multiset<T1, T2, T3>::iterator,
          typename std::multiset<T1, T2, T3>::iterator>
f34(std::multiset<T1, T2, T3> &a0,
    const typename std::multiset<T1, T2, T3>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::multiset<T1, T2, T3>::const_iterator,
          typename std::multiset<T1, T2, T3>::const_iterator>
f35(const std::multiset<T1, T2, T3> &a0,
    const typename std::multiset<T1, T2, T3>::key_type &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multiset<T1, T2, T3>
f36(T4 a0, T4 a1,
    const typename std::multiset<T1, T2, T3>::allocator_type &a2) {
  return std::multiset<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename... T4>
typename std::multiset<T1, T2, T3>::iterator f37(std::multiset<T1, T2, T3> &a0,
                                                 T4 &&...a1) {
  return a0.template emplace<T4...>(std::forward<T4>(a1)...);
}

template <typename T1, typename T2, typename T3, typename... T4>
typename std::multiset<T1, T2, T3>::iterator
f38(std::multiset<T1, T2, T3> &a0,
    typename std::multiset<T1, T2, T3>::const_iterator a1, T4 &&...a2) {
  return a0.template emplace_hint<T4...>(a1, std::forward<T4>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4>
void f39(std::multiset<T1, T2, T3> &a0, T4 a1, T4 a2) {
  return a0.template insert<T4>(a1, a2);
}
