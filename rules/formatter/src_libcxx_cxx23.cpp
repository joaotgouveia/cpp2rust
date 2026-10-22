#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <format>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

template <typename T1, typename T2>
void f143(std::formatter<std::basic_string_view<T1, T2>, T1> &a0) {
  return a0.set_debug_format();
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f144(std::formatter<T1, T2> &a0, T3 &a1) {
  return a0.template parse<T3>(a1);
}

template <typename T1, typename T2, typename T3>
typename T3::iterator f145(const std::formatter<T1, T2> &a0, const T1 &a1,
                           T3 &a2) {
  return a0.template format<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3>
void f146(std::formatter<std::basic_string<T1, T2, T3>, T1> &a0) {
  return a0.set_debug_format();
}
