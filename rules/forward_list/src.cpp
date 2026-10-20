#include "../common/init.h"
#include "../common/traits.h"
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
using t7 = typename std::forward_list<T1, T2>::size_type;

template <typename T1, typename T2>
using t8 = typename std::forward_list<T1, T2>::difference_type;

template <typename T1, typename T2>
using t9 = typename std::forward_list<T1, T2>::allocator_type;

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
std::forward_list<T1, T2> f5(typename std::forward_list<T1, T2>::size_type a0,
                             const T2 &a1) {
  return std::forward_list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f6(typename std::forward_list<T1, T2>::size_type a0,
                             const T1 &a1, const T2 &a2) {
  return std::forward_list<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f7(const std::forward_list<T1, T2> &a0) {
  return std::forward_list<T1, T2>(a0);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f8(std::forward_list<T1, T2> &&a0) {
  return std::forward_list<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f9(std::initializer_list<T1> a0, const T2 &a1) {
  return std::forward_list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> &f10(std::forward_list<T1, T2> &a0,
                               const std::forward_list<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> &f11(std::forward_list<T1, T2> &a0,
                               std::forward_list<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::forward_list<T1, T2> &f12(std::forward_list<T1, T2> &a0,
                               std::initializer_list<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
void f13(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::size_type a1, const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
void f14(std::forward_list<T1, T2> &a0, std::initializer_list<T1> a1) {
  return a0.assign(a1);
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::allocator_type
f15(const std::forward_list<T1, T2> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f16(std::forward_list<T1, T2> &a0) {
  return a0.before_begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f17(const std::forward_list<T1, T2> &a0) {
  return a0.before_begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f18(std::forward_list<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f19(const std::forward_list<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f20(std::forward_list<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f21(const std::forward_list<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f22(const std::forward_list<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f23(const std::forward_list<T1, T2> &a0) {
  return a0.cbefore_begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f24(const std::forward_list<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
bool f25(const std::forward_list<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::size_type
f26(const std::forward_list<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::value_type &
f27(std::forward_list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
const typename std::forward_list<T1, T2>::value_type &
f28(const std::forward_list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
void f29(std::forward_list<T1, T2> &a0, const T1 &a1) {
  return a0.push_front(a1);
}

template <typename T1, typename T2>
void f30(std::forward_list<T1, T2> &a0, T1 &&a1) {
  return a0.push_front(std::move(a1));
}

template <typename T1, typename T2> void f31(std::forward_list<T1, T2> &a0) {
  return a0.pop_front();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f32(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, const T1 &a2) {
  return a0.insert_after(a1, a2);
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f33(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, T1 &&a2) {
  return a0.insert_after(a1, std::move(a2));
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f34(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1,
    typename std::forward_list<T1, T2>::size_type a2, const T1 &a3) {
  return a0.insert_after(a1, a2, a3);
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f35(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1,
    std::initializer_list<T1> a2) {
  return a0.insert_after(a1, a2);
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f36(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1) {
  return a0.erase_after(a1);
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f37(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1,
    typename std::forward_list<T1, T2>::const_iterator a2) {
  return a0.erase_after(a1, a2);
}

template <typename T1, typename T2>
void f38(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2>
void f39(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::size_type a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f40(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::size_type a1,
         const typename std::forward_list<T1, T2>::value_type &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2> void f41(std::forward_list<T1, T2> &a0) {
  return a0.clear();
}

template <typename T1, typename T2>
void f42(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &&a2) {
  return a0.splice_after(a1, std::move(a2));
}

template <typename T1, typename T2>
void f43(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &a2) {
  return a0.splice_after(a1, a2);
}

template <typename T1, typename T2>
void f44(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &&a2,
         typename std::forward_list<T1, T2>::const_iterator a3) {
  return a0.splice_after(a1, std::move(a2), a3);
}

template <typename T1, typename T2>
void f45(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &a2,
         typename std::forward_list<T1, T2>::const_iterator a3) {
  return a0.splice_after(a1, a2, a3);
}

template <typename T1, typename T2>
void f46(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &&a2,
         typename std::forward_list<T1, T2>::const_iterator a3,
         typename std::forward_list<T1, T2>::const_iterator a4) {
  return a0.splice_after(a1, std::move(a2), a3, a4);
}

template <typename T1, typename T2>
void f47(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &a2,
         typename std::forward_list<T1, T2>::const_iterator a3,
         typename std::forward_list<T1, T2>::const_iterator a4) {
  return a0.splice_after(a1, a2, a3, a4);
}

template <typename T1, typename T2>
void f48(std::forward_list<T1, T2> &a0, const T1 &a1) {
  return a0.remove(a1);
}

template <typename T1, typename T2> void f49(std::forward_list<T1, T2> &a0) {
  return a0.unique();
}

template <typename T1, typename T2>
void f50(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &&a1) {
  return a0.merge(std::move(a1));
}

template <typename T1, typename T2>
void f51(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &a1) {
  return a0.merge(a1);
}

template <typename T1, typename T2> void f52(std::forward_list<T1, T2> &a0) {
  return a0.sort();
}

template <typename T1, typename T2> void f53(std::forward_list<T1, T2> &a0) {
  return a0.reverse();
}

template <typename T1, typename T2, typename T3>
std::forward_list<T1, T2> f54(T3 a0, T3 a1, const T2 &a2) {
  return std::forward_list<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
void f55(std::forward_list<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(a1, a2);
}

template <typename T1, typename T2, typename... T3>
void f56(std::forward_list<T1, T2> &a0, T3 &&...a1) {
  return a0.template emplace_front<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename... T3>
typename std::forward_list<T1, T2>::iterator
f57(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, T3 &&...a2) {
  return a0.template emplace_after<T3...>(a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2, typename T3>
typename std::forward_list<T1, T2>::iterator
f58(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, T3 a2, T3 a3) {
  return a0.template insert_after<T3>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
void f59(std::forward_list<T1, T2> &a0, T3 a1) {
  return a0.template remove_if<T3>(a1);
}

template <typename T1, typename T2, typename T3>
void f60(std::forward_list<T1, T2> &a0, T3 a1) {
  return a0.template unique<T3>(a1);
}

template <typename T1, typename T2, typename T3>
void f61(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &&a1, T3 a2) {
  return a0.template merge<T3>(std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
void f62(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &a1, T3 a2) {
  return a0.template merge<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3>
void f63(std::forward_list<T1, T2> &a0, T3 a1) {
  return a0.template sort<T3>(a1);
}

template <typename T1, typename T2>
using t10 = typename std::forward_list<T1, T2>::iterator::value_type;

template <typename T1, typename T2>
using t11 = typename std::forward_list<T1, T2>::iterator::difference_type;

template <typename T1, typename T2>
using t12 = typename std::forward_list<T1, T2>::iterator::iterator_category;

template <typename T1, typename T2>
using t13 = typename std::forward_list<T1, T2>::iterator::pointer;

template <typename T1, typename T2>
using t14 = typename std::forward_list<T1, T2>::iterator::reference;

template <typename T1, typename T2>
using t15 = typename std::forward_list<T1, T2>::const_iterator::value_type;

template <typename T1, typename T2>
using t16 = typename std::forward_list<T1, T2>::const_iterator::difference_type;

template <typename T1, typename T2>
using t17 =
    typename std::forward_list<T1, T2>::const_iterator::iterator_category;

template <typename T1, typename T2>
using t18 = typename std::forward_list<T1, T2>::const_iterator::pointer;

template <typename T1, typename T2>
using t19 = typename std::forward_list<T1, T2>::const_iterator::reference;

template <typename T1, typename T2>
bool f64(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
void f65(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
