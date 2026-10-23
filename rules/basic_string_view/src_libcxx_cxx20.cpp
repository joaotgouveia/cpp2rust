#include <cstddef>
#include <iterator>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string_view<T1, T2> f128(T3 a0, T4 a1)
  requires((std::is_same_v<std::iter_value_t<T3>, T1> &&
            !std::is_convertible_v<
                T4, typename std::basic_string_view<T1, T2>::size_type>))
{
  return std::basic_string_view<T1, T2>(a0, a1);
}
