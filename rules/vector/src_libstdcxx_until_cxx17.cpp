#include <algorithm>
#include <utility>
#include <vector>

template <typename T1, typename... T2>
void f162(std::vector<bool, T1> &a0, T2 &&...a1) {
  return a0.template emplace_back<T2...>(std::forward<T2>(a1)...);
}
