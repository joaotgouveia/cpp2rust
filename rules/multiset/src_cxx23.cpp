#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <ranges>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
void f45(std::multiset<T1, T2, T3> &a0, T4 &&a1) {
  return a0.template insert_range<T4>(std::forward<T4>(a1));
}
