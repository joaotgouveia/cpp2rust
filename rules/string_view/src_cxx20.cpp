#include <iosfwd>
#include <string_view>
#include <utility>

template <typename T1, typename T2>
auto f1(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 <=> a1;
}

using t1 = std::u8string_view;
