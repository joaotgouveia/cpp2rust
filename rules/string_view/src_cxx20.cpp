#include "../common/init.h"
#include "../common/traits.h"
#include <iosfwd>
#include <string_view>
#include <utility>

template <typename T1, typename T2>
auto f18(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 <=> a1;
}

using t5 = std::u8string_view;
