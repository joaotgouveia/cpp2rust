#include "../common/init.h"
#include "../common/traits.h"
#include <iosfwd>
#include <string_view>
#include <utility>

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f1(std::basic_ostream<T1, T2> &a0,
                               std::basic_string_view<T1, T2> a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
bool f2(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 == a1;
}

using t1 = std::string_view;

using t2 = std::u16string_view;

using t3 = std::u32string_view;

using t4 = std::wstring_view;
