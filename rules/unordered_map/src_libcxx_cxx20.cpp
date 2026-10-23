#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f153(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.contains(a1);
}
