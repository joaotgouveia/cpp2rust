#include <flat_map>
#include <utility>

template <typename T1, typename T2, typename T3, bool T4>
using t1 = std::__key_value_iterator<T1, T2, T3, T4>;

template <typename T1, typename T2, typename T3, bool T4>
using t2 = typename std::__key_value_iterator<T1, T2, T3, T4>::iterator_concept;

template <typename T1, typename T2, typename T3, bool T4>
using t3 =
    typename std::__key_value_iterator<T1, T2, T3, T4>::iterator_category;

template <typename T1, typename T2, typename T3, bool T4>
using t4 = typename std::__key_value_iterator<T1, T2, T3, T4>::value_type;

template <typename T1, typename T2, typename T3, bool T4>
using t5 = typename std::__key_value_iterator<T1, T2, T3, T4>::difference_type;

template <typename T1, typename T2, typename T3, bool T4>
std::__key_value_iterator<T1, T2, T3, T4> f1() {
  return std::__key_value_iterator<T1, T2, T3, T4>();
}

template <typename T1, typename T2, typename T3, bool T4>
std::__key_value_iterator<T1, T2, T3, T4>
f2(std::__key_value_iterator<T1, T2, T3, !T4> a0)
  requires(
      T4 &&
      std::convertible_to<
          typename T2::iterator,
          typename std::__key_value_iterator<T1, T2, T3, T4>::__key_iterator> &&
      std::convertible_to<typename T3::iterator,
                          typename std::__key_value_iterator<
                              T1, T2, T3, T4>::__mapped_iterator>)
{
  return std::__key_value_iterator<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, bool T4>
auto f3(const std::__key_value_iterator<T1, T2, T3, T4> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, typename T3, bool T4>
auto f4(const std::__key_value_iterator<T1, T2, T3, T4> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2, typename T3, bool T4>
std::__key_value_iterator<T1, T2, T3, T4> &
f5(std::__key_value_iterator<T1, T2, T3, T4> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2, typename T3, bool T4>
std::__key_value_iterator<T1, T2, T3, T4>
f6(std::__key_value_iterator<T1, T2, T3, T4> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2, typename T3, bool T4>
std::__key_value_iterator<T1, T2, T3, T4> &
f7(std::__key_value_iterator<T1, T2, T3, T4> &a0) {
  return a0.operator--();
}

template <typename T1, typename T2, typename T3, bool T4>
std::__key_value_iterator<T1, T2, T3, T4>
f8(std::__key_value_iterator<T1, T2, T3, T4> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1, typename T2, typename T3, bool T4>
std::__key_value_iterator<T1, T2, T3, T4> &
f9(std::__key_value_iterator<T1, T2, T3, T4> &a0,
   typename std::__key_value_iterator<T1, T2, T3, T4>::difference_type a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2, typename T3, bool T4>
std::__key_value_iterator<T1, T2, T3, T4> &
f10(std::__key_value_iterator<T1, T2, T3, T4> &a0,
    typename std::__key_value_iterator<T1, T2, T3, T4>::difference_type a1) {
  return a0.operator-=(a1);
}

template <typename T1, typename T2, typename T3, bool T4>
auto f11(
    const std::__key_value_iterator<T1, T2, T3, T4> &a0,
    typename std::__key_value_iterator<T1, T2, T3, T4>::difference_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3, bool T4>
typename std::__key_value_iterator<T1, T2, T3, T4>::difference_type
f12(const std::__key_value_iterator<T1, T2, T3, T4> &a0,
    const std::__key_value_iterator<T1, T2, T3, T4> &a1) {
  return operator-(a0, a1);
}

template <typename T1, typename T2, typename T3, bool T4>
std::__key_value_iterator<T1, T2, T3, T4>
f13(const std::__key_value_iterator<T1, T2, T3, T4> &a0,
    typename std::__key_value_iterator<T1, T2, T3, T4>::difference_type a1) {
  return operator-(a0, a1);
}

template <typename T1, typename T2, typename T3, bool T4>
std::__key_value_iterator<T1, T2, T3, T4>
f14(typename std::__key_value_iterator<T1, T2, T3, T4>::difference_type a0,
    const std::__key_value_iterator<T1, T2, T3, T4> &a1) {
  return operator+(a0, a1);
}

template <typename T1, typename T2, typename T3, bool T4>
std::__key_value_iterator<T1, T2, T3, T4>
f15(const std::__key_value_iterator<T1, T2, T3, T4> &a0,
    typename std::__key_value_iterator<T1, T2, T3, T4>::difference_type a1) {
  return operator+(a0, a1);
}

template <typename T1, typename T2, typename T3, bool T4>
auto f16(const std::__key_value_iterator<T1, T2, T3, T4> &a0,
         const std::__key_value_iterator<T1, T2, T3, T4> &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, typename T2, typename T3, bool T4>
bool f17(const std::__key_value_iterator<T1, T2, T3, T4> &a0,
         const std::__key_value_iterator<T1, T2, T3, T4> &a1) {
  return operator>=(a0, a1);
}

template <typename T1, typename T2, typename T3, bool T4>
bool f18(const std::__key_value_iterator<T1, T2, T3, T4> &a0,
         const std::__key_value_iterator<T1, T2, T3, T4> &a1) {
  return operator<=(a0, a1);
}

template <typename T1, typename T2, typename T3, bool T4>
bool f19(const std::__key_value_iterator<T1, T2, T3, T4> &a0,
         const std::__key_value_iterator<T1, T2, T3, T4> &a1) {
  return operator>(a0, a1);
}

template <typename T1, typename T2, typename T3, bool T4>
bool f20(const std::__key_value_iterator<T1, T2, T3, T4> &a0,
         const std::__key_value_iterator<T1, T2, T3, T4> &a1) {
  return operator<(a0, a1);
}

template <typename T1, typename T2, typename T3, bool T4>
bool f21(const std::__key_value_iterator<T1, T2, T3, T4> &a0,
         const std::__key_value_iterator<T1, T2, T3, T4> &a1) {
  return operator==(a0, a1);
}
