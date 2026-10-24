#include <algorithm>
#include <bitset>
#include <initializer_list>
#include <iterator>
#include <utility>
#include <vector>

template <typename T1, typename T2, typename... T3>
void f58(std::vector<T1, T2> &a0, T3 &&...a1) {
  return a0.template emplace_back<T3...>(std::forward<T3>(a1)...);
}
