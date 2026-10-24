#include <algorithm>
#include <forward_list>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2> using t1 = std::forward_list<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::forward_list<T1, T2>::value_type;

template <typename T1, typename T2>
using t3 = typename std::forward_list<T1, T2>::pointer;

template <typename T1, typename T2>
using t4 = typename std::forward_list<T1, T2>::const_pointer;

template <typename T1, typename T2>
using t5 = typename std::forward_list<T1, T2>::reference;

template <typename T1, typename T2>
using t6 = typename std::forward_list<T1, T2>::const_reference;

template <typename T1, typename T2>
using t7 = typename std::forward_list<T1, T2>::iterator;

template <typename T1, typename T2>
using t8 = typename std::forward_list<T1, T2>::const_iterator;

template <typename T1, typename T2>
using t9 = typename std::forward_list<T1, T2>::size_type;

template <typename T1, typename T2>
using t10 = typename std::forward_list<T1, T2>::difference_type;

template <typename T1, typename T2>
using t11 = typename std::forward_list<T1, T2>::allocator_type;

template <typename T1, typename T2> std::forward_list<T1, T2> f1() {
  return std::forward_list<T1, T2>();
}

template <typename T1, typename T2> std::forward_list<T1, T2> f2(const T2 &a0) {
  return std::forward_list<T1, T2>(a0);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f3(const std::forward_list<T1, T2> &a0,
                             const T2 &a1) {
  return std::forward_list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f4(std::forward_list<T1, T2> &&a0, const T2 &a1) {
  return std::forward_list<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f5(const std::forward_list<T1, T2> &a0) {
  return std::forward_list<T1, T2>(a0);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f6(std::forward_list<T1, T2> &&a0) {
  return std::forward_list<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f7(std::initializer_list<T1> a0, const T2 &a1) {
  return std::forward_list<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> &f8(std::forward_list<T1, T2> &a0,
                              const std::forward_list<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> &f9(std::forward_list<T1, T2> &a0,
                              std::forward_list<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::forward_list<T1, T2> &f10(std::forward_list<T1, T2> &a0,
                               std::initializer_list<T1> a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
void f11(std::forward_list<T1, T2> &a0, std::initializer_list<T1> a1) {
  return a0.assign(std::move(a1));
}

template <typename T1, typename T2>
T2 f12(const std::forward_list<T1, T2> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f13(std::forward_list<T1, T2> &a0) {
  return a0.before_begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f14(const std::forward_list<T1, T2> &a0) {
  return a0.before_begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f15(std::forward_list<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f16(const std::forward_list<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f17(std::forward_list<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f18(const std::forward_list<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f19(const std::forward_list<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f20(const std::forward_list<T1, T2> &a0) {
  return a0.cbefore_begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f21(const std::forward_list<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
bool f22(const std::forward_list<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2> T1 &f23(std::forward_list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
const T1 &f24(const std::forward_list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
void f25(std::forward_list<T1, T2> &a0, const T1 &a1) {
  return a0.push_front(a1);
}

template <typename T1, typename T2>
void f26(std::forward_list<T1, T2> &a0, T1 &&a1) {
  return a0.push_front(std::move(a1));
}

template <typename T1, typename T2> void f27(std::forward_list<T1, T2> &a0) {
  return a0.pop_front();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f28(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, const T1 &a2) {
  return a0.insert_after(std::move(a1), a2);
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f29(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, T1 &&a2) {
  return a0.insert_after(std::move(a1), std::move(a2));
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f30(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1,
    std::initializer_list<T1> a2) {
  return a0.insert_after(std::move(a1), std::move(a2));
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f31(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1) {
  return a0.erase_after(std::move(a1));
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f32(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1,
    typename std::forward_list<T1, T2>::const_iterator a2) {
  return a0.erase_after(std::move(a1), std::move(a2));
}

template <typename T1, typename T2>
void f33(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2> void f34(std::forward_list<T1, T2> &a0) {
  return a0.clear();
}

template <typename T1, typename T2>
void f35(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &&a2) {
  return a0.splice_after(std::move(a1), std::move(a2));
}

template <typename T1, typename T2>
void f36(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &a2) {
  return a0.splice_after(std::move(a1), a2);
}

template <typename T1, typename T2>
void f37(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &&a2,
         typename std::forward_list<T1, T2>::const_iterator a3) {
  return a0.splice_after(std::move(a1), std::move(a2), std::move(a3));
}

template <typename T1, typename T2>
void f38(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &a2,
         typename std::forward_list<T1, T2>::const_iterator a3) {
  return a0.splice_after(std::move(a1), a2, std::move(a3));
}

template <typename T1, typename T2>
void f39(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &&a2,
         typename std::forward_list<T1, T2>::const_iterator a3,
         typename std::forward_list<T1, T2>::const_iterator a4) {
  return a0.splice_after(std::move(a1), std::move(a2), std::move(a3),
                         std::move(a4));
}

template <typename T1, typename T2>
void f40(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &a2,
         typename std::forward_list<T1, T2>::const_iterator a3,
         typename std::forward_list<T1, T2>::const_iterator a4) {
  return a0.splice_after(std::move(a1), a2, std::move(a3), std::move(a4));
}

template <typename T1, typename T2>
void f41(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &&a1) {
  return a0.merge(std::move(a1));
}

template <typename T1, typename T2>
void f42(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &a1) {
  return a0.merge(a1);
}

template <typename T1, typename T2> void f43(std::forward_list<T1, T2> &a0) {
  return a0.sort();
}

template <typename T1, typename T2> void f44(std::forward_list<T1, T2> &a0) {
  return a0.reverse();
}

template <typename T1, typename T2, typename... T3>
typename std::forward_list<T1, T2>::iterator
f45(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, T3 &&...a2) {
  return a0.template emplace_after<T3...>(std::move(a1),
                                          std::forward<T3>(a2)...);
}

template <typename T1, typename T2, typename T3>
void f46(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &&a1, T3 a2) {
  return a0.template merge<T3>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f47(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &a1, T3 a2) {
  return a0.template merge<T3>(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f48(std::forward_list<T1, T2> &a0, T3 a1) {
  return a0.template sort<T3>(std::move(a1));
}

template <typename T1, typename T2>
bool f49(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
void f50(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
