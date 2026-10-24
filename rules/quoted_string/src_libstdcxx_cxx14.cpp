#include <iomanip>
#include <utility>

template <typename T1, typename T2>
using t1 = std::__detail::_Quoted_string<T1, T2>;

template <typename T1, typename T2>
std::__detail::_Quoted_string<T1, T2> f1(T1 a0, T2 a1, T2 a2) {
  return std::__detail::_Quoted_string<T1, T2>(std::move(a0), std::move(a1),
                                               std::move(a2));
}
