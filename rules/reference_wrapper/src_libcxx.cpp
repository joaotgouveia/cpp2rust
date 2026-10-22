#include "../common/init.h"
#include "../common/traits.h"
#include <functional>
#include <utility>

template <typename T1> std::reference_wrapper<const T1> f6(const T1 &a0) {
  return std::cref<T1>(a0);
}

template <typename T1>
std::reference_wrapper<const T1> f7(std::reference_wrapper<T1> a0) {
  return std::cref<T1>(a0);
}

template <typename T1> std::reference_wrapper<T1> f8(T1 &a0) {
  return std::ref<T1>(a0);
}

template <typename T1>
std::reference_wrapper<T1> f9(std::reference_wrapper<T1> a0) {
  return std::ref<T1>(a0);
}

template <typename T1>
typename std::reference_wrapper<T1>::type &
f10(const std::reference_wrapper<T1> &a0) {
  return static_cast<typename std::reference_wrapper<T1>::type &>(a0);
}

template <typename T1>
typename std::reference_wrapper<T1>::type &
f11(const std::reference_wrapper<T1> &a0) {
  return a0.get();
}
