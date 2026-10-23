#include <initializer_list>
#include <utility>
#include <vector>

template <typename T1, typename... T2>
typename std::vector<bool, T1>::iterator
f232(std::vector<bool, T1> &a0,
     typename std::vector<bool, T1>::const_iterator a1, T2 &&...a2) {
  return a0.template emplace<T2...>(a1, std::forward<T2>(a2)...);
}
