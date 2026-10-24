#include <algorithm>
#include <deque>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2> using t1 = std::deque<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::deque<T1, T2>::value_type;

template <typename T1, typename T2>
using t3 = typename std::deque<T1, T2>::pointer;

template <typename T1, typename T2>
using t4 = typename std::deque<T1, T2>::const_pointer;

template <typename T1, typename T2>
using t5 = typename std::deque<T1, T2>::reference;

template <typename T1, typename T2>
using t6 = typename std::deque<T1, T2>::const_reference;

template <typename T1, typename T2>
using t7 = typename std::deque<T1, T2>::iterator;

template <typename T1, typename T2>
using t8 = typename std::deque<T1, T2>::const_iterator;

template <typename T1, typename T2>
using t9 = typename std::deque<T1, T2>::const_reverse_iterator;

template <typename T1, typename T2>
using t10 = typename std::deque<T1, T2>::reverse_iterator;

template <typename T1, typename T2>
using t11 = typename std::deque<T1, T2>::size_type;

template <typename T1, typename T2>
using t12 = typename std::deque<T1, T2>::difference_type;

template <typename T1, typename T2>
using t13 = typename std::deque<T1, T2>::allocator_type;

template <typename T1, typename T2> std::deque<T1, T2> f1() {
  return std::deque<T1, T2>();
}

template <typename T1, typename T2> std::deque<T1, T2> f2(const T2 &a0) {
  return std::deque<T1, T2>(a0);
}

template <typename T1, typename T2>
std::deque<T1, T2> f3(const std::deque<T1, T2> &a0) {
  return std::deque<T1, T2>(a0);
}

template <typename T1, typename T2>
std::deque<T1, T2> f4(std::deque<T1, T2> &&a0) {
  return std::deque<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::deque<T1, T2> f5(const std::deque<T1, T2> &a0, const T2 &a1) {
  return std::deque<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::deque<T1, T2> f6(std::deque<T1, T2> &&a0, const T2 &a1) {
  return std::deque<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::deque<T1, T2> f7(std::initializer_list<T1> a0, const T2 &a1) {
  return std::deque<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::deque<T1, T2> &f8(std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::deque<T1, T2> &f9(std::deque<T1, T2> &a0, std::deque<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::deque<T1, T2> &f10(std::deque<T1, T2> &a0, std::initializer_list<T1> a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
void f11(std::deque<T1, T2> &a0, std::initializer_list<T1> a1) {
  return a0.assign(std::move(a1));
}

template <typename T1, typename T2> T2 f12(const std::deque<T1, T2> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::iterator f13(std::deque<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_iterator f14(const std::deque<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::iterator f15(std::deque<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_iterator f16(const std::deque<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::reverse_iterator f17(std::deque<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_reverse_iterator
f18(const std::deque<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::reverse_iterator f19(std::deque<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_reverse_iterator
f20(const std::deque<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_iterator f21(const std::deque<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_iterator f22(const std::deque<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_reverse_iterator
f23(const std::deque<T1, T2> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_reverse_iterator
f24(const std::deque<T1, T2> &a0) {
  return a0.crend();
}

template <typename T1, typename T2> void f25(std::deque<T1, T2> &a0) {
  return a0.shrink_to_fit();
}

template <typename T1, typename T2> bool f26(const std::deque<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
void f27(std::deque<T1, T2> &a0, const T1 &a1) {
  return a0.push_front(a1);
}

template <typename T1, typename T2> void f28(std::deque<T1, T2> &a0, T1 &&a1) {
  return a0.push_front(std::move(a1));
}

template <typename T1, typename T2>
void f29(std::deque<T1, T2> &a0, const T1 &a1) {
  return a0.push_back(a1);
}

template <typename T1, typename T2> void f30(std::deque<T1, T2> &a0, T1 &&a1) {
  return a0.push_back(std::move(a1));
}

template <typename T1, typename T2> void f31(std::deque<T1, T2> &a0) {
  return a0.pop_front();
}

template <typename T1, typename T2> void f32(std::deque<T1, T2> &a0) {
  return a0.pop_back();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::iterator
f33(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
    const T1 &a2) {
  return a0.insert(std::move(a1), a2);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::iterator
f34(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
    T1 &&a2) {
  return a0.insert(std::move(a1), std::move(a2));
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::iterator
f35(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
    std::initializer_list<T1> a2) {
  return a0.insert(std::move(a1), std::move(a2));
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::iterator
f36(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1) {
  return a0.erase(std::move(a1));
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::iterator
f37(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
    typename std::deque<T1, T2>::const_iterator a2) {
  return a0.erase(std::move(a1), std::move(a2));
}

template <typename T1, typename T2>
void f38(std::deque<T1, T2> &a0, std::deque<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2> void f39(std::deque<T1, T2> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename... T3>
typename std::deque<T1, T2>::iterator
f40(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
    T3 &&...a2) {
  return a0.template emplace<T3...>(std::move(a1), std::forward<T3>(a2)...);
}

template <typename T1, typename T2>
using t14 = typename std::deque<T1, T2>::iterator::iterator_category;

template <typename T1, typename T2>
using t15 = typename std::deque<T1, T2>::iterator::value_type;

template <typename T1, typename T2>
using t16 = typename std::deque<T1, T2>::iterator::pointer;

template <typename T1, typename T2>
using t17 = typename std::deque<T1, T2>::iterator::reference;

template <typename T1, typename T2>
using t18 = typename std::deque<T1, T2>::iterator::difference_type;

template <typename T1, typename T2>
using t19 = typename std::deque<T1, T2>::const_iterator::iterator_category;

template <typename T1, typename T2>
using t20 = typename std::deque<T1, T2>::const_iterator::value_type;

template <typename T1, typename T2>
using t21 = typename std::deque<T1, T2>::const_iterator::pointer;

template <typename T1, typename T2>
using t22 = typename std::deque<T1, T2>::const_iterator::reference;

template <typename T1, typename T2>
using t23 = typename std::deque<T1, T2>::const_iterator::difference_type;
