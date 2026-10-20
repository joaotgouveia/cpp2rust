#include "../common/init.h"
#include "../common/traits.h"
#include <initializer_list>
#include <regex>
#include <utility>
#include <vector>

template <typename T1, typename T2, typename T3>
bool f14(const std::regex_token_iterator<T1, T2, T3> &a0,
         const std::regex_token_iterator<T1, T2, T3> &a1) {
  return a0.operator!=(a1);
}
