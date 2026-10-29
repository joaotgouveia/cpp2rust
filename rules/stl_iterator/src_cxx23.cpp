#include <algorithm>
#include <utility>

template <typename T1, typename T2>
using t1 = __gnu_cxx::__normal_iterator<T1, T2>;

template <typename T1, typename T2>
using t2 = typename __gnu_cxx::__normal_iterator<T1, T2>::iterator_type;

template <typename T1, typename T2>
using t3 = typename __gnu_cxx::__normal_iterator<T1, T2>::iterator_category;

template <typename T1, typename T2>
using t4 = typename __gnu_cxx::__normal_iterator<T1, T2>::value_type;

template <typename T1, typename T2>
using t5 = typename __gnu_cxx::__normal_iterator<T1, T2>::difference_type;

template <typename T1, typename T2>
using t6 = typename __gnu_cxx::__normal_iterator<T1, T2>::reference;

template <typename T1, typename T2>
using t7 = typename __gnu_cxx::__normal_iterator<T1, T2>::pointer;

template <typename T1, typename T2>
using t8 = typename __gnu_cxx::__normal_iterator<T1, T2>::iterator_concept;

template <typename T1, typename T2> __gnu_cxx::__normal_iterator<T1, T2> f1() {
  return __gnu_cxx::__normal_iterator<T1, T2>();
}

template <typename T1, typename T2>
__gnu_cxx::__normal_iterator<T1, T2> f2(const T1 &a0) {
  return __gnu_cxx::__normal_iterator<T1, T2>(a0);
}

