#include <functional>
#include <utility>

template <typename T1> using t1 = std::_Bind<T1>;

template <typename T1, typename T2> using t2 = std::_Bind_result<T1, T2>;

template <typename T1> using t3 = std::_Mem_fn<T1>;

template <typename T1, typename... T2>
auto f1(T1 &&a0, T2 &&...a1)
    -> decltype(std::bind<T1, T2...>(std::forward<T1>(a0),
                                     std::forward<T2>(a1)...)) {
  return std::bind<T1, T2...>(std::forward<T1>(a0), std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3>
auto f2(T2 &&a0, T3 &&...a1)
    -> decltype(std::bind<T1, T2, T3...>(std::forward<T2>(a0),
                                         std::forward<T3>(a1)...)) {
  return std::bind<T1, T2, T3...>(std::forward<T2>(a0),
                                  std::forward<T3>(a1)...);
}

template <typename T1, typename T2> std::_Mem_fn<T1 T2::*> f3(T1 T2::*a0) {
  return std::mem_fn<T1, T2>(std::forward<T1 T2::*>(a0));
}
