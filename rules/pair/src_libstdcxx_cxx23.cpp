#include <algorithm>
#include <utility>

template <typename T1, typename T2>
const std::pair<T1, T2> &f22(const std::pair<T1, T2> &a0,
                             const std::pair<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
const std::pair<T1, T2> &f23(const std::pair<T1, T2> &a0,
                             std::pair<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> f24(std::pair<T3, T4> &a0) {
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> f25(const std::pair<T3, T4> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}
