#include <functional>
#include <utility>

template <typename T1> using t1 = std::__mem_fn<T1>;

template <typename T1> using t2 = typename std::__mem_fn<T1>::type;

template <typename T1> std::__mem_fn<T1> f1(T1 a0) {
  return std::__mem_fn<T1>(std::move(a0));
}

template <typename T1, typename... T2>
auto f2(const std::__mem_fn<T1> &a0, T2 &&...a1)
    -> decltype(a0.template operator()<T2...>(std::forward<T2>(a1)...)) {
  return a0.template operator()<T2...>(std::forward<T2>(a1)...);
}

template <typename T1, typename T2> std::__mem_fn<T1 T2::*> f3(T1 T2::*a0) {
  return std::mem_fn<T1, T2>(std::forward<T1 T2::*>(a0));
}
