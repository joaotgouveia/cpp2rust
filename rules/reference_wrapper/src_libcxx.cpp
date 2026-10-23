#include <functional>
#include <utility>

template <typename T1> std::reference_wrapper<const T1> f5(const T1 &a0) {
  return std::cref<T1>(a0);
}

template <typename T1>
std::reference_wrapper<const T1> f6(std::reference_wrapper<T1> a0) {
  return std::cref<T1>(a0);
}

template <typename T1> std::reference_wrapper<T1> f7(T1 &a0) {
  return std::ref<T1>(a0);
}

template <typename T1>
std::reference_wrapper<T1> f8(std::reference_wrapper<T1> a0) {
  return std::ref<T1>(a0);
}

template <typename T1, typename T2, typename T3 = void,
          typename std::enable_if<
              !std::is_same<typename std::remove_cv<
                                typename std::remove_reference<T2>::type>::type,
                            std::reference_wrapper<T1>>::value,
              int>::type T4 = 0>
std::reference_wrapper<T1> f9(T2 &&a0) {
  return std::reference_wrapper<T1>(std::forward<T2>(a0));
}
