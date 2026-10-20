#include "../common/init.h"
#include "../common/traits.h"
#include <utility>
#include <valarray>

template <typename T1>
auto f1(const std::valarray<T1> &a0) -> decltype(std::abs<T1>(a0)) {
  return std::abs<T1>(a0);
}

template <typename T1>
auto f2(const std::valarray<T1> &a0) -> decltype(std::acos<T1>(a0)) {
  return std::acos<T1>(a0);
}

template <typename T1>
auto f3(const std::valarray<T1> &a0) -> decltype(std::asin<T1>(a0)) {
  return std::asin<T1>(a0);
}

template <typename T1>
auto f4(const std::valarray<T1> &a0) -> decltype(std::atan<T1>(a0)) {
  return std::atan<T1>(a0);
}

template <typename T1>
auto f5(const std::valarray<T1> &a0,
        const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::atan2<T1>(a0, a1)) {
  return std::atan2<T1>(a0, a1);
}

template <typename T1>
auto f6(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::atan2<T1>(a0, a1)) {
  return std::atan2<T1>(a0, a1);
}

template <typename T1>
auto f7(const typename std::valarray<T1>::value_type &a0,
        const std::valarray<T1> &a1) -> decltype(std::atan2<T1>(a0, a1)) {
  return std::atan2<T1>(a0, a1);
}

template <typename T1>
auto f8(const std::valarray<T1> &a0) -> decltype(std::cos<T1>(a0)) {
  return std::cos<T1>(a0);
}

template <typename T1>
auto f9(const std::valarray<T1> &a0) -> decltype(std::cosh<T1>(a0)) {
  return std::cosh<T1>(a0);
}

template <typename T1>
auto f10(const std::valarray<T1> &a0) -> decltype(std::exp<T1>(a0)) {
  return std::exp<T1>(a0);
}

template <typename T1>
auto f11(const std::valarray<T1> &a0) -> decltype(std::log<T1>(a0)) {
  return std::log<T1>(a0);
}

template <typename T1>
auto f12(const std::valarray<T1> &a0) -> decltype(std::log10<T1>(a0)) {
  return std::log10<T1>(a0);
}

template <typename T1>
auto f13(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1)
    -> decltype(std::pow<T1>(a0, a1)) {
  return std::pow<T1>(a0, a1);
}

template <typename T1>
auto f14(const std::valarray<T1> &a0, const std::valarray<T1> &a1)
    -> decltype(std::pow<T1>(a0, a1)) {
  return std::pow<T1>(a0, a1);
}

template <typename T1>
auto f15(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) -> decltype(std::pow<T1>(a0, a1)) {
  return std::pow<T1>(a0, a1);
}

template <typename T1>
auto f16(const std::valarray<T1> &a0) -> decltype(std::sin<T1>(a0)) {
  return std::sin<T1>(a0);
}

template <typename T1>
auto f17(const std::valarray<T1> &a0) -> decltype(std::sinh<T1>(a0)) {
  return std::sinh<T1>(a0);
}

template <typename T1>
auto f18(const std::valarray<T1> &a0) -> decltype(std::sqrt<T1>(a0)) {
  return std::sqrt<T1>(a0);
}

template <typename T1>
auto f19(const std::valarray<T1> &a0) -> decltype(std::tan<T1>(a0)) {
  return std::tan<T1>(a0);
}

template <typename T1>
auto f20(const std::valarray<T1> &a0) -> decltype(std::tanh<T1>(a0)) {
  return std::tanh<T1>(a0);
}
