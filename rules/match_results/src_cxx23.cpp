#include <regex>
#include <string>
#include <utility>

template <typename T1, typename T2> using t1 = std::match_results<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::match_results<T1, T2>::value_type;

template <typename T1, typename T2>
using t3 = typename std::match_results<T1, T2>::const_reference;

template <typename T1, typename T2>
using t4 = typename std::match_results<T1, T2>::reference;

template <typename T1, typename T2>
using t5 = typename std::match_results<T1, T2>::const_iterator;

template <typename T1, typename T2>
using t6 = typename std::match_results<T1, T2>::iterator;

template <typename T1, typename T2>
using t7 = typename std::match_results<T1, T2>::difference_type;

template <typename T1, typename T2>
using t8 = typename std::match_results<T1, T2>::size_type;

template <typename T1, typename T2>
using t9 = typename std::match_results<T1, T2>::allocator_type;

template <typename T1, typename T2>
using t10 = typename std::match_results<T1, T2>::char_type;

template <typename T1, typename T2>
using t11 = typename std::match_results<T1, T2>::string_type;

template <typename T1, typename T2> std::match_results<T1, T2> f1() {
  return std::match_results<T1, T2>();
}

template <typename T1, typename T2>
std::match_results<T1, T2> f2(const T2 &a0) {
  return std::match_results<T1, T2>(a0);
}

template <typename T1, typename T2>
std::match_results<T1, T2> f3(const std::match_results<T1, T2> &a0) {
  return std::match_results<T1, T2>(a0);
}

