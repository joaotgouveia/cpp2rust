#include <algorithm>
#include <utility>
#include <vector>

template <typename T1> using t41 = std::pmr::vector<T1>;

template <typename T1, typename... T2>
std::_Bit_reference f122(std::vector<bool, T1> &a0, T2 &&...a1) {
  return a0.template emplace_back<T2...>(std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3>
typename std::vector<T1, T2>::reference f123(std::vector<T1, T2> &a0,
                                             T3 &&...a1) {
  return a0.template emplace_back<T3...>(std::forward<T3>(a1)...);
}
