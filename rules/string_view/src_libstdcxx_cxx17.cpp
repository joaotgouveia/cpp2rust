#include <iosfwd>
#include <string_view>
#include <utility>

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f20(std::basic_ostream<T1, T2> &a0,
                                std::basic_string_view<T1, T2> a1) {
  return std::operator<< <T1, T2>(a0, std::move(a1));
}

using t2 = std::string_view;

using t3 = std::u16string_view;

using t4 = std::u32string_view;

using t5 = std::wstring_view;
