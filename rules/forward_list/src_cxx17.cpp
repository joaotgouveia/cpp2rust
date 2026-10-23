#include <algorithm>
#include <forward_list>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2, typename... T3>
T1 &f61(std::forward_list<T1, T2> &a0, T3 &&...a1) {
  return a0.template emplace_front<T3...>(std::forward<T3>(a1)...);
}

template <typename T1> using t10 = std::pmr::forward_list<T1>;
