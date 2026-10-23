#include <algorithm>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3,
          typename T4 = std::remove_cvref_t<T3>>
std::basic_string_view<T1, T2> f69(T3 &&a0)
  requires(!(std::is_same<T3, const std::basic_string_view<T1, T2> &>::value) &&
           !(std::is_same<T3, const T1 *>::value) &&
           !(std::is_same<T3, std::nullptr_t>::value))
{
  return std::basic_string_view<T1, T2>(std::forward<T3>(a0));
}
