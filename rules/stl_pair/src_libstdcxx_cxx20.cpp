#include <algorithm>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
auto f27(const std::pair<T1, T2> &a0, const std::pair<T3, T4> &a1) {
  return std::operator<=> <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f28(const std::pair<T1, T2> &a0, const std::pair<T3, T4> &a1) {
  return std::operator== <T1, T2, T3, T4>(a0, a1);
}
