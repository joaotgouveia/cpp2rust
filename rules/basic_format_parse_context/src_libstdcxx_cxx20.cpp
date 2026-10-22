#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <format>
#include <string_view>
#include <utility>

template <typename T1>
std::basic_format_parse_context<T1> f7(std::basic_string_view<T1> a0) {
  return std::basic_format_parse_context<T1>(a0);
}
