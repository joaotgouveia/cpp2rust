#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1> std::back_insert_iterator<T1> f1(T1 &a0) {
  return std::back_inserter<T1>(a0);
}

template <typename T1> std::front_insert_iterator<T1> f2(T1 &a0) {
  return std::front_inserter<T1>(a0);
}

template <typename T1> std::move_iterator<T1> f3(T1 a0) {
  return std::make_move_iterator<T1>(a0);
}

template <typename T1, typename T2>
bool f4(const std::reverse_iterator<T1> &a0,
        const std::reverse_iterator<T2> &a1) {
  return a0 != a1;
}

template <typename T1>
std::move_iterator<T1> f5(typename std::move_iterator<T1>::difference_type a0,
                          const std::move_iterator<T1> &a1) {
  return a0 + a1;
}

template <typename T1>
std::reverse_iterator<T1>
f6(typename std::reverse_iterator<T1>::difference_type a0,
   const std::reverse_iterator<T1> &a1) {
  return a0 + a1;
}

template <typename T1, typename T2>
auto f7(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1)
    -> decltype(a0 - a1) {
  return a0 - a1;
}

template <typename T1, typename T2>
auto f8(const std::reverse_iterator<T1> &a0,
        const std::reverse_iterator<T2> &a1) -> decltype(a0 - a1) {
  return a0 - a1;
}

template <typename T1, typename T2>
bool f9(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f10(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f11(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f12(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return a0 <= a1;
}

template <typename T1>
bool f13(const std::move_iterator<T1> &a0, const std::move_iterator<T1> &a1) {
  return a0 == a1;
}

template <typename T1>
bool f14(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T1> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
bool f15(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
bool f16(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
bool f17(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f18(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f19(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2>
bool f20(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return a0 >= a1;
}
