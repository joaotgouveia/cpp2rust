#include <algorithm>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> f29(T3 &&a0, T4 &&a1)
  requires((std::pair<T1, T2>::template _S_constructible<T3, T4>()) &&
           (!std::pair<T1, T2>::template _S_dangles<T3, T4>())) &&
          (!(std::is_same<T3, const std::type_identity_t<T1> &>::value &&
             std::is_same<T4, const T2 &>::value))
{
  return std::pair<T1, T2>(std::forward<T3>(a0), std::forward<T4>(a1));
}
