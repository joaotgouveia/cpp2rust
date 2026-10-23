#include <initializer_list>
#include <unordered_set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
bool f127(const std::unordered_set<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.contains(a1);
}
