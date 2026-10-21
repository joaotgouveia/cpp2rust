#include "../common/init.h"
#include "../common/traits.h"
#include <functional>
#include <utility>

<<<<<<< HEAD
template <typename T1, typename... T2>
auto f1(T1 &&a0, T2 &&...a1)
    -> decltype(std::bind<T1, T2...>(std::forward<T1>(a0),
                                     std::forward<T2>(a1)...)) {
  return std::bind<T1, T2...>(std::forward<T1>(a0), std::forward<T2>(a1)...);
=======
template <typename T1> using t1 = std::reference_wrapper<T1>;

template <typename T1> using t2 = typename std::reference_wrapper<T1>::type;

template <typename T1> std::reference_wrapper<T1> f1(T1 &a0) {
  return std::reference_wrapper<T1>(a0);
>>>>>>> master
}

template <typename T1, typename T2, typename... T3>
auto f2(T2 &&a0, T3 &&...a1)
    -> decltype(std::bind<T1, T2, T3...>(std::forward<T2>(a0),
                                         std::forward<T3>(a1)...)) {
  return std::bind<T1, T2, T3...>(std::forward<T2>(a0),
                                  std::forward<T3>(a1)...);
}

template <typename T1, typename T2>
auto f3(T1 T2::*a0) -> decltype(std::mem_fn<T1, T2>(a0)) {
  return std::mem_fn<T1, T2>(a0);
}
