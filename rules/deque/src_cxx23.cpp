#include <algorithm>
#include <deque>
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
std::deque<T1, T2> f3(std::size_t a0, const T2 &a1) {
  return std::deque<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::deque<T1, T2> f4(std::size_t a0, const T1 &a1, const T2 &a2) {
  return std::deque<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
std::deque<T1, T2> f5(const std::deque<T1, T2> &a0) {
  return std::deque<T1, T2>(a0);
}

template <typename T1, typename T2>
std::deque<T1, T2> f6(std::deque<T1, T2> &&a0) {
  return std::deque<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::deque<T1, T2> f7(const std::deque<T1, T2> &a0, const T2 &a1) {
  return std::deque<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::deque<T1, T2> f8(std::deque<T1, T2> &&a0, const T2 &a1) {
  return std::deque<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::deque<T1, T2> f9(std::initializer_list<T1> a0, const T2 &a1) {
  return std::deque<T1, T2>(std::forward<std::initializer_list<T1>>(a0), a1);
}

template <typename T1, typename T2>
std::deque<T1, T2> &f10(std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::deque<T1, T2> &f11(std::deque<T1, T2> &a0, std::deque<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::deque<T1, T2> &f12(std::deque<T1, T2> &a0, std::initializer_list<T1> a1) {
  return a0.operator=(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2>
void f13(std::deque<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
void f14(std::deque<T1, T2> &a0, std::initializer_list<T1> a1) {
  return a0.assign(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2> T2 f15(const std::deque<T1, T2> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::iterator f16(std::deque<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_iterator f17(const std::deque<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::iterator f18(std::deque<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_iterator f19(const std::deque<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::reverse_iterator f20(std::deque<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_reverse_iterator
f21(const std::deque<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::reverse_iterator f22(std::deque<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_reverse_iterator
f23(const std::deque<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_iterator f24(const std::deque<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_iterator f25(const std::deque<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_reverse_iterator
f26(const std::deque<T1, T2> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_reverse_iterator
f27(const std::deque<T1, T2> &a0) {
  return a0.crend();
}

template <typename T1, typename T2>
std::size_t f28(const std::deque<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
std::size_t f29(const std::deque<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
void f30(std::deque<T1, T2> &a0, std::size_t a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f31(std::deque<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2> void f32(std::deque<T1, T2> &a0) {
  return a0.shrink_to_fit();
}

template <typename T1, typename T2> bool f33(const std::deque<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::reference f34(std::deque<T1, T2> &a0,
                                           std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_reference f35(const std::deque<T1, T2> &a0,
                                                 std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::reference f36(std::deque<T1, T2> &a0,
                                           std::size_t a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_reference f37(const std::deque<T1, T2> &a0,
                                                 std::size_t a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::reference f38(std::deque<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_reference f39(const std::deque<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::reference f40(std::deque<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::const_reference f41(const std::deque<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
void f42(std::deque<T1, T2> &a0, const T1 &a1) {
  return a0.push_front(a1);
}

template <typename T1, typename T2> void f43(std::deque<T1, T2> &a0, T1 &&a1) {
  return a0.push_front(std::move(a1));
}

template <typename T1, typename T2>
void f44(std::deque<T1, T2> &a0, const T1 &a1) {
  return a0.push_back(a1);
}

template <typename T1, typename T2> void f45(std::deque<T1, T2> &a0, T1 &&a1) {
  return a0.push_back(std::move(a1));
}

template <typename T1, typename T2> void f46(std::deque<T1, T2> &a0) {
  return a0.pop_front();
}

template <typename T1, typename T2> void f47(std::deque<T1, T2> &a0) {
  return a0.pop_back();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::iterator
f48(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
    const T1 &a2) {
  return a0.insert(
      std::forward<typename std::deque<T1, T2>::const_iterator>(a1), a2);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::iterator
f49(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
    T1 &&a2) {
  return a0.insert(
      std::forward<typename std::deque<T1, T2>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::iterator
f50(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
    std::initializer_list<T1> a2) {
  return a0.insert(
      std::forward<typename std::deque<T1, T2>::const_iterator>(a1),
      std::forward<std::initializer_list<T1>>(a2));
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::iterator
f51(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
    std::size_t a2, const T1 &a3) {
  return a0.insert(
      std::forward<typename std::deque<T1, T2>::const_iterator>(a1), a2, a3);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::iterator
f52(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1) {
  return a0.erase(
      std::forward<typename std::deque<T1, T2>::const_iterator>(a1));
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::iterator
f53(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
    typename std::deque<T1, T2>::const_iterator a2) {
  return a0.erase(
      std::forward<typename std::deque<T1, T2>::const_iterator>(a1),
      std::forward<typename std::deque<T1, T2>::const_iterator>(a2));
}

template <typename T1, typename T2>
void f54(std::deque<T1, T2> &a0, std::deque<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2> void f55(std::deque<T1, T2> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T3>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
std::deque<T1, T2> f56(T3 a0, T3 a1, const T2 &a2) {
  return std::deque<T1, T2>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
std::deque<T1, T2> f57(std::from_range_t a0, T3 &&a1, const T2 &a2) {
  return std::deque<T1, T2>(std::move(a0), std::forward<T3>(a1), a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T3>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
void f58(std::deque<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f59(std::deque<T1, T2> &a0, T3 &&a1) {
  return a0.template assign_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename... T3>
typename std::deque<T1, T2>::reference f60(std::deque<T1, T2> &a0, T3 &&...a1) {
  return a0.template emplace_front<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename... T3>
typename std::deque<T1, T2>::reference f61(std::deque<T1, T2> &a0, T3 &&...a1) {
  return a0.template emplace_back<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename... T3>
typename std::deque<T1, T2>::iterator
f62(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
    T3 &&...a2) {
  return a0.template emplace<T3...>(
      std::forward<typename std::deque<T1, T2>::const_iterator>(a1),
      std::forward<T3>(a2)...);
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T3>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
typename std::deque<T1, T2>::iterator
f63(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
    T3 a2, T3 a3) {
  return a0.template insert<T3>(
      std::forward<typename std::deque<T1, T2>::const_iterator>(a1),
      std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::deque<T1, T2>::iterator
f64(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
    T3 &&a2) {
  return a0.template insert_range<T3>(
      std::forward<typename std::deque<T1, T2>::const_iterator>(a1),
      std::forward<T3>(a2));
}

template <typename T1, typename T2, typename T3>
void f65(std::deque<T1, T2> &a0, T3 &&a1) {
  return a0.template prepend_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
void f66(std::deque<T1, T2> &a0, T3 &&a1) {
  return a0.template append_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2>
using t14 = typename std::deque<T1, T2>::iterator::iterator;

template <typename T1, typename T2>
using t15 = typename std::deque<T1, T2>::iterator::const_iterator;

template <typename T1, typename T2>
using t16 = typename std::deque<T1, T2>::iterator::iterator_category;

template <typename T1, typename T2>
using t17 = typename std::deque<T1, T2>::iterator::value_type;

template <typename T1, typename T2>
using t18 = typename std::deque<T1, T2>::iterator::pointer;

template <typename T1, typename T2>
using t19 = typename std::deque<T1, T2>::iterator::reference;

template <typename T1, typename T2>
using t20 = typename std::deque<T1, T2>::iterator::size_type;

template <typename T1, typename T2>
using t21 = typename std::deque<T1, T2>::iterator::difference_type;

template <typename T1, typename T2>
using t22 = typename std::deque<T1, T2>::const_iterator::iterator;

template <typename T1, typename T2>
using t23 = typename std::deque<T1, T2>::const_iterator::const_iterator;

template <typename T1, typename T2>
using t24 = typename std::deque<T1, T2>::const_iterator::iterator_category;

template <typename T1, typename T2>
using t25 = typename std::deque<T1, T2>::const_iterator::value_type;

template <typename T1, typename T2>
using t26 = typename std::deque<T1, T2>::const_iterator::pointer;

template <typename T1, typename T2>
using t27 = typename std::deque<T1, T2>::const_iterator::reference;

template <typename T1, typename T2>
using t28 = typename std::deque<T1, T2>::const_iterator::size_type;

template <typename T1, typename T2>
using t29 = typename std::deque<T1, T2>::const_iterator::difference_type;

template <typename T1, typename T2, typename T3>
typename std::deque<T1, T2>::size_type f67(std::deque<T1, T2> &a0,
                                           const T3 &a1) {
  return std::erase(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::deque<T1, T2>::size_type f68(std::deque<T1, T2> &a0, T3 a1) {
  return std::erase_if(a0, std::move(a1));
}

template <typename T1> using t30 = std::pmr::deque<T1>;
