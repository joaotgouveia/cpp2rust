#include <algorithm>
#include <memory>
#include <ranges>

template <typename T1, typename T2>
std::ranges::in_value_result<T1, T2> f14(T1 a0, T2 a1) {
  return {a0, a1};
}

template <typename T1, typename T2>
std::ranges::out_value_result<T1, T2> f15(T1 a0, T2 a1) {
  return {a0, a1};
}

template <typename T1, typename T2>
std::ranges::elements_of<T1, T2> f17(T1 a0, T2 a1) {
  return {a0, a1};
}
