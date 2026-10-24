#include <algorithm>
#include <format>
#include <iterator>

template <typename T1, typename T2>
std::ranges::in_fun_result<T1, T2> f6(T1 a0, T2 a1) {
  return {a0, a1};
}

template <typename T1, typename T2>
std::ranges::in_in_result<T1, T2> f7(T1 a0, T2 a1) {
  return {a0, a1};
}

template <typename T1, typename T2>
std::ranges::in_out_result<T1, T2> f8(T1 a0, T2 a1) {
  return {a0, a1};
}

template <typename T1, typename T2, typename T3>
std::ranges::in_in_out_result<T1, T2, T3> f9(T1 a0, T2 a1, T3 a2) {
  return {a0, a1, a2};
}

template <typename T1, typename T2, typename T3>
std::ranges::in_out_out_result<T1, T2, T3> f10(T1 a0, T2 a1, T3 a2) {
  return {a0, a1, a2};
}

template <typename T1> std::ranges::min_max_result<T1> f11(T1 a0, T1 a1) {
  return {a0, a1};
}

template <typename T1> std::ranges::in_found_result<T1> f12(T1 a0, bool a1) {
  return {a0, a1};
}

template <typename T1>
std::format_to_n_result<T1> f13(T1 a0, std::iter_difference_t<T1> a1) {
  return {a0, a1};
}
