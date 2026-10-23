#include <string_view>
#include <utility>

template <typename T1, typename T2, int T3 = 1>
bool f36(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator== <T1, T2>(a0, a1);
}

using t6 = std::string_view;

using t7 = std::u16string_view;

using t8 = std::u32string_view;

using t9 = std::wstring_view;
