#include <algorithm>
#include <list>
#include <utility>

template <typename T1, typename T2, typename T3>
typename std::list<T1, T2>::size_type f1(std::list<T1, T2> &a0, const T3 &a1) {
  return std::erase<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::list<T1, T2>::size_type f2(std::list<T1, T2> &a0, T3 a1) {
  return std::erase_if<T1, T2, T3>(a0, std::move(a1));
}

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
using t7 = typename std::list<T1, T2>::iterator;

template <typename T1, typename T2>
using t8 = typename std::list<T1, T2>::const_iterator;

template <typename T1, typename T2>
using t9 = typename std::list<T1, T2>::const_reverse_iterator;

template <typename T1, typename T2>
using t10 = typename std::list<T1, T2>::reverse_iterator;

template <typename T1, typename T2>
using t11 = typename std::list<T1, T2>::size_type;

template <typename T1, typename T2>
using t12 = typename std::list<T1, T2>::difference_type;

template <typename T1, typename T2>
using t13 = typename std::list<T1, T2>::allocator_type;

template <typename T1, typename T2> std::list<T1, T2> f3() {
  return std::list<T1, T2>();
}

template <typename T1, typename T2> std::list<T1, T2> f4(const T2 &a0) {
  return std::list<T1, T2>(a0);
}

template <typename T1, typename T2>
std::list<T1, T2> f5(std::size_t a0, const T2 &a1) {
  return std::list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::list<T1, T2> f6(std::size_t a0, const T1 &a1, const T2 &a2) {
  return std::list<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
std::list<T1, T2> f7(const std::list<T1, T2> &a0) {
  return std::list<T1, T2>(a0);
}

template <typename T1, typename T2>
std::list<T1, T2> f8(std::list<T1, T2> &&a0) {
  return std::list<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::list<T1, T2> f9(std::initializer_list<T1> a0, const T2 &a1) {
  return std::list<T1, T2>(std::forward<std::initializer_list<T1>>(a0), a1);
}

template <typename T1, typename T2>
std::list<T1, T2> f10(const std::list<T1, T2> &a0, const T2 &a1) {
  return std::list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::list<T1, T2> f11(std::list<T1, T2> &&a0, const T2 &a1) {
  return std::list<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::list<T1, T2> &f12(std::list<T1, T2> &a0, const std::list<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::list<T1, T2> &f13(std::list<T1, T2> &a0, std::list<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::list<T1, T2> &f14(std::list<T1, T2> &a0, std::initializer_list<T1> a1) {
  return a0.operator=(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2>
void f15(std::list<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
void f16(std::list<T1, T2> &a0, std::initializer_list<T1> a1) {
  return a0.assign(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2> T2 f17(const std::list<T1, T2> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator f18(std::list<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_iterator f19(const std::list<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator f20(std::list<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_iterator f21(const std::list<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::reverse_iterator f22(std::list<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_reverse_iterator
f23(const std::list<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::reverse_iterator f24(std::list<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_reverse_iterator
f25(const std::list<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_iterator f26(const std::list<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_iterator f27(const std::list<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_reverse_iterator
f28(const std::list<T1, T2> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_reverse_iterator
f29(const std::list<T1, T2> &a0) {
  return a0.crend();
}

template <typename T1, typename T2> bool f30(const std::list<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
std::size_t f31(const std::list<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
std::size_t f32(const std::list<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
void f33(std::list<T1, T2> &a0, std::size_t a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f34(std::list<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2>
typename std::list<T1, T2>::reference f35(std::list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_reference f36(const std::list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::reference f37(std::list<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_reference f38(const std::list<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
void f39(std::list<T1, T2> &a0, const T1 &a1) {
  return a0.push_front(a1);
}

template <typename T1, typename T2> void f40(std::list<T1, T2> &a0, T1 &&a1) {
  return a0.push_front(std::move(a1));
}

template <typename T1, typename T2> void f41(std::list<T1, T2> &a0) {
  return a0.pop_front();
}

template <typename T1, typename T2>
void f42(std::list<T1, T2> &a0, const T1 &a1) {
  return a0.push_back(a1);
}

template <typename T1, typename T2> void f43(std::list<T1, T2> &a0, T1 &&a1) {
  return a0.push_back(std::move(a1));
}

template <typename T1, typename T2> void f44(std::list<T1, T2> &a0) {
  return a0.pop_back();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator
f45(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
    const T1 &a2) {
  return a0.insert(std::forward<typename std::list<T1, T2>::const_iterator>(a1),
                   a2);
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator
f46(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
    T1 &&a2) {
  return a0.insert(std::forward<typename std::list<T1, T2>::const_iterator>(a1),
                   std::move(a2));
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator
f47(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
    std::initializer_list<T1> a2) {
  return a0.insert(std::forward<typename std::list<T1, T2>::const_iterator>(a1),
                   std::forward<std::initializer_list<T1>>(a2));
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator
f48(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
    std::size_t a2, const T1 &a3) {
  return a0.insert(std::forward<typename std::list<T1, T2>::const_iterator>(a1),
                   a2, a3);
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator
f49(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1) {
  return a0.erase(std::forward<typename std::list<T1, T2>::const_iterator>(a1));
}

template <typename T1, typename T2>
typename std::list<T1, T2>::iterator
f50(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
    typename std::list<T1, T2>::const_iterator a2) {
  return a0.erase(std::forward<typename std::list<T1, T2>::const_iterator>(a1),
                  std::forward<typename std::list<T1, T2>::const_iterator>(a2));
}

template <typename T1, typename T2>
void f51(std::list<T1, T2> &a0, std::list<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2> void f52(std::list<T1, T2> &a0) {
  return a0.clear();
}

template <typename T1, typename T2>
void f53(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
         std::list<T1, T2> &&a2) {
  return a0.splice(std::forward<typename std::list<T1, T2>::const_iterator>(a1),
                   std::move(a2));
}

template <typename T1, typename T2>
void f54(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
         std::list<T1, T2> &a2) {
  return a0.splice(std::forward<typename std::list<T1, T2>::const_iterator>(a1),
                   a2);
}

template <typename T1, typename T2>
void f55(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
         std::list<T1, T2> &&a2,
         typename std::list<T1, T2>::const_iterator a3) {
  return a0.splice(
      std::forward<typename std::list<T1, T2>::const_iterator>(a1),
      std::move(a2),
      std::forward<typename std::list<T1, T2>::const_iterator>(a3));
}

template <typename T1, typename T2>
void f56(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
         std::list<T1, T2> &a2, typename std::list<T1, T2>::const_iterator a3) {
  return a0.splice(
      std::forward<typename std::list<T1, T2>::const_iterator>(a1), a2,
      std::forward<typename std::list<T1, T2>::const_iterator>(a3));
}

template <typename T1, typename T2>
void f57(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
         std::list<T1, T2> &&a2, typename std::list<T1, T2>::const_iterator a3,
         typename std::list<T1, T2>::const_iterator a4) {
  return a0.splice(
      std::forward<typename std::list<T1, T2>::const_iterator>(a1),
      std::move(a2),
      std::forward<typename std::list<T1, T2>::const_iterator>(a3),
      std::forward<typename std::list<T1, T2>::const_iterator>(a4));
}

template <typename T1, typename T2>
void f58(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
         std::list<T1, T2> &a2, typename std::list<T1, T2>::const_iterator a3,
         typename std::list<T1, T2>::const_iterator a4) {
  return a0.splice(
      std::forward<typename std::list<T1, T2>::const_iterator>(a1), a2,
      std::forward<typename std::list<T1, T2>::const_iterator>(a3),
      std::forward<typename std::list<T1, T2>::const_iterator>(a4));
}

template <typename T1, typename T2>
std::size_t f59(std::list<T1, T2> &a0, const T1 &a1) {
  return a0.remove(a1);
}

template <typename T1, typename T2> std::size_t f60(std::list<T1, T2> &a0) {
  return a0.unique();
}

template <typename T1, typename T2>
void f61(std::list<T1, T2> &a0, std::list<T1, T2> &&a1) {
  return a0.merge(std::move(a1));
}

template <typename T1, typename T2>
void f62(std::list<T1, T2> &a0, std::list<T1, T2> &a1) {
  return a0.merge(a1);
}

template <typename T1, typename T2> void f63(std::list<T1, T2> &a0) {
  return a0.reverse();
}

template <typename T1, typename T2> void f64(std::list<T1, T2> &a0) {
  return a0.sort();
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T3>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
std::list<T1, T2> f65(T3 a0, T3 a1, const T2 &a2) {
  return std::list<T1, T2>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
std::list<T1, T2> f66(std::from_range_t a0, T3 &&a1, const T2 &a2) {
  return std::list<T1, T2>(std::move(a0), std::forward<T3>(a1), a2);
}

template <typename T1, typename T2, typename T3>
void f67(std::list<T1, T2> &a0, T3 &&a1) {
  return a0.template assign_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T3>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
void f68(std::list<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename... T3>
typename std::list<T1, T2>::reference f69(std::list<T1, T2> &a0, T3 &&...a1) {
  return a0.template emplace_front<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename T3>
void f70(std::list<T1, T2> &a0, T3 &&a1) {
  return a0.template prepend_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
void f71(std::list<T1, T2> &a0, T3 &&a1) {
  return a0.template append_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename... T3>
typename std::list<T1, T2>::reference f72(std::list<T1, T2> &a0, T3 &&...a1) {
  return a0.template emplace_back<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename... T3>
typename std::list<T1, T2>::iterator
f73(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
    T3 &&...a2) {
  return a0.template emplace<T3...>(
      std::forward<typename std::list<T1, T2>::const_iterator>(a1),
      std::forward<T3>(a2)...);
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T3>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
typename std::list<T1, T2>::iterator
f74(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1, T3 a2,
    T3 a3) {
  return a0.template insert<T3>(
      std::forward<typename std::list<T1, T2>::const_iterator>(a1),
      std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::list<T1, T2>::iterator
f75(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
    T3 &&a2) {
  return a0.template insert_range<T3>(
      std::forward<typename std::list<T1, T2>::const_iterator>(a1),
      std::forward<T3>(a2));
}

template <typename T1, typename T2, typename T3>
std::size_t f76(std::list<T1, T2> &a0, T3 a1) {
  return a0.template remove_if<T3>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::size_t f77(std::list<T1, T2> &a0, T3 a1) {
  return a0.template unique<T3>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
void f78(std::list<T1, T2> &a0, std::list<T1, T2> &&a1, T3 a2) {
  return a0.template merge<T3>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f79(std::list<T1, T2> &a0, std::list<T1, T2> &a1, T3 a2) {
  return a0.template merge<T3>(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f80(std::list<T1, T2> &a0, T3 a1) {
  return a0.template sort<T3>(std::move(a1));
}

template <typename T1> using t14 = std::pmr::list<T1>;
