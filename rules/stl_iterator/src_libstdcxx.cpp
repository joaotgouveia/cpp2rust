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

template <typename T1, typename T2>
auto f15(const __gnu_cxx::__normal_iterator<T1, T2> &a0,
         const __gnu_cxx::__normal_iterator<T1, T2> &a1)
    -> decltype(__gnu_cxx::operator- <T1, T2>(a0, a1)) {
  return __gnu_cxx::operator- <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
auto f16(const __gnu_cxx::__normal_iterator<T1, T3> &a0,
         const __gnu_cxx::__normal_iterator<T2, T3> &a1)
    -> decltype(__gnu_cxx::operator- <T1, T2, T3>(a0, a1)) {
  return __gnu_cxx::operator- <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2>
bool f17(const __gnu_cxx::__normal_iterator<T1, T2> &a0,
         const __gnu_cxx::__normal_iterator<T1, T2> &a1) {
  return __gnu_cxx::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f18(const __gnu_cxx::__normal_iterator<T1, T3> &a0,
         const __gnu_cxx::__normal_iterator<T2, T3> &a1) {
  return __gnu_cxx::operator== <T1, T2, T3>(a0, a1);
}

template <typename T1> std::back_insert_iterator<T1> f19(T1 &a0) {
  return std::back_inserter<T1>(a0);
}

template <typename T1> std::front_insert_iterator<T1> f20(T1 &a0) {
  return std::front_inserter<T1>(a0);
}

template <typename T1> std::move_iterator<T1> f21(T1 a0) {
  return std::make_move_iterator<T1>(std::move(a0));
}

template <typename T1, typename T2>
bool f22(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1>
std::move_iterator<T1> f23(typename std::move_iterator<T1>::difference_type a0,
                           const std::move_iterator<T1> &a1) {
  return std::operator+ <T1>(
      std::forward<typename std::move_iterator<T1>::difference_type>(a0), a1);
}

template <typename T1>
std::reverse_iterator<T1>
f24(typename std::reverse_iterator<T1>::difference_type a0,
    const std::reverse_iterator<T1> &a1) {
  return std::operator+ <T1>(
      std::forward<typename std::reverse_iterator<T1>::difference_type>(a0),
      a1);
}

template <typename T1, typename T2>
auto f25(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1)
    -> decltype(std::operator- <T1, T2>(a0, a1)) {
  return std::operator- <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
auto f26(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1)
    -> decltype(std::operator- <T1, T2>(a0, a1)) {
  return std::operator- <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f27(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f28(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f29(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f30(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1>
bool f31(const std::move_iterator<T1> &a0, const std::move_iterator<T1> &a1) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1>
bool f32(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T1> &a1) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1, typename T2>
bool f33(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f34(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f35(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f36(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f37(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f38(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}
