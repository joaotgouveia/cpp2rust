#include <algorithm>
#include <tuple>
#include <utility>

template <typename T1, typename T2>
void f5(const std::pair<T1, T2> &a0, const std::pair<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3> std::pair<T1, T2> f6(T3 &&a0) {
  return std::pair<T1, T2>(std::forward<T3>(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
const std::pair<T1, T2> &f7(const std::pair<T1, T2> &a0,
                            const std::pair<T3, T4> &a1) {
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
const std::pair<T1, T2> &f8(const std::pair<T1, T2> &a0,
                            std::pair<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::pair<T1, T2> &f9(std::pair<T1, T2> &a0, T3 &&a1) {
  return a0.template operator= <T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
const std::pair<T1, T2> &f10(const std::pair<T1, T2> &a0, T3 &&a1) {
  return a0.template operator= <T3>(std::forward<T3>(a1));
}
