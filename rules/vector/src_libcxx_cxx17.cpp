#include <bitset>
#include <initializer_list>
#include <iterator>
#include <utility>
#include <vector>

template <typename T1, typename... T2>
std::__bit_reference<std::vector<bool, T1>> f210(std::vector<bool, T1> &a0,
                                                 T2 &&...a1) {
  return a0.template emplace_back<T2...>(std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3>
T1 &f211(std::vector<T1, T2> &a0, T3 &&...a1) {
  return a0.template emplace_back<T3...>(std::forward<T3>(a1)...);
}
