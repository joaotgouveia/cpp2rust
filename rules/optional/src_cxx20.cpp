#include <algorithm>
#include <compare>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1>
std::strong_ordering f30(const std::optional<T1> &a0, std::nullopt_t a1) {
  return std::operator<=> <T1>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2> f31(const std::optional<T1> &a0,
                                            const std::optional<T2> &a1) {
  return std::operator<=> <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2> f32(const std::optional<T1> &a0,
                                            const T2 &a1) {
  return std::operator<=> <T1, T2>(a0, a1);
}
