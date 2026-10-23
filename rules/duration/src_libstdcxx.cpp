#include <chrono>
#include <utility>

template <typename T1, typename T2>
std::chrono::duration<T1, T2> f14(const std::chrono::duration<T1, T2> &a0) {
  return std::chrono::duration<T1, T2>(a0);
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> &f15(std::chrono::duration<T1, T2> &a0,
                                   const std::chrono::duration<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::chrono::duration<typename std::common_type<T1>::type,
                      typename std::chrono::duration<T1, T2>::period>
f16(const std::chrono::duration<T1, T2> &a0) {
  return a0.operator+();
}

template <typename T1, typename T2>
std::chrono::duration<typename std::common_type<T1>::type,
                      typename std::chrono::duration<T1, T2>::period>
f17(const std::chrono::duration<T1, T2> &a0) {
  return a0.operator-();
}

template <
    typename T1, typename T2, typename T3,
    typename T4 = std::_Require<
        std::is_convertible<const T3 &, T1>,
        std::__or_<std::chrono::treat_as_floating_point<T1>,
                   std::__not_<std::chrono::treat_as_floating_point<T3>>>>>
std::chrono::duration<T1, T2> f18(const T3 &a0) {
  return std::chrono::duration<T1, T2>(a0);
}
