#include <array>
#include <tuple>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::__check_pair_construction<T1, T2>::
                                 template __is_pair_constructible<T3, T4>(),
                             int>
              T5 = 0>
std::pair<T1, T2> f85(T3 &&a0, T4 &&a1)
  requires(!(std::is_same<T3, const T1 &>::value &&
             std::is_same<T4, const T2 &>::value))
{
  return std::pair<T1, T2>(std::forward<T3>(a0), std::forward<T4>(a1));
}
