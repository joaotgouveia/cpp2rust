#include <algorithm>
#include <regex>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
typename std::basic_regex<T1, T2>::flag_type f36() {
  return std::basic_regex<T1, T2>::multiline;
}
