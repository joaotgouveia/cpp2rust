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
  return a0.operator[](a1);
}

template <typename T1, typename T2>
__gnu_cxx::__normal_iterator<T1, T2> &
f10(__gnu_cxx::__normal_iterator<T1, T2> &a0,
    typename __gnu_cxx::__normal_iterator<T1, T2>::difference_type a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2>
__gnu_cxx::__normal_iterator<T1, T2>
f11(const __gnu_cxx::__normal_iterator<T1, T2> &a0,
    typename __gnu_cxx::__normal_iterator<T1, T2>::difference_type a1) {
  return a0.operator+(a1);
}

template <typename T1, typename T2>
__gnu_cxx::__normal_iterator<T1, T2> &
f12(__gnu_cxx::__normal_iterator<T1, T2> &a0,
    typename __gnu_cxx::__normal_iterator<T1, T2>::difference_type a1) {
  return a0.operator-=(a1);
}

template <typename T1, typename T2>
__gnu_cxx::__normal_iterator<T1, T2>
f13(const __gnu_cxx::__normal_iterator<T1, T2> &a0,
    typename __gnu_cxx::__normal_iterator<T1, T2>::difference_type a1) {
  return a0.operator-(a1);
}

template <typename T1, typename T2>
const T1 &f14(const __gnu_cxx::__normal_iterator<T1, T2> &a0) {
  return a0.base();
}

template <typename T1> std::back_insert_iterator<T1> f15(T1 &a0) {
  return std::back_inserter<T1>(a0);
}

template <typename T1> std::front_insert_iterator<T1> f16(T1 &a0) {
  return std::front_inserter<T1>(a0);
}

template <typename T1> std::move_iterator<T1> f17(T1 a0) {
  return std::make_move_iterator<T1>(a0);
}

template <typename T1, typename T2>
bool f18(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1>
std::move_iterator<T1> f19(typename std::move_iterator<T1>::difference_type a0,
                           const std::move_iterator<T1> &a1) {
  return std::operator+ <T1>(a0, a1);
}

template <typename T1>
std::reverse_iterator<T1>
f20(typename std::reverse_iterator<T1>::difference_type a0,
    const std::reverse_iterator<T1> &a1) {
  return std::operator+ <T1>(a0, a1);
}

template <typename T1, typename T2>
auto f21(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1)
    -> decltype(std::operator- <T1, T2>(a0, a1)) {
  return std::operator- <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
auto f22(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1)
    -> decltype(std::operator- <T1, T2>(a0, a1)) {
  return std::operator- <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f23(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f24(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f25(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f26(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1>
bool f27(const std::move_iterator<T1> &a0, const std::move_iterator<T1> &a1) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1>
bool f28(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T1> &a1) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1, typename T2>
bool f29(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f30(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f31(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f32(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f33(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f34(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}
