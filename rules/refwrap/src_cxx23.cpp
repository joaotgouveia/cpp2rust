#include <barrier>
#include <utility>

template <typename T1> std::reference_wrapper<const T1> f1(const T1 &a0) {
  return std::cref<T1>(a0);
}

template <typename T1>
std::reference_wrapper<const T1> f2(std::reference_wrapper<T1> a0) {
  return std::cref<T1>(std::forward<std::reference_wrapper<T1>>(a0));
}

template <typename T1> std::reference_wrapper<T1> f3(T1 &a0) {
  return std::ref<T1>(a0);
}

template <typename T1>
std::reference_wrapper<T1> f4(std::reference_wrapper<T1> a0) {
  return std::ref<T1>(std::forward<std::reference_wrapper<T1>>(a0));
}
