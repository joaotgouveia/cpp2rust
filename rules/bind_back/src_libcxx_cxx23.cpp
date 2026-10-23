#include <functional>
#include <utility>

template <typename T1, typename... T2> auto f1(T1 &&a0, T2 &&...a1) {
  return std::bind_back<T1, T2...>(std::forward<T1>(a0),
                                   std::forward<T2>(a1)...);
}
