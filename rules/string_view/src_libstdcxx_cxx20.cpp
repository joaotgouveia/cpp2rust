#include <iosfwd>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f21(std::basic_string_view<T1, T2> a0, const T3 &a1)
  requires(
      std::is_convertible<const T3 &, std::basic_string_view<T1, T2>>::value)
{
  return std::operator==(std::forward<std::basic_string_view<T1, T2>>(a0), a1);
}
