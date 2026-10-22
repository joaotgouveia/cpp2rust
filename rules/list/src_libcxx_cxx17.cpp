#include "../common/init.h"
#include "../common/traits.h"
#include <initializer_list>
#include <list>
#include <utility>

template <typename T1, typename T2, typename... T3>
typename std::list<T1, T2>::value_type &f102(std::list<T1, T2> &a0,
                                             T3 &&...a1) {
  return a0.template emplace_front<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename... T3>
typename std::list<T1, T2>::value_type &f103(std::list<T1, T2> &a0,
                                             T3 &&...a1) {
  return a0.template emplace_back<T3...>(std::forward<T3>(a1)...);
}
