#include <regex>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
std::match_results<T1, T2>
f29(const typename std::match_results<T1, T2>::allocator_type &a0) {
  return std::match_results<T1, T2>(a0);
}
