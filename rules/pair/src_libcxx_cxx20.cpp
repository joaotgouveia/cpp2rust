#include <array>
#include <tuple>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
auto f60(const std::pair<T1, T2> &a0, const std::pair<T3, T4> &a1) {
  return a0 <=> a1;
}
