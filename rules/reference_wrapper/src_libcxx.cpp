#include <functional>
#include <utility>

template <typename T1> std::reference_wrapper<const T1> f8(const T1 &a0) {
  return std::cref<T1>(a0);
}

template <typename T1>
std::reference_wrapper<const T1> f9(std::reference_wrapper<T1> a0) {
  return std::cref<T1>(std::move(a0));
}

template <typename T1> std::reference_wrapper<T1> f10(T1 &a0) {
  return std::ref<T1>(a0);
}

template <typename T1>
std::reference_wrapper<T1> f11(std::reference_wrapper<T1> a0) {
  return std::ref<T1>(std::move(a0));
}

template <typename T1, typename T2, typename T3 = void,
          std::__enable_if_t<!std::is_same<std::__remove_cvref_t<T2>,
                                           std::reference_wrapper<T1>>::value,
                             int>
              T4 = 0>
std::reference_wrapper<T1> f12(T2 &&a0) {
  return std::reference_wrapper<T1>(std::forward<T2>(a0));
}
