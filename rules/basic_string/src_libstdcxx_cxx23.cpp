#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <iosfwd>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f245(const std::basic_string<T1, T2, T3> &a0, T1 a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3>
bool f246(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> f247(std::from_range_t a0, T4 &&a1,
                                   const T3 &a2) {
  return std::basic_string<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2);
}
