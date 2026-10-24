#include <algorithm>
#include <condition_variable>
#include <utility>

template <
    typename T1, std::_Lock_policy T2, typename T3,
    typename T4 = typename std::enable_if<
        std::__sp_compatible_with<T3 *, T1 *>::value, void>::type,
    typename std::enable_if<(!(std::is_same<T3, T1>::value)), int>::type T5 = 0>
std::__weak_ptr<T1, T2> f78(const std::__weak_ptr<T3, T2> &a0) {
  return std::__weak_ptr<T1, T2>(a0);
}

template <
    typename T1, std::_Lock_policy T2, typename T3,
    typename T4 = typename std::enable_if<
        std::__sp_compatible_with<T3 *, T1 *>::value, void>::type,
    typename std::enable_if<(!(std::is_same<T3, T1>::value)), int>::type T5 = 0>
std::__weak_ptr<T1, T2> f79(std::__weak_ptr<T3, T2> &&a0) {
  return std::__weak_ptr<T1, T2>(std::move(a0));
}

template <
    typename T1, std::_Lock_policy T2, typename T3,
    typename std::enable_if<(!(std::is_same<T3, T1>::value)), int>::type T4 = 0>
typename std::enable_if<std::__sp_compatible_with<T3 *, T1 *>::value,
                        std::__weak_ptr<T1, T2> &>::type
f80(std::__weak_ptr<T1, T2> &a0, const std::__weak_ptr<T3, T2> &a1) {
  return a0.template operator= <T3>(a1);
}

template <
    typename T1, std::_Lock_policy T2, typename T3,
    typename std::enable_if<(!(std::is_same<T3, T1>::value)), int>::type T4 = 0>
typename std::enable_if<std::__sp_compatible_with<T3 *, T1 *>::value,
                        std::__weak_ptr<T1, T2> &>::type
f81(std::__weak_ptr<T1, T2> &a0, std::__weak_ptr<T3, T2> &&a1) {
  return a0.template operator= <T3>(std::move(a1));
}
