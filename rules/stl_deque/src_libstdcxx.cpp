#include <algorithm>
#include <deque>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::_Deque_iterator<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = typename std::_Deque_iterator<T1, T2, T3>::iterator;

template <typename T1, typename T2, typename T3>
using t3 = typename std::_Deque_iterator<T1, T2, T3>::const_iterator;

template <typename T1, typename T2, typename T3>
using t4 = typename std::_Deque_iterator<T1, T2, T3>::iterator_category;

template <typename T1, typename T2, typename T3>
using t5 = typename std::_Deque_iterator<T1, T2, T3>::value_type;

template <typename T1, typename T2, typename T3>
using t6 = typename std::_Deque_iterator<T1, T2, T3>::pointer;

template <typename T1, typename T2, typename T3>
using t7 = typename std::_Deque_iterator<T1, T2, T3>::reference;

template <typename T1, typename T2, typename T3>
using t8 = typename std::_Deque_iterator<T1, T2, T3>::size_type;

template <typename T1, typename T2, typename T3>
using t9 = typename std::_Deque_iterator<T1, T2, T3>::difference_type;

template <typename T1, typename T2, typename T3>
std::_Deque_iterator<T1, T2, T3>
f1(typename std::pointer_traits<T3>::template rebind<T1> a0,
   typename std::pointer_traits<T3>::template rebind<
       typename std::pointer_traits<T3>::template rebind<T1>>
       a1) {
  return std::_Deque_iterator<T1, T2, T3>(
      std::forward<typename std::pointer_traits<T3>::template rebind<T1>>(a0),
      std::forward<typename std::pointer_traits<T3>::template rebind<
          typename std::pointer_traits<T3>::template rebind<T1>>>(a1));
}

template <typename T1, typename T2, typename T3>
std::_Deque_iterator<T1, T2, T3> f2() {
  return std::_Deque_iterator<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::_Deque_iterator<T1, T2, T3>
f3(const std::_Deque_iterator<T1, T2, T3> &a0) {
  return std::_Deque_iterator<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::_Deque_iterator<T1, T2, T3> &
f4(std::_Deque_iterator<T1, T2, T3> &a0,
   const std::_Deque_iterator<T1, T2, T3> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
T2 f5(const std::_Deque_iterator<T1, T2, T3> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, typename T3>
T3 f6(const std::_Deque_iterator<T1, T2, T3> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2, typename T3>
std::_Deque_iterator<T1, T2, T3> &f7(std::_Deque_iterator<T1, T2, T3> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2, typename T3>
std::_Deque_iterator<T1, T2, T3> f8(std::_Deque_iterator<T1, T2, T3> &a0,
                                    int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2, typename T3>
std::_Deque_iterator<T1, T2, T3> &f9(std::_Deque_iterator<T1, T2, T3> &a0) {
  return a0.operator--();
}

template <typename T1, typename T2, typename T3>
std::_Deque_iterator<T1, T2, T3> f10(std::_Deque_iterator<T1, T2, T3> &a0,
                                     int a1) {
  return a0.operator--(a1);
}

template <typename T1, typename T2, typename T3>
std::_Deque_iterator<T1, T2, T3> &f11(std::_Deque_iterator<T1, T2, T3> &a0,
                                      std::ptrdiff_t a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2, typename T3>
std::_Deque_iterator<T1, T2, T3> &f12(std::_Deque_iterator<T1, T2, T3> &a0,
                                      std::ptrdiff_t a1) {
  return a0.operator-=(a1);
}

template <typename T1, typename T2, typename T3>
T2 f13(const std::_Deque_iterator<T1, T2, T3> &a0, std::ptrdiff_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3>
std::_Deque_iterator<T1, T2, T3>
f14(std::ptrdiff_t a0, const std::_Deque_iterator<T1, T2, T3> &a1) {
  return operator+(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::_Deque_iterator<T1, T2, T3> f15(const std::_Deque_iterator<T1, T2, T3> &a0,
                                     std::ptrdiff_t a1) {
  return operator-(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::_Deque_iterator<T1, T2, T3> f16(const std::_Deque_iterator<T1, T2, T3> &a0,
                                     std::ptrdiff_t a1) {
  return operator+(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::ptrdiff_t f17(const std::_Deque_iterator<T1, T2, T3> &a0,
                   const std::_Deque_iterator<T1, T4, T5> &a1) {
  return operator-(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::ptrdiff_t f18(const std::_Deque_iterator<T1, T2, T3> &a0,
                   const std::_Deque_iterator<T1, T2, T3> &a1) {
  return operator-(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f19(const std::_Deque_iterator<T1, T2, T3> &a0,
         const std::_Deque_iterator<T1, T4, T5> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f20(const std::_Deque_iterator<T1, T2, T3> &a0,
         const std::_Deque_iterator<T1, T2, T3> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2>
bool f21(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
void f22(std::deque<T1, T2> &a0, std::deque<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
