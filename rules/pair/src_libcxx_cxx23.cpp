#include <tuple>
#include <utility>

template <typename T1, typename T2>
void f72(const std::pair<T1, T2> &a0, const std::pair<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::__check_pair_construction<T1, T2>::
                                 template __is_pair_constructible<T3 &, T4 &>(),
                             int>
              T5 = 0>
std::pair<T1, T2> f73(std::pair<T3, T4> &a0) {
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__check_pair_construction<T1, T2>::
                  template __is_pair_constructible<const T3 &&, const T4 &&>(),
              int>
              T5 = 0>
std::pair<T1, T2> f74(const std::pair<T3, T4> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3 = void>
const std::pair<T1, T2> &f75(const std::pair<T1, T2> &a0,
                             const std::pair<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3 = void>
const std::pair<T1, T2> &f76(const std::pair<T1, T2> &a0,
                             std::pair<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}
