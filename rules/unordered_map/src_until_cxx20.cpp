#include <algorithm>
#include <initializer_list>
#include <unordered_map>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f1(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
        const std::unordered_map<T1, T2, T3, T4, T5> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f2(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
        const std::unordered_multimap<T1, T2, T3, T4, T5> &a1) {
  return a0 != a1;
}
