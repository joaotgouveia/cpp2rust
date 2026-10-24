#include <algorithm>
#include <bitset>
#include <initializer_list>
#include <iterator>
#include <utility>
#include <vector>

template <typename T1, typename... T2>
std::__bit_iterator<std::vector<bool, T1>, false>
f217(std::vector<bool, T1> &a0,
     std::__bit_iterator<std::vector<bool, T1>, true> a1, T2 &&...a2) {
  return a0.template emplace<T2...>(a1, std::forward<T2>(a2)...);
}
