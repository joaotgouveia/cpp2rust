#include <algorithm>
#include <forward_list>
#include <utility>

template <typename T1, typename T2, typename T3>
typename std::forward_list<T1, T2>::size_type f1(std::forward_list<T1, T2> &a0,
                                                 const T3 &a1) {
  return std::erase(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::forward_list<T1, T2>::size_type f2(std::forward_list<T1, T2> &a0,
                                                 T3 a1) {
  return std::erase_if(a0, std::move(a1));
}

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

template <typename T1, typename T2> std::forward_list<T1, T2> f3() {
  return std::forward_list<T1, T2>();
}

template <typename T1, typename T2> std::forward_list<T1, T2> f4(const T2 &a0) {
  return std::forward_list<T1, T2>(a0);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f5(const std::forward_list<T1, T2> &a0,
                             const T2 &a1) {
  return std::forward_list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f6(std::forward_list<T1, T2> &&a0, const T2 &a1) {
  return std::forward_list<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f7(std::size_t a0, const T2 &a1) {
  return std::forward_list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f8(std::size_t a0, const T1 &a1, const T2 &a2) {
  return std::forward_list<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f9(const std::forward_list<T1, T2> &a0) {
  return std::forward_list<T1, T2>(a0);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f10(std::forward_list<T1, T2> &&a0) {
  return std::forward_list<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f11(std::initializer_list<T1> a0, const T2 &a1) {
  return std::forward_list<T1, T2>(std::forward<std::initializer_list<T1>>(a0),
                                   a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> &f12(std::forward_list<T1, T2> &a0,
                               const std::forward_list<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> &f13(std::forward_list<T1, T2> &a0,
                               std::forward_list<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::forward_list<T1, T2> &f14(std::forward_list<T1, T2> &a0,
                               std::initializer_list<T1> a1) {
  return a0.operator=(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2>
void f15(std::forward_list<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
void f16(std::forward_list<T1, T2> &a0, std::initializer_list<T1> a1) {
  return a0.assign(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2>
T2 f17(const std::forward_list<T1, T2> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f18(std::forward_list<T1, T2> &a0) {
  return a0.before_begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f19(const std::forward_list<T1, T2> &a0) {
  return a0.before_begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f20(std::forward_list<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f21(const std::forward_list<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f22(std::forward_list<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f23(const std::forward_list<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f24(const std::forward_list<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f25(const std::forward_list<T1, T2> &a0) {
  return a0.cbefore_begin();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::const_iterator
f26(const std::forward_list<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
bool f27(const std::forward_list<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
std::size_t f28(const std::forward_list<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2> T1 &f29(std::forward_list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
const T1 &f30(const std::forward_list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
void f31(std::forward_list<T1, T2> &a0, const T1 &a1) {
  return a0.push_front(a1);
}

template <typename T1, typename T2>
void f32(std::forward_list<T1, T2> &a0, T1 &&a1) {
  return a0.push_front(std::move(a1));
}

template <typename T1, typename T2> void f33(std::forward_list<T1, T2> &a0) {
  return a0.pop_front();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f34(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, const T1 &a2) {
  return a0.insert_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1), a2);
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f35(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, T1 &&a2) {
  return a0.insert_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f36(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, std::size_t a2,
    const T1 &a3) {
  return a0.insert_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1), a2,
      a3);
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f37(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1,
    std::initializer_list<T1> a2) {
  return a0.insert_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1),
      std::forward<std::initializer_list<T1>>(a2));
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f38(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1) {
  return a0.erase_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1));
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f39(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1,
    typename std::forward_list<T1, T2>::const_iterator a2) {
  return a0.erase_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1),
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a2));
}

template <typename T1, typename T2>
void f40(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2>
void f41(std::forward_list<T1, T2> &a0, std::size_t a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f42(std::forward_list<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2> void f43(std::forward_list<T1, T2> &a0) {
  return a0.clear();
}

template <typename T1, typename T2>
void f44(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &&a2) {
  return a0.splice_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1, typename T2>
void f45(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &a2) {
  return a0.splice_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1), a2);
}

template <typename T1, typename T2>
void f46(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &&a2,
         typename std::forward_list<T1, T2>::const_iterator a3) {
  return a0.splice_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1),
      std::move(a2),
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a3));
}

template <typename T1, typename T2>
void f47(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &a2,
         typename std::forward_list<T1, T2>::const_iterator a3) {
  return a0.splice_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1), a2,
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a3));
}

template <typename T1, typename T2>
void f48(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &&a2,
         typename std::forward_list<T1, T2>::const_iterator a3,
         typename std::forward_list<T1, T2>::const_iterator a4) {
  return a0.splice_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1),
      std::move(a2),
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a3),
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a4));
}

template <typename T1, typename T2>
void f49(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::const_iterator a1,
         std::forward_list<T1, T2> &a2,
         typename std::forward_list<T1, T2>::const_iterator a3,
         typename std::forward_list<T1, T2>::const_iterator a4) {
  return a0.splice_after(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1), a2,
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a3),
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a4));
}

template <typename T1, typename T2>
std::size_t f50(std::forward_list<T1, T2> &a0, const T1 &a1) {
  return a0.remove(a1);
}

template <typename T1, typename T2>
std::size_t f51(std::forward_list<T1, T2> &a0) {
  return a0.unique();
}

template <typename T1, typename T2>
void f52(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &&a1) {
  return a0.merge(std::move(a1));
}

template <typename T1, typename T2>
void f53(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &a1) {
  return a0.merge(a1);
}

template <typename T1, typename T2> void f54(std::forward_list<T1, T2> &a0) {
  return a0.sort();
}

template <typename T1, typename T2> void f55(std::forward_list<T1, T2> &a0) {
  return a0.reverse();
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T3>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
std::forward_list<T1, T2> f56(T3 a0, T3 a1, const T2 &a2) {
  return std::forward_list<T1, T2>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
std::forward_list<T1, T2> f57(std::from_range_t a0, T3 &&a1, const T2 &a2) {
  return std::forward_list<T1, T2>(std::move(a0), std::forward<T3>(a1), a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T3>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
void f58(std::forward_list<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f59(std::forward_list<T1, T2> &a0, T3 &&a1) {
  return a0.template assign_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename... T3>
T1 &f60(std::forward_list<T1, T2> &a0, T3 &&...a1) {
  return a0.template emplace_front<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename T3>
void f61(std::forward_list<T1, T2> &a0, T3 &&a1) {
  return a0.template prepend_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename... T3>
typename std::forward_list<T1, T2>::iterator
f62(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, T3 &&...a2) {
  return a0.template emplace_after<T3...>(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1),
      std::forward<T3>(a2)...);
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T3>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
typename std::forward_list<T1, T2>::iterator
f63(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, T3 a2, T3 a3) {
  return a0.template insert_after<T3>(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1),
      std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::forward_list<T1, T2>::iterator
f64(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, T3 &&a2) {
  return a0.template insert_range_after<T3>(
      std::forward<typename std::forward_list<T1, T2>::const_iterator>(a1),
      std::forward<T3>(a2));
}

template <typename T1, typename T2, typename T3>
std::size_t f65(std::forward_list<T1, T2> &a0, T3 a1) {
  return a0.template remove_if<T3>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::size_t f66(std::forward_list<T1, T2> &a0, T3 a1) {
  return a0.template unique<T3>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
void f67(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &&a1, T3 a2) {
  return a0.template merge<T3>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f68(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &a1, T3 a2) {
  return a0.template merge<T3>(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f69(std::forward_list<T1, T2> &a0, T3 a1) {
  return a0.template sort<T3>(std::move(a1));
}

template <typename T1, typename T2>
using t12 = typename std::forward_list<T1, T2>::iterator::value_type;

template <typename T1, typename T2>
using t13 = typename std::forward_list<T1, T2>::iterator::difference_type;

template <typename T1, typename T2>
using t14 = typename std::forward_list<T1, T2>::iterator::iterator_category;

template <typename T1, typename T2>
using t15 = typename std::forward_list<T1, T2>::iterator::pointer;

template <typename T1, typename T2>
using t16 = typename std::forward_list<T1, T2>::iterator::reference;

template <typename T1, typename T2>
using t17 = typename std::forward_list<T1, T2>::const_iterator::value_type;

template <typename T1, typename T2>
using t18 = typename std::forward_list<T1, T2>::const_iterator::difference_type;

template <typename T1, typename T2>
using t19 =
    typename std::forward_list<T1, T2>::const_iterator::iterator_category;

template <typename T1, typename T2>
using t20 = typename std::forward_list<T1, T2>::const_iterator::pointer;

template <typename T1, typename T2>
using t21 = typename std::forward_list<T1, T2>::const_iterator::reference;

template <typename T1, typename T2>
auto f70(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return std::operator<=>(a0, a1);
}

template <typename T1, typename T2>
bool f71(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return std::operator==(a0, a1);
}

template <typename T1> using t22 = std::pmr::forward_list<T1>;

template <typename T1, typename T2>
void f72(std::forward_list<T1, T2> &a0, std::forward_list<T1, T2> &a1) {
  return std::swap(a0, a1);
}
