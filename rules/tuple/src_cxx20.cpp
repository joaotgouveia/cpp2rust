#include <algorithm>
#include <array>
#include <tuple>
#include <utility>

template <typename... T1, typename... T2>
auto f30(const std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return a0 <=> a1;
}
