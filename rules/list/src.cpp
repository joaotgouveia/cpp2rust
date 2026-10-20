#include "../common/init.h"
#include "../common/traits.h"
#include <initializer_list>
#include <list>
#include <utility>

template <typename T1, typename T2> using t1 = std::list<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::list<T1, T2>::value_type;

template <typename T1, typename T2>
using t3 = typename std::list<T1, T2>::pointer;

template <typename T1, typename T2>
using t4 = typename std::list<T1, T2>::const_pointer;

template <typename T1, typename T2>
using t5 = typename std::list<T1, T2>::reference;

template <typename T1, typename T2>
using t6 = typename std::list<T1, T2>::const_reference;

template <typename T1, typename T2>
using t7 = typename std::list<T1, T2>::size_type;

template <typename T1, typename T2>
using t8 = typename std::list<T1, T2>::difference_type;

template <typename T1, typename T2>
using t9 = typename std::list<T1, T2>::allocator_type;

template <typename T1, typename T2> std::list<T1, T2> f1() {
  return std::list<T1, T2>();
}

template <typename T1, typename T2>
std::list<T1, T2> f2(const typename std::list<T1, T2>::allocator_type &a0) {
  return std::list<T1, T2>(a0);
}

template <typename T1, typename T2>
std::list<T1, T2> f3(typename std::list<T1, T2>::size_type a0,
                     const typename std::list<T1, T2>::allocator_type &a1) {
  return std::list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::list<T1, T2> f4(typename std::list<T1, T2>::size_type a0,
                     const typename std::list<T1, T2>::value_type &a1,
                     const typename std::list<T1, T2>::allocator_type &a2) {
  return std::list<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
std::list<T1, T2> f5(const std::list<T1, T2> &a0) {
  return std::list<T1, T2>(a0);
}

template <typename T1, typename T2>
std::list<T1, T2> f6(std::list<T1, T2> &&a0) {
  return std::list<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::list<T1, T2>
f7(std::initializer_list<typename std::list<T1, T2>::value_type> a0,
   const typename std::list<T1, T2>::allocator_type &a1) {
  return std::list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::list<T1, T2> f8(const std::list<T1, T2> &a0,
                     const typename std::list<T1, T2>::allocator_type &a1) {
  return std::list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::list<T1, T2> f9(std::list<T1, T2> &&a0,
                     const typename std::list<T1, T2>::allocator_type &a1) {
  return std::list<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::list<T1, T2> &f10(std::list<T1, T2> &a0, const std::list<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::list<T1, T2> &f11(std::list<T1, T2> &a0, std::list<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::list<T1, T2> &
f12(std::list<T1, T2> &a0,
    std::initializer_list<typename std::list<T1, T2>::value_type> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
void f13(std::list<T1, T2> &a0, typename std::list<T1, T2>::size_type a1,
         const typename std::list<T1, T2>::value_type &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
void f14(std::list<T1, T2> &a0,
         std::initializer_list<typename std::list<T1, T2>::value_type> a1) {
  return a0.assign(a1);
}

template <typename T1, typename T2>
typename std::list<T1, T2>::allocator_type f15(const std::list<T1, T2> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator f16(std::list<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_iterator f17(const std::list<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator f18(std::list<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_iterator f19(const std::list<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::reverse_iterator f20(std::list<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_reverse_iterator
f21(const std::list<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::reverse_iterator f22(std::list<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_reverse_iterator
f23(const std::list<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_iterator f24(const std::list<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_iterator f25(const std::list<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_reverse_iterator
f26(const std::list<T1, T2> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_reverse_iterator
f27(const std::list<T1, T2> &a0) {
  return a0.crend();
}

template <typename T1, typename T2> bool f28(const std::list<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::size_type f29(const std::list<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::size_type f30(const std::list<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
void f31(std::list<T1, T2> &a0, typename std::list<T1, T2>::size_type a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f32(std::list<T1, T2> &a0, typename std::list<T1, T2>::size_type a1,
         const typename std::list<T1, T2>::value_type &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2>
typename std::list<T1, T2>::reference f33(std::list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_reference f34(const std::list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::reference f35(std::list<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_reference f36(const std::list<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
void f37(std::list<T1, T2> &a0,
         const typename std::list<T1, T2>::value_type &a1) {
  return a0.push_front(a1);
}

template <typename T1, typename T2>
void f38(std::list<T1, T2> &a0, typename std::list<T1, T2>::value_type &&a1) {
  return a0.push_front(std::move(a1));
}

template <typename T1, typename T2> void f39(std::list<T1, T2> &a0) {
  return a0.pop_front();
}

template <typename T1, typename T2>
void f40(std::list<T1, T2> &a0,
         const typename std::list<T1, T2>::value_type &a1) {
  return a0.push_back(a1);
}

template <typename T1, typename T2>
void f41(std::list<T1, T2> &a0, typename std::list<T1, T2>::value_type &&a1) {
  return a0.push_back(std::move(a1));
}

template <typename T1, typename T2> void f42(std::list<T1, T2> &a0) {
  return a0.pop_back();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator
f43(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
    const typename std::list<T1, T2>::value_type &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator
f44(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
    typename std::list<T1, T2>::value_type &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator
f45(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
    std::initializer_list<typename std::list<T1, T2>::value_type> a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator
f46(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
    typename std::list<T1, T2>::size_type a2,
    const typename std::list<T1, T2>::value_type &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator
f47(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator
f48(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
    typename std::list<T1, T2>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2>
void f49(std::list<T1, T2> &a0, std::list<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2> void f50(std::list<T1, T2> &a0) {
  return a0.clear();
}

template <typename T1, typename T2>
void f51(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
         std::list<T1, T2> &&a2) {
  return a0.splice(a1, std::move(a2));
}

template <typename T1, typename T2>
void f52(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
         std::list<T1, T2> &a2) {
  return a0.splice(a1, a2);
}

template <typename T1, typename T2>
void f53(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
         std::list<T1, T2> &&a2,
         typename std::list<T1, T2>::const_iterator a3) {
  return a0.splice(a1, std::move(a2), a3);
}

template <typename T1, typename T2>
void f54(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
         std::list<T1, T2> &a2, typename std::list<T1, T2>::const_iterator a3) {
  return a0.splice(a1, a2, a3);
}

template <typename T1, typename T2>
void f55(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
         std::list<T1, T2> &&a2, typename std::list<T1, T2>::const_iterator a3,
         typename std::list<T1, T2>::const_iterator a4) {
  return a0.splice(a1, std::move(a2), a3, a4);
}

template <typename T1, typename T2>
void f56(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
         std::list<T1, T2> &a2, typename std::list<T1, T2>::const_iterator a3,
         typename std::list<T1, T2>::const_iterator a4) {
  return a0.splice(a1, a2, a3, a4);
}

template <typename T1, typename T2>
void f57(std::list<T1, T2> &a0, const T1 &a1) {
  return a0.remove(a1);
}

template <typename T1, typename T2> void f58(std::list<T1, T2> &a0) {
  return a0.unique();
}

template <typename T1, typename T2>
void f59(std::list<T1, T2> &a0, std::list<T1, T2> &&a1) {
  return a0.merge(std::move(a1));
}

template <typename T1, typename T2>
void f60(std::list<T1, T2> &a0, std::list<T1, T2> &a1) {
  return a0.merge(a1);
}

template <typename T1, typename T2> void f61(std::list<T1, T2> &a0) {
  return a0.reverse();
}

template <typename T1, typename T2> void f62(std::list<T1, T2> &a0) {
  return a0.sort();
}

template <typename T1, typename T2, typename T3>
std::list<T1, T2> f63(T3 a0, T3 a1,
                      const typename std::list<T1, T2>::allocator_type &a2) {
  return std::list<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
void f64(std::list<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(a1, a2);
}

template <typename T1, typename T2, typename... T3>
void f65(std::list<T1, T2> &a0, T3 &&...a1) {
  return a0.template emplace_front<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename... T3>
void f66(std::list<T1, T2> &a0, T3 &&...a1) {
  return a0.template emplace_back<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename... T3>
typename std::list<T1, T2>::iterator
f67(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
    T3 &&...a2) {
  return a0.template emplace<T3...>(a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2, typename T3>
typename std::list<T1, T2>::iterator
f68(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1, T3 a2,
    T3 a3) {
  return a0.template insert<T3>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
void f69(std::list<T1, T2> &a0, T3 a1) {
  return a0.template remove_if<T3>(a1);
}

template <typename T1, typename T2, typename T3>
void f70(std::list<T1, T2> &a0, T3 a1) {
  return a0.template unique<T3>(a1);
}

template <typename T1, typename T2, typename T3>
void f71(std::list<T1, T2> &a0, std::list<T1, T2> &&a1, T3 a2) {
  return a0.template merge<T3>(std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
void f72(std::list<T1, T2> &a0, std::list<T1, T2> &a1, T3 a2) {
  return a0.template merge<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3>
void f73(std::list<T1, T2> &a0, T3 a1) {
  return a0.template sort<T3>(a1);
}

template <typename T1, typename T2>
using t10 = typename std::list<T1, T2>::const_reverse_iterator::pointer;

template <typename T1, typename T2>
using t11 = typename std::list<T1, T2>::const_reverse_iterator::difference_type;

template <typename T1, typename T2>
using t12 = typename std::list<T1, T2>::const_reverse_iterator::reference;

template <typename T1, typename T2>
using t13 = typename std::list<T1, T2>::reverse_iterator::pointer;

template <typename T1, typename T2>
using t14 = typename std::list<T1, T2>::reverse_iterator::difference_type;

template <typename T1, typename T2>
using t15 = typename std::list<T1, T2>::reverse_iterator::reference;
