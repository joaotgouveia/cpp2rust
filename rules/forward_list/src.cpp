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

template <typename T1, typename T2>
std::forward_list<T1, T2> f2(const std::forward_list<T1, T2> &a0) {
  return std::forward_list<T1, T2>(a0);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f3(std::forward_list<T1, T2> &&a0) {
  return std::forward_list<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::forward_list<T1, T2> &f4(std::forward_list<T1, T2> &a0,
                              const std::forward_list<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> &f5(std::forward_list<T1, T2> &a0,
                              std::forward_list<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::allocator_type
f6(const std::forward_list<T1, T2> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator f7(std::forward_list<T1, T2> &a0) {
  return a0.before_begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f8(const std::forward_list<T1, T2> &a0) {
  return a0.before_begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator f9(std::forward_list<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f10(const std::forward_list<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f11(std::forward_list<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f12(const std::forward_list<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f13(const std::forward_list<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f14(const std::forward_list<T1, T2> &a0) {
  return a0.cbefore_begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f15(const std::forward_list<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
bool f16(const std::forward_list<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::size_type
f17(const std::forward_list<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::value_type &
f18(std::forward_list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
const typename std::forward_list<T1, T2>::value_type &
f19(const std::forward_list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2> void f20(std::forward_list<T1, T2> &a0) {
  return a0.pop_front();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f21(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1) {
  return a0.erase_after(a1);
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f22(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1,
    typename std::forward_list<T1, T2>::const_iterator a2) {
  return a0.erase_after(a1, a2);
}

template <typename T1, typename T2>
void f23(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2>
void f24(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::size_type a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f25(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::size_type a1,
         const typename std::forward_list<T1, T2>::value_type &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2> void f26(std::forward_list<T1, T2> &a0) {
  return a0.clear();
}

template <typename T1, typename T2>
void f27(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &&a2) {
  return a0.splice_after(a1, std::move(a2));
}

template <typename T1, typename T2>
void f28(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &a2) {
  return a0.splice_after(a1, a2);
}

template <typename T1, typename T2>
void f29(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &&a2,
         typename std::forward_list<T1, T2>::const_iterator a3) {
  return a0.splice_after(a1, std::move(a2), a3);
}

template <typename T1, typename T2>
void f30(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &a2,
         typename std::forward_list<T1, T2>::const_iterator a3) {
  return a0.splice_after(a1, a2, a3);
}

template <typename T1, typename T2>
void f31(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &&a2,
         typename std::forward_list<T1, T2>::const_iterator a3,
         typename std::forward_list<T1, T2>::const_iterator a4) {
  return a0.splice_after(a1, std::move(a2), a3, a4);
}

template <typename T1, typename T2>
void f32(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &a2,
         typename std::forward_list<T1, T2>::const_iterator a3,
         typename std::forward_list<T1, T2>::const_iterator a4) {
  return a0.splice_after(a1, a2, a3, a4);
}

template <typename T1, typename T2>
void f33(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &&a1) {
  return a0.merge(std::move(a1));
}

template <typename T1, typename T2>
void f34(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &a1) {
  return a0.merge(a1);
}

template <typename T1, typename T2> void f35(std::forward_list<T1, T2> &a0) {
  return a0.sort();
}

template <typename T1, typename T2> void f36(std::forward_list<T1, T2> &a0) {
  return a0.reverse();
}

template <typename T1, typename T2, typename... T3>
typename std::forward_list<T1, T2>::iterator
f37(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, T3 &&...a2) {
  return a0.template emplace_after<T3...>(a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2, typename T3>
void f38(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &&a1, T3 a2) {
  return a0.template merge<T3>(std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
void f39(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &a1, T3 a2) {
  return a0.template merge<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3>
void f40(std::forward_list<T1, T2> &a0, T3 a1) {
  return a0.template sort<T3>(a1);
}

template <typename T1, typename T2>
bool f41(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
void f42(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
