#include "../common/init.h"
#include "../common/traits.h"
#include <format>
#include <string_view>
#include <utility>

template <typename T1, typename... T2>
using t1 = std::basic_format_string<T1, T2...>;

template <typename T1, typename... T2>
std::basic_string_view<T1> f1(const std::basic_format_string<T1, T2...> &a0) {
  return a0.get();
}
