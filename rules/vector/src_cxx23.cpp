#include <algorithm>
#include <utility>
#include <vector>

template <typename T1, typename T2, typename T3>
typename std::vector<T1, T2>::size_type f1(std::vector<T1, T2> &a0,
                                           const T3 &a1) {
  return std::erase(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::vector<T1, T2>::size_type f2(std::vector<T1, T2> &a0, T3 a1) {
  return std::erase_if(a0, std::move(a1));
}

template <typename T1> using t1 = std::pmr::vector<T1>;

template <typename T1> using t2 = std::vector<bool, T1>;

template <typename T1> using t3 = typename std::vector<bool, T1>::value_type;

template <typename T1> using t4 = typename std::vector<bool, T1>::size_type;

template <typename T1>
using t5 = typename std::vector<bool, T1>::difference_type;

template <typename T1> using t6 = typename std::vector<bool, T1>::reference;

template <typename T1>
using t7 = typename std::vector<bool, T1>::const_reference;

template <typename T1> using t8 = typename std::vector<bool, T1>::pointer;

template <typename T1> using t9 = typename std::vector<bool, T1>::const_pointer;

template <typename T1> using t10 = typename std::vector<bool, T1>::iterator;

template <typename T1>
using t11 = typename std::vector<bool, T1>::const_iterator;

template <typename T1>
using t12 = typename std::vector<bool, T1>::const_reverse_iterator;

template <typename T1>
using t13 = typename std::vector<bool, T1>::reverse_iterator;

template <typename T1>
using t14 = typename std::vector<bool, T1>::allocator_type;

template <typename T1> T1 f3(const std::vector<bool, T1> &a0) {
  return a0.get_allocator();
}

template <typename T1> std::vector<bool, T1> f4() {
  return std::vector<bool, T1>();
}

template <typename T1> std::vector<bool, T1> f5(const T1 &a0) {
  return std::vector<bool, T1>(a0);
}

template <typename T1> std::vector<bool, T1> f6(std::size_t a0, const T1 &a1) {
  return std::vector<bool, T1>(a0, a1);
}

template <typename T1>
std::vector<bool, T1> f7(std::size_t a0, const bool &a1, const T1 &a2) {
  return std::vector<bool, T1>(a0, a1, a2);
}

template <typename T1>
std::vector<bool, T1> f8(const std::vector<bool, T1> &a0) {
  return std::vector<bool, T1>(a0);
}

template <typename T1> std::vector<bool, T1> f9(std::vector<bool, T1> &&a0) {
  return std::vector<bool, T1>(std::move(a0));
}

template <typename T1>
std::vector<bool, T1> f10(std::vector<bool, T1> &&a0, const T1 &a1) {
  return std::vector<bool, T1>(std::move(a0), a1);
}

template <typename T1>
std::vector<bool, T1> f11(const std::vector<bool, T1> &a0, const T1 &a1) {
  return std::vector<bool, T1>(a0, a1);
}

template <typename T1>
std::vector<bool, T1> f12(std::initializer_list<bool> a0, const T1 &a1) {
  return std::vector<bool, T1>(std::move(a0), a1);
}

template <typename T1>
std::vector<bool, T1> &f13(std::vector<bool, T1> &a0,
                           const std::vector<bool, T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::vector<bool, T1> &f14(std::vector<bool, T1> &a0,
                           std::vector<bool, T1> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::vector<bool, T1> &f15(std::vector<bool, T1> &a0,
                           std::initializer_list<bool> a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
void f16(std::vector<bool, T1> &a0, std::size_t a1, const bool &a2) {
  return a0.assign(a1, a2);
}

template <typename T1>
void f17(std::vector<bool, T1> &a0, std::initializer_list<bool> a1) {
  return a0.assign(std::move(a1));
}

template <typename T1>
typename std::vector<bool, T1>::iterator f18(std::vector<bool, T1> &a0) {
  return a0.begin();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f19(const std::vector<bool, T1> &a0) {
  return a0.begin();
}

template <typename T1>
typename std::vector<bool, T1>::iterator f20(std::vector<bool, T1> &a0) {
  return a0.end();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f21(const std::vector<bool, T1> &a0) {
  return a0.end();
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator
f22(std::vector<bool, T1> &a0) {
  return a0.rbegin();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f23(const std::vector<bool, T1> &a0) {
  return a0.rbegin();
}

template <typename T1>
typename std::vector<bool, T1>::reverse_iterator
f24(std::vector<bool, T1> &a0) {
  return a0.rend();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f25(const std::vector<bool, T1> &a0) {
  return a0.rend();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f26(const std::vector<bool, T1> &a0) {
  return a0.cbegin();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f27(const std::vector<bool, T1> &a0) {
  return a0.cend();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f28(const std::vector<bool, T1> &a0) {
  return a0.crbegin();
}

template <typename T1>
typename std::vector<bool, T1>::const_reverse_iterator
f29(const std::vector<bool, T1> &a0) {
  return a0.crend();
}

template <typename T1> std::size_t f30(const std::vector<bool, T1> &a0) {
  return a0.size();
}

template <typename T1> std::size_t f31(const std::vector<bool, T1> &a0) {
  return a0.max_size();
}

template <typename T1> std::size_t f32(const std::vector<bool, T1> &a0) {
  return a0.capacity();
}

template <typename T1> bool f33(const std::vector<bool, T1> &a0) {
  return a0.empty();
}

template <typename T1>
typename std::vector<bool, T1>::reference f34(std::vector<bool, T1> &a0,
                                              std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1>
bool f35(const std::vector<bool, T1> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1>
typename std::vector<bool, T1>::reference f36(std::vector<bool, T1> &a0,
                                              std::size_t a1) {
  return a0.at(a1);
}

template <typename T1>
bool f37(const std::vector<bool, T1> &a0, std::size_t a1) {
  return a0.at(a1);
}

template <typename T1> void f38(std::vector<bool, T1> &a0, std::size_t a1) {
  return a0.reserve(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reference f39(std::vector<bool, T1> &a0) {
  return a0.front();
}

template <typename T1> bool f40(const std::vector<bool, T1> &a0) {
  return a0.front();
}

template <typename T1>
typename std::vector<bool, T1>::reference f41(std::vector<bool, T1> &a0) {
  return a0.back();
}

template <typename T1> bool f42(const std::vector<bool, T1> &a0) {
  return a0.back();
}

template <typename T1> void f43(std::vector<bool, T1> &a0, bool a1) {
  return a0.push_back(a1);
}

template <typename T1>
void f44(std::vector<bool, T1> &a0, std::vector<bool, T1> &a1) {
  return a0.swap(a1);
}

template <typename T1>
void f45(typename std::vector<bool, T1>::reference a0,
         typename std::vector<bool, T1>::reference a1) {
  return std::vector<bool, T1>::swap(
      std::forward<typename std::vector<bool, T1>::reference>(a0),
      std::forward<typename std::vector<bool, T1>::reference>(a1));
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f46(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1, const bool &a2) {
  return a0.insert(
      std::forward<typename std::vector<bool, T1>::const_iterator>(a1), a2);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f47(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1) {
  return a0.insert(
      std::forward<typename std::vector<bool, T1>::const_iterator>(a1));
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f48(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1, std::size_t a2,
    const bool &a3) {
  return a0.insert(
      std::forward<typename std::vector<bool, T1>::const_iterator>(a1), a2, a3);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f49(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1,
    std::initializer_list<bool> a2) {
  return a0.insert(
      std::forward<typename std::vector<bool, T1>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1> void f50(std::vector<bool, T1> &a0) {
  return a0.pop_back();
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f51(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1) {
  return a0.erase(
      std::forward<typename std::vector<bool, T1>::const_iterator>(a1));
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f52(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1,
    typename std::vector<bool, T1>::const_iterator a2) {
  return a0.erase(
      std::forward<typename std::vector<bool, T1>::const_iterator>(a1),
      std::forward<typename std::vector<bool, T1>::const_iterator>(a2));
}

template <typename T1>
void f53(std::vector<bool, T1> &a0, std::size_t a1, bool a2) {
  return a0.resize(a1, a2);
}

template <typename T1> void f54(std::vector<bool, T1> &a0) {
  return a0.shrink_to_fit();
}

template <typename T1> void f55(std::vector<bool, T1> &a0) { return a0.flip(); }

template <typename T1> void f56(std::vector<bool, T1> &a0) {
  return a0.clear();
}

template <typename T1, typename T2,
          typename T3 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T2>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
std::vector<bool, T1> f57(T2 a0, T2 a1, const T1 &a2) {
  return std::vector<bool, T1>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2>
std::vector<bool, T1> f58(std::from_range_t a0, T2 &&a1, const T1 &a2) {
  return std::vector<bool, T1>(std::move(a0), std::forward<T2>(a1), a2);
}

template <typename T1, typename T2,
          typename T3 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T2>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
void f59(std::vector<bool, T1> &a0, T2 a1, T2 a2) {
  return a0.template assign<T2>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2>
void f60(std::vector<bool, T1> &a0, T2 &&a1) {
  return a0.template assign_range<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2,
          typename T3 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T2>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
typename std::vector<bool, T1>::iterator
f61(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1, T2 a2, T2 a3) {
  return a0.template insert<T2>(
      std::forward<typename std::vector<bool, T1>::const_iterator>(a1),
      std::move(a2), std::move(a3));
}

template <typename T1, typename T2>
typename std::vector<bool, T1>::iterator
f62(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1, T2 &&a2) {
  return a0.template insert_range<T2>(
      std::forward<typename std::vector<bool, T1>::const_iterator>(a1),
      std::forward<T2>(a2));
}

template <typename T1, typename T2>
void f63(std::vector<bool, T1> &a0, T2 &&a1) {
  return a0.template append_range<T2>(std::forward<T2>(a1));
}

template <typename T1, typename... T2>
typename std::vector<bool, T1>::reference f64(std::vector<bool, T1> &a0,
                                              T2 &&...a1) {
  return a0.template emplace_back<T2...>(std::forward<T2>(a1)...);
}

template <typename T1, typename... T2>
typename std::vector<bool, T1>::iterator
f65(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1, T2 &&...a2) {
  return a0.template emplace<T2...>(
      std::forward<typename std::vector<bool, T1>::const_iterator>(a1),
      std::forward<T2>(a2)...);
}

template <typename T1>
typename std::vector<bool, T1>::reference
f66(const typename std::vector<bool, T1>::reference &a0) {
  return typename std::vector<bool, T1>::reference(a0);
}

template <typename T1>
bool f67(const typename std::vector<bool, T1>::reference &a0) {
  return static_cast<bool>(a0);
}

template <typename T1>
typename std::vector<bool, T1>::reference &
f68(typename std::vector<bool, T1>::reference &a0, bool a1) {
  return a0.operator=(a1);
}

template <typename T1>
const typename std::vector<bool, T1>::reference &
f69(const typename std::vector<bool, T1>::reference &a0, bool a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::reference &
f70(typename std::vector<bool, T1>::reference &a0,
    const typename std::vector<bool, T1>::reference &a1) {
  return a0.operator=(a1);
}

template <typename T1>
bool f71(const typename std::vector<bool, T1>::reference &a0,
         const typename std::vector<bool, T1>::reference &a1) {
  return a0.operator==(a1);
}

template <typename T1>
bool f72(const typename std::vector<bool, T1>::reference &a0,
         const typename std::vector<bool, T1>::reference &a1) {
  return a0.operator<(a1);
}

template <typename T1> void f73(typename std::vector<bool, T1>::reference &a0) {
  return a0.flip();
}

template <typename T1>
void f74(bool &a0, typename std::vector<bool, T1>::reference a1) {
  return swap(a0, std::forward<typename std::vector<bool, T1>::reference>(a1));
}

template <typename T1>
void f75(typename std::vector<bool, T1>::reference a0, bool &a1) {
  return swap(std::forward<typename std::vector<bool, T1>::reference>(a0), a1);
}

template <typename T1>
void f76(typename std::vector<bool, T1>::reference a0,
         typename std::vector<bool, T1>::reference a1) {
  return swap(std::forward<typename std::vector<bool, T1>::reference>(a0),
              std::forward<typename std::vector<bool, T1>::reference>(a1));
}

template <typename T1>
using t15 = typename std::vector<bool, T1>::iterator::reference;

template <typename T1>
using t16 = typename std::vector<bool, T1>::iterator::pointer;

template <typename T1>
using t17 = typename std::vector<bool, T1>::iterator::iterator;

template <typename T1> typename std::vector<bool, T1>::iterator f77() {
  return typename std::vector<bool, T1>::iterator();
}

template <typename T1>
typename std::vector<bool, T1>::iterator f78(unsigned long *a0,
                                             unsigned int a1) {
  return typename std::vector<bool, T1>::iterator(a0, a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f79(typename std::vector<bool, T1>::iterator &a0) {
  return a0.operator++();
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f80(typename std::vector<bool, T1>::iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f81(typename std::vector<bool, T1>::iterator &a0) {
  return a0.operator--();
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f82(typename std::vector<bool, T1>::iterator &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f83(typename std::vector<bool, T1>::iterator &a0, long a1) {
  return a0.operator+=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f84(typename std::vector<bool, T1>::iterator &a0, long a1) {
  return a0.operator-=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f85(const typename std::vector<bool, T1>::iterator &a0) {
  return typename std::vector<bool, T1>::iterator(a0);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f86(typename std::vector<bool, T1>::iterator &&a0) {
  return typename std::vector<bool, T1>::iterator(std::move(a0));
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f87(typename std::vector<bool, T1>::iterator &a0,
    const typename std::vector<bool, T1>::iterator &a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator &
f88(typename std::vector<bool, T1>::iterator &a0,
    typename std::vector<bool, T1>::iterator &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f89(const typename std::vector<bool, T1>::iterator &a0, long a1) {
  return operator-(a0, a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f90(long a0, const typename std::vector<bool, T1>::iterator &a1) {
  return operator+(a0, a1);
}

template <typename T1>
typename std::vector<bool, T1>::iterator
f91(const typename std::vector<bool, T1>::iterator &a0, long a1) {
  return operator+(a0, a1);
}

template <typename T1>
std::ptrdiff_t f92(const typename std::vector<bool, T1>::iterator &a0,
                   const typename std::vector<bool, T1>::iterator &a1) {
  return operator-(a0, a1);
}

template <typename T1>
std::strong_ordering f93(const typename std::vector<bool, T1>::iterator &a0,
                         const typename std::vector<bool, T1>::iterator &a1) {
  return operator<=>(a0, a1);
}

template <typename T1>
bool f94(const typename std::vector<bool, T1>::iterator &a0,
         const typename std::vector<bool, T1>::iterator &a1) {
  return operator==(a0, a1);
}

template <typename T1>
using t18 = typename std::vector<bool, T1>::const_iterator::reference;

template <typename T1>
using t19 = typename std::vector<bool, T1>::const_iterator::const_reference;

template <typename T1>
using t20 = typename std::vector<bool, T1>::const_iterator::pointer;

template <typename T1>
using t21 = typename std::vector<bool, T1>::const_iterator::const_iterator;

template <typename T1> typename std::vector<bool, T1>::const_iterator f95() {
  return typename std::vector<bool, T1>::const_iterator();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator f96(unsigned long *a0,
                                                   unsigned int a1) {
  return typename std::vector<bool, T1>::const_iterator(a0, a1);
}

template <typename T1>
bool f97(const typename std::vector<bool, T1>::const_iterator &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f98(typename std::vector<bool, T1>::const_iterator &a0) {
  return a0.operator++();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f99(typename std::vector<bool, T1>::const_iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f100(typename std::vector<bool, T1>::const_iterator &a0) {
  return a0.operator--();
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f101(typename std::vector<bool, T1>::const_iterator &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f102(typename std::vector<bool, T1>::const_iterator &a0, long a1) {
  return a0.operator+=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f103(typename std::vector<bool, T1>::const_iterator &a0, long a1) {
  return a0.operator-=(a1);
}

template <typename T1>
bool f104(const typename std::vector<bool, T1>::const_iterator &a0, long a1) {
  return a0.operator[](a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f105(const typename std::vector<bool, T1>::const_iterator &a0) {
  return typename std::vector<bool, T1>::const_iterator(a0);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f106(typename std::vector<bool, T1>::const_iterator &&a0) {
  return typename std::vector<bool, T1>::const_iterator(std::move(a0));
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f107(typename std::vector<bool, T1>::const_iterator &a0,
     const typename std::vector<bool, T1>::const_iterator &a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator &
f108(typename std::vector<bool, T1>::const_iterator &a0,
     typename std::vector<bool, T1>::const_iterator &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f109(long a0, const typename std::vector<bool, T1>::const_iterator &a1) {
  return operator+(a0, a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f110(const typename std::vector<bool, T1>::const_iterator &a0, long a1) {
  return operator-(a0, a1);
}

template <typename T1>
typename std::vector<bool, T1>::const_iterator
f111(const typename std::vector<bool, T1>::const_iterator &a0, long a1) {
  return operator+(a0, a1);
}

template <typename T1>
std::ptrdiff_t f112(const typename std::vector<bool, T1>::const_iterator &a0,
                    const typename std::vector<bool, T1>::const_iterator &a1) {
  return operator-(a0, a1);
}

template <typename T1>
std::strong_ordering
f113(const typename std::vector<bool, T1>::const_iterator &a0,
     const typename std::vector<bool, T1>::const_iterator &a1) {
  return operator<=>(a0, a1);
}

template <typename T1>
bool f114(const typename std::vector<bool, T1>::const_iterator &a0,
          const typename std::vector<bool, T1>::const_iterator &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2> using t22 = std::vector<T1, T2>;

template <typename T1, typename T2>
using t23 = typename std::vector<T1, T2>::value_type;

template <typename T1, typename T2>
using t24 = typename std::vector<T1, T2>::pointer;

template <typename T1, typename T2>
using t25 = typename std::vector<T1, T2>::const_pointer;

template <typename T1, typename T2>
using t26 = typename std::vector<T1, T2>::reference;

template <typename T1, typename T2>
using t27 = typename std::vector<T1, T2>::const_reference;

template <typename T1, typename T2>
using t28 = typename std::vector<T1, T2>::iterator;

template <typename T1, typename T2>
using t29 = typename std::vector<T1, T2>::const_iterator;

template <typename T1, typename T2>
using t30 = typename std::vector<T1, T2>::const_reverse_iterator;

template <typename T1, typename T2>
using t31 = typename std::vector<T1, T2>::reverse_iterator;

template <typename T1, typename T2>
using t32 = typename std::vector<T1, T2>::size_type;

template <typename T1, typename T2>
using t33 = typename std::vector<T1, T2>::difference_type;

template <typename T1, typename T2>
using t34 = typename std::vector<T1, T2>::allocator_type;

template <typename T1, typename T2> std::vector<T1, T2> f115() {
  return std::vector<T1, T2>();
}

template <typename T1, typename T2> std::vector<T1, T2> f116(const T2 &a0) {
  return std::vector<T1, T2>(a0);
}

template <typename T1, typename T2>
std::vector<T1, T2> f117(std::size_t a0, const T2 &a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> f118(std::size_t a0, const T1 &a1, const T2 &a2) {
  return std::vector<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
std::vector<T1, T2> f119(const std::vector<T1, T2> &a0) {
  return std::vector<T1, T2>(a0);
}

template <typename T1, typename T2>
std::vector<T1, T2> f120(std::vector<T1, T2> &&a0) {
  return std::vector<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::vector<T1, T2> f121(const std::vector<T1, T2> &a0, const T2 &a1) {
  return std::vector<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> f122(std::vector<T1, T2> &&a0, const T2 &a1) {
  return std::vector<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> f123(std::initializer_list<T1> a0, const T2 &a1) {
  return std::vector<T1, T2>(std::forward<std::initializer_list<T1>>(a0), a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> &f124(std::vector<T1, T2> &a0,
                          const std::vector<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::vector<T1, T2> &f125(std::vector<T1, T2> &a0, std::vector<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::vector<T1, T2> &f126(std::vector<T1, T2> &a0,
                          std::initializer_list<T1> a1) {
  return a0.operator=(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2>
void f127(std::vector<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
void f128(std::vector<T1, T2> &a0, std::initializer_list<T1> a1) {
  return a0.assign(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator f129(std::vector<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_iterator
f130(const std::vector<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator f131(std::vector<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_iterator
f132(const std::vector<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reverse_iterator f133(std::vector<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reverse_iterator
f134(const std::vector<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reverse_iterator f135(std::vector<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reverse_iterator
f136(const std::vector<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_iterator
f137(const std::vector<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_iterator
f138(const std::vector<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reverse_iterator
f139(const std::vector<T1, T2> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reverse_iterator
f140(const std::vector<T1, T2> &a0) {
  return a0.crend();
}

template <typename T1, typename T2>
std::size_t f141(const std::vector<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
std::size_t f142(const std::vector<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
void f143(std::vector<T1, T2> &a0, std::size_t a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f144(std::vector<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2> void f145(std::vector<T1, T2> &a0) {
  return a0.shrink_to_fit();
}

template <typename T1, typename T2>
std::size_t f146(const std::vector<T1, T2> &a0) {
  return a0.capacity();
}

template <typename T1, typename T2> bool f147(const std::vector<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
void f148(std::vector<T1, T2> &a0, std::size_t a1) {
  return a0.reserve(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f149(std::vector<T1, T2> &a0,
                                             std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f150(const std::vector<T1, T2> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f151(std::vector<T1, T2> &a0,
                                             std::size_t a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f152(const std::vector<T1, T2> &a0, std::size_t a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f153(std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f154(const std::vector<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::reference f155(std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::const_reference
f156(const std::vector<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2> T1 *f157(std::vector<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
const T1 *f158(const std::vector<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
void f159(std::vector<T1, T2> &a0, const T1 &a1) {
  return a0.push_back(a1);
}

template <typename T1, typename T2>
void f160(std::vector<T1, T2> &a0, T1 &&a1) {
  return a0.push_back(std::move(a1));
}

template <typename T1, typename T2> void f161(std::vector<T1, T2> &a0) {
  return a0.pop_back();
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f162(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     const T1 &a2) {
  return a0.insert(
      std::forward<typename std::vector<T1, T2>::const_iterator>(a1), a2);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f163(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     T1 &&a2) {
  return a0.insert(
      std::forward<typename std::vector<T1, T2>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f164(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     std::initializer_list<T1> a2) {
  return a0.insert(
      std::forward<typename std::vector<T1, T2>::const_iterator>(a1),
      std::forward<std::initializer_list<T1>>(a2));
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f165(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     std::size_t a2, const T1 &a3) {
  return a0.insert(
      std::forward<typename std::vector<T1, T2>::const_iterator>(a1), a2, a3);
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f166(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1) {
  return a0.erase(
      std::forward<typename std::vector<T1, T2>::const_iterator>(a1));
}

template <typename T1, typename T2>
typename std::vector<T1, T2>::iterator
f167(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     typename std::vector<T1, T2>::const_iterator a2) {
  return a0.erase(
      std::forward<typename std::vector<T1, T2>::const_iterator>(a1),
      std::forward<typename std::vector<T1, T2>::const_iterator>(a2));
}

template <typename T1, typename T2>
void f168(std::vector<T1, T2> &a0, std::vector<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2> void f169(std::vector<T1, T2> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T3>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
std::vector<T1, T2> f170(T3 a0, T3 a1, const T2 &a2) {
  return std::vector<T1, T2>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
std::vector<T1, T2> f171(std::from_range_t a0, T3 &&a1, const T2 &a2) {
  return std::vector<T1, T2>(std::move(a0), std::forward<T3>(a1), a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T3>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
void f172(std::vector<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f173(std::vector<T1, T2> &a0, T3 &&a1) {
  return a0.template assign_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename... T3>
typename std::vector<T1, T2>::reference f174(std::vector<T1, T2> &a0,
                                             T3 &&...a1) {
  return a0.template emplace_back<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename... T3>
typename std::vector<T1, T2>::iterator
f175(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     T3 &&...a2) {
  return a0.template emplace<T3...>(
      std::forward<typename std::vector<T1, T2>::const_iterator>(a1),
      std::forward<T3>(a2)...);
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T3>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
typename std::vector<T1, T2>::iterator
f176(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     T3 a2, T3 a3) {
  return a0.template insert<T3>(
      std::forward<typename std::vector<T1, T2>::const_iterator>(a1),
      std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::vector<T1, T2>::iterator
f177(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     T3 &&a2) {
  return a0.template insert_range<T3>(
      std::forward<typename std::vector<T1, T2>::const_iterator>(a1),
      std::forward<T3>(a2));
}

template <typename T1, typename T2, typename T3>
void f178(std::vector<T1, T2> &a0, T3 &&a1) {
  return a0.template append_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2> T2 f179(const std::vector<T1, T2> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2>
using t35 = typename std::vector<T1, T2>::iterator::iterator_type;

template <typename T1, typename T2>
using t36 = typename std::vector<T1, T2>::iterator::iterator_category;

template <typename T1, typename T2>
using t37 = typename std::vector<T1, T2>::iterator::value_type;

template <typename T1, typename T2>
using t38 = typename std::vector<T1, T2>::iterator::difference_type;

template <typename T1, typename T2>
using t39 = typename std::vector<T1, T2>::iterator::reference;

template <typename T1, typename T2>
using t40 = typename std::vector<T1, T2>::iterator::pointer;

template <typename T1, typename T2>
using t41 = typename std::vector<T1, T2>::iterator::iterator_concept;

template <typename T1, typename T2>
using t42 = typename std::vector<T1, T2>::const_iterator::iterator_type;

template <typename T1, typename T2>
using t43 = typename std::vector<T1, T2>::const_iterator::iterator_category;

template <typename T1, typename T2>
using t44 = typename std::vector<T1, T2>::const_iterator::value_type;

template <typename T1, typename T2>
using t45 = typename std::vector<T1, T2>::const_iterator::difference_type;

template <typename T1, typename T2>
using t46 = typename std::vector<T1, T2>::const_iterator::reference;

template <typename T1, typename T2>
using t47 = typename std::vector<T1, T2>::const_iterator::pointer;

template <typename T1, typename T2>
using t48 = typename std::vector<T1, T2>::const_iterator::iterator_concept;
