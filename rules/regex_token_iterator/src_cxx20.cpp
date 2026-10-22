#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <format>
#include <initializer_list>
#include <iterator>
#include <regex>
#include <utility>
#include <vector>

template <typename T1, typename T2, typename T3>
using t17 = typename std::regex_token_iterator<T1, T2, T3>::iterator_concept;

template <typename T1, typename T2, typename T3>
bool f15(const std::regex_token_iterator<T1, T2, T3> &a0,
         std::default_sentinel_t a1) {
  return a0.operator==(std::move(a1));
}
