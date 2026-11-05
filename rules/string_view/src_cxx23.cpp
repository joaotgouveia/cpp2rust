#include <algorithm>
#include <iosfwd>
#include <string_view>
#include <utility>

std::basic_string_view<char> f1(const char *a0, std::size_t a1) {
  return std::operator""sv(a0, a1);
}

std::basic_string_view<wchar_t> f2(const wchar_t *a0, std::size_t a1) {
  return std::operator""sv(a0, a1);
}

std::basic_string_view<char16_t> f3(const char16_t *a0, std::size_t a1) {
  return std::operator""sv(a0, a1);
}

std::basic_string_view<char8_t> f4(const char8_t *a0, std::size_t a1) {
  return std::operator""sv(a0, a1);
}

std::basic_string_view<char32_t> f5(const char32_t *a0, std::size_t a1) {
  return std::operator""sv(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f6(std::basic_ostream<T1, T2> &a0,
                               std::basic_string_view<T1, T2> a1) {
  return std::operator<<(a0, std::forward<std::basic_string_view<T1, T2>>(a1));
}

template <typename T1, typename T2, typename T3>
auto f7(std::basic_string_view<T1, T2> a0, const T3 &a1)
  requires(
      std::is_convertible<const T3 &, std::basic_string_view<T1, T2>>::value)
{
  return std::operator<=>(std::forward<std::basic_string_view<T1, T2>>(a0), a1);
}

template <typename T1, typename T2, typename T3>
bool f8(std::basic_string_view<T1, T2> a0, const T3 &a1)
  requires(
      std::is_convertible<const T3 &, std::basic_string_view<T1, T2>>::value)
{
  return std::operator==(std::forward<std::basic_string_view<T1, T2>>(a0), a1);
}

using t1 = std::string_view;

using t2 = std::u16string_view;

using t3 = std::u32string_view;

using t4 = std::u8string_view;

using t5 = std::wstring_view;
