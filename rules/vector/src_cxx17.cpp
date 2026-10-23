#include <algorithm>
#include <initializer_list>
#include <utility>
#include <vector>

template <typename T1, typename... T2>
typename std::vector<bool, T1>::reference f76(std::vector<bool, T1> &a0,
                                              T2 &&...a1) {
  return a0.template emplace_back<T2...>(std::forward<T2>(a1)...);
}
