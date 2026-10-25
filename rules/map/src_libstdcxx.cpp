#include <algorithm>
#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::map<T1, T2, T3, T4> f80(T5 a0, T5 a1) {
  return std::map<T1, T2, T3, T4>(std::move(a0), std::move(a1));
}