template <typename T1, typename T2>
typename __gnu_cxx::__normal_iterator<T1, T2>::reference
f3(const __gnu_cxx::__normal_iterator<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
typename __gnu_cxx::__normal_iterator<T1, T2>::pointer
f4(const __gnu_cxx::__normal_iterator<T1, T2> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2>
__gnu_cxx::__normal_iterator<T1, T2> &
f5(__gnu_cxx::__normal_iterator<T1, T2> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2>
__gnu_cxx::__normal_iterator<T1, T2>
f6(__gnu_cxx::__normal_iterator<T1, T2> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
__gnu_cxx::__normal_iterator<T1, T2> &
f7(__gnu_cxx::__normal_iterator<T1, T2> &a0) {
  return a0.operator--();
}

template <typename T1, typename T2>
__gnu_cxx::__normal_iterator<T1, T2>
f8(__gnu_cxx::__normal_iterator<T1, T2> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1, typename T2>
typename __gnu_cxx::__normal_iterator<T1, T2>::reference
f9(const __gnu_cxx::__normal_iterator<T1, T2> &a0,
   typename __gnu_cxx::__normal_iterator<T1, T2>::difference_type a1) {
  return a0.operator[](
      std::forward<
          typename __gnu_cxx::__normal_iterator<T1, T2>::difference_type>(a1));
}

template <typename T1, typename T2>
__gnu_cxx::__normal_iterator<T1, T2> &
f10(__gnu_cxx::__normal_iterator<T1, T2> &a0,
    typename __gnu_cxx::__normal_iterator<T1, T2>::difference_type a1) {
  return a0.operator+=(
      std::forward<
          typename __gnu_cxx::__normal_iterator<T1, T2>::difference_type>(a1));
}

template <typename T1, typename T2>
__gnu_cxx::__normal_iterator<T1, T2>
f11(const __gnu_cxx::__normal_iterator<T1, T2> &a0,
    typename __gnu_cxx::__normal_iterator<T1, T2>::difference_type a1) {
  return a0.operator+(
      std::forward<
          typename __gnu_cxx::__normal_iterator<T1, T2>::difference_type>(a1));
}

template <typename T1, typename T2>
__gnu_cxx::__normal_iterator<T1, T2> &
f12(__gnu_cxx::__normal_iterator<T1, T2> &a0,
    typename __gnu_cxx::__normal_iterator<T1, T2>::difference_type a1) {
  return a0.operator-=(
      std::forward<
          typename __gnu_cxx::__normal_iterator<T1, T2>::difference_type>(a1));
}

template <typename T1, typename T2>
__gnu_cxx::__normal_iterator<T1, T2>
f13(const __gnu_cxx::__normal_iterator<T1, T2> &a0,
    typename __gnu_cxx::__normal_iterator<T1, T2>::difference_type a1) {
  return a0.operator-(
      std::forward<
          typename __gnu_cxx::__normal_iterator<T1, T2>::difference_type>(a1));
}

template <typename T1, typename T2>
const T1 &f14(const __gnu_cxx::__normal_iterator<T1, T2> &a0) {
  return a0.base();
}

template <typename T1, typename T2, typename T3>
__gnu_cxx::__normal_iterator<T1, T2>
f15(const __gnu_cxx::__normal_iterator<T3, T2> &a0)
  requires(std::is_convertible_v<T3, T1>)
{
  return __gnu_cxx::__normal_iterator<T1, T2>(a0);
}

template <typename T1, typename T2>
auto f16(const __gnu_cxx::__normal_iterator<T1, T2> &a0,
         const __gnu_cxx::__normal_iterator<T1, T2> &a1) {
  return __gnu_cxx::operator- <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
auto f17(const __gnu_cxx::__normal_iterator<T1, T3> &a0,
         const __gnu_cxx::__normal_iterator<T2, T3> &a1) {
  return __gnu_cxx::operator- <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2>
auto f18(const __gnu_cxx::__normal_iterator<T1, T2> &a0,
         const __gnu_cxx::__normal_iterator<T1, T2> &a1) {
  return __gnu_cxx::operator<=> <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
auto f19(const __gnu_cxx::__normal_iterator<T1, T3> &a0,
         const __gnu_cxx::__normal_iterator<T2, T3> &a1) {
  return __gnu_cxx::operator<=> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2>
bool f20(const __gnu_cxx::__normal_iterator<T1, T2> &a0,
         const __gnu_cxx::__normal_iterator<T1, T2> &a1) {
  return __gnu_cxx::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f21(const __gnu_cxx::__normal_iterator<T1, T3> &a0,
         const __gnu_cxx::__normal_iterator<T2, T3> &a1) {
  return __gnu_cxx::operator== <T1, T2, T3>(a0, a1);
}

template <typename T1> std::back_insert_iterator<T1> f22(T1 &a0) {
  return std::back_inserter<T1>(a0);
}

template <typename T1> using t9 = std::const_iterator<T1>;

template <typename T1> using t10 = std::const_sentinel<T1>;

template <typename T1> std::front_insert_iterator<T1> f23(T1 &a0) {
  return std::front_inserter<T1>(a0);
}

template <typename T1> using t11 = std::iter_const_reference_t<T1>;

template <typename T1>
std::const_iterator<T1> f24(T1 a0)
  requires(std::input_iterator<T1>)
{
  return std::make_const_iterator<T1>(std::move(a0));
}

template <typename T1>
std::const_sentinel<T1> f25(T1 a0)
  requires(std::semiregular<T1>)
{
  return std::make_const_sentinel<T1>(std::move(a0));
}

template <typename T1> std::move_iterator<T1> f26(T1 a0) {
  return std::make_move_iterator<T1>(std::move(a0));
}

template <typename T1> std::reverse_iterator<T1> f27(T1 a0) {
  return std::make_reverse_iterator<T1>(std::move(a0));
}

template <typename T1, typename T2>
bool f28(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1>
std::move_iterator<T1> f29(typename std::move_iterator<T1>::difference_type a0,
                           const std::move_iterator<T1> &a1) {
  return std::operator+ <T1>(
      std::forward<typename std::move_iterator<T1>::difference_type>(a0), a1);
}

template <typename T1>
std::reverse_iterator<T1>
f30(typename std::reverse_iterator<T1>::difference_type a0,
    const std::reverse_iterator<T1> &a1) {
  return std::operator+ <T1>(
      std::forward<typename std::reverse_iterator<T1>::difference_type>(a0),
      a1);
}

template <typename T1, typename T2>
auto f31(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator- <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
auto f32(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator- <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f33(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f34(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f35(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f36(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1>
std::compare_three_way_result_t<T1> f37(const std::move_iterator<T1> &a0,
                                        const std::move_iterator<T1> &a1)
  requires(std::three_way_comparable<T1>)
{
  return std::operator<=> <T1>(a0, a1);
}

template <typename T1>
std::compare_three_way_result_t<T1, T1> f38(const std::reverse_iterator<T1> &a0,
                                            const std::reverse_iterator<T1> &a1)
  requires(std::three_way_comparable<T1>)
{
  return std::operator<=> <T1>(a0, a1);
}

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2> f39(const std::move_iterator<T1> &a0,
                                            const std::move_iterator<T2> &a1)
  requires(std::three_way_comparable_with<T2, T1>)
{
  return std::operator<=> <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2> f40(const std::reverse_iterator<T1> &a0,
                                            const std::reverse_iterator<T2> &a1)
  requires(std::three_way_comparable_with<T2, T1>)
{
  return std::operator<=> <T1, T2>(a0, a1);
}

template <typename T1>
bool f41(const std::move_iterator<T1> &a0, const std::move_iterator<T1> &a1) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1>
bool f42(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T1> &a1) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1, typename T2>
bool f43(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f44(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f45(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f46(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f47(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f48(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}