template <typename T1, typename T2>
std::match_results<T1, T2> f4(std::match_results<T1, T2> &&a0) {
  return std::match_results<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::match_results<T1, T2> &f5(std::match_results<T1, T2> &a0,
                               const std::match_results<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::match_results<T1, T2> &f6(std::match_results<T1, T2> &a0,
                               std::match_results<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::match_results<T1, T2> f7(const std::match_results<T1, T2> &a0,
                              const T2 &a1) {
  return std::match_results<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::match_results<T1, T2> f8(std::match_results<T1, T2> &&a0, const T2 &a1) {
  return std::match_results<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
bool f9(const std::match_results<T1, T2> &a0) {
  return a0.ready();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::size_type
f10(const std::match_results<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::size_type
f11(const std::match_results<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
bool f12(const std::match_results<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::difference_type
f13(const std::match_results<T1, T2> &a0,
    typename std::match_results<T1, T2>::size_type a1) {
  return a0.length(
      std::forward<typename std::match_results<T1, T2>::size_type>(a1));
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::difference_type
f14(const std::match_results<T1, T2> &a0,
    typename std::match_results<T1, T2>::size_type a1) {
  return a0.position(
      std::forward<typename std::match_results<T1, T2>::size_type>(a1));
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::string_type
f15(const std::match_results<T1, T2> &a0,
    typename std::match_results<T1, T2>::size_type a1) {
  return a0.str(
      std::forward<typename std::match_results<T1, T2>::size_type>(a1));
}

template <typename T1, typename T2>
const std::sub_match<T1> &
f16(const std::match_results<T1, T2> &a0,
    typename std::match_results<T1, T2>::size_type a1) {
  return a0.operator[](
      std::forward<typename std::match_results<T1, T2>::size_type>(a1));
}

template <typename T1, typename T2>
const std::sub_match<T1> &f17(const std::match_results<T1, T2> &a0) {
  return a0.prefix();
}

template <typename T1, typename T2>
const std::sub_match<T1> &f18(const std::match_results<T1, T2> &a0) {
  return a0.suffix();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator
f19(const std::match_results<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator
f20(const std::match_results<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator
f21(const std::match_results<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator
f22(const std::match_results<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::string_type
f23(const std::match_results<T1, T2> &a0,
    const typename std::match_results<T1, T2>::char_type *a1,
    std::regex_constants::match_flag_type a2) {
  return a0.format(a1, a2);
}

template <typename T1, typename T2>
T2 f24(const std::match_results<T1, T2> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2>
void f25(std::match_results<T1, T2> &a0, std::match_results<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
T3 f26(const std::match_results<T1, T2> &a0, T3 a1,
       const typename std::match_results<T1, T2>::char_type *a2,
       const typename std::match_results<T1, T2>::char_type *a3,
       std::regex_constants::match_flag_type a4) {
  return a0.template format<T3>(std::move(a1), a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T3 f27(const std::match_results<T1, T2> &a0, T3 a1,
       const std::basic_string<typename std::match_results<T1, T2>::char_type,
                               T4, T5> &a2,
       std::regex_constants::match_flag_type a3) {
  return a0.template format<T3, T4, T5>(std::move(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<typename std::match_results<T1, T2>::char_type, T3, T4>
f28(const std::match_results<T1, T2> &a0,
    const std::basic_string<typename std::match_results<T1, T2>::char_type, T3,
                            T4> &a1,
    std::regex_constants::match_flag_type a2) {
  return a0.template format<T3, T4>(a1, a2);
}

template <typename T1, typename T2>
using t12 = typename std::match_results<T1, T2>::const_iterator::iterator_type;

template <typename T1, typename T2>
using t13 =
    typename std::match_results<T1, T2>::const_iterator::iterator_category;

template <typename T1, typename T2>
using t14 = typename std::match_results<T1, T2>::const_iterator::value_type;

template <typename T1, typename T2>
using t15 =
    typename std::match_results<T1, T2>::const_iterator::difference_type;

template <typename T1, typename T2>
using t16 = typename std::match_results<T1, T2>::const_iterator::reference;

template <typename T1, typename T2>
using t17 = typename std::match_results<T1, T2>::const_iterator::pointer;

template <typename T1, typename T2>
using t18 =
    typename std::match_results<T1, T2>::const_iterator::iterator_concept;

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator f29() {
  return typename std::match_results<T1, T2>::const_iterator();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator
f30(const typename std::allocator_traits<T1>::const_pointer &a0) {
  return typename std::match_results<T1, T2>::const_iterator(a0);
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator::reference
f31(const typename std::match_results<T1, T2>::const_iterator &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator::pointer
f32(const typename std::match_results<T1, T2>::const_iterator &a0) {
  return a0.operator->();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator &
f33(typename std::match_results<T1, T2>::const_iterator &a0) {
  return a0.operator++();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator
f34(typename std::match_results<T1, T2>::const_iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator &
f35(typename std::match_results<T1, T2>::const_iterator &a0) {
  return a0.operator--();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator
f36(typename std::match_results<T1, T2>::const_iterator &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator::reference
f37(const typename std::match_results<T1, T2>::const_iterator &a0,
    typename std::match_results<T1, T2>::const_iterator::difference_type a1) {
  return a0.operator[](
      std::forward<
          typename std::match_results<T1, T2>::const_iterator::difference_type>(
          a1));
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator &
f38(typename std::match_results<T1, T2>::const_iterator &a0,
    typename std::match_results<T1, T2>::const_iterator::difference_type a1) {
  return a0.operator+=(
      std::forward<
          typename std::match_results<T1, T2>::const_iterator::difference_type>(
          a1));
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator
f39(const typename std::match_results<T1, T2>::const_iterator &a0,
    typename std::match_results<T1, T2>::const_iterator::difference_type a1) {
  return a0.operator+(
      std::forward<
          typename std::match_results<T1, T2>::const_iterator::difference_type>(
          a1));
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator &
f40(typename std::match_results<T1, T2>::const_iterator &a0,
    typename std::match_results<T1, T2>::const_iterator::difference_type a1) {
  return a0.operator-=(
      std::forward<
          typename std::match_results<T1, T2>::const_iterator::difference_type>(
          a1));
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator
f41(const typename std::match_results<T1, T2>::const_iterator &a0,
    typename std::match_results<T1, T2>::const_iterator::difference_type a1) {
  return a0.operator-(
      std::forward<
          typename std::match_results<T1, T2>::const_iterator::difference_type>(
          a1));
}

template <typename T1, typename T2>
const typename std::allocator_traits<T1>::const_pointer &
f42(const typename std::match_results<T1, T2>::const_iterator &a0) {
  return a0.base();
}

template <typename T1, typename T2>
bool f43(const typename std::match_results<T1, T2>::const_iterator &a0,
         const typename std::match_results<T1, T2>::const_iterator &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2>
auto f44(const typename std::match_results<T1, T2>::const_iterator &a0,
         const typename std::match_results<T1, T2>::const_iterator &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, typename T2>
auto f45(const typename std::match_results<T1, T2>::const_iterator &a0,
         const typename std::match_results<T1, T2>::const_iterator &a1) {
  return operator-(a0, a1);
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator
f46(typename std::iterator_traits<
        typename std::allocator_traits<T1>::const_pointer>::difference_type a0,
    const typename std::match_results<T1, T2>::const_iterator &a1) {
  return operator+(
      std::forward<typename std::iterator_traits<
          typename std::allocator_traits<T1>::const_pointer>::difference_type>(
          a0),
      a1);
}
