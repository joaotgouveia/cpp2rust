#include <algorithm>
#include <condition_variable>
#include <utility>

template <typename T1, typename T2, typename... T3>
std::shared_ptr<std::_NonArray<T1>> f41(const T2 &a0, T3 &&...a1) {
  return std::allocate_shared<T1, T2, T3...>(a0, std::forward<T3>(a1)...);
}

template <typename T1, typename... T2>
std::shared_ptr<std::_NonArray<T1>> f42(T2 &&...a0) {
  return std::make_shared<T1, T2...>(std::forward<T2>(a0)...);
}

template <
    typename T1, typename T2,
    typename T3 = typename std::enable_if<std::is_constructible<
        std::__shared_ptr<T1>, const std::shared_ptr<T2> &>::value>::type,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T4 = 0>
std::shared_ptr<T1> f43(const std::shared_ptr<T2> &a0) {
  return std::shared_ptr<T1>(a0);
}

template <
    typename T1, typename T2,
    typename T3 = typename std::enable_if<std::is_constructible<
        std::__shared_ptr<T1>, std::shared_ptr<T2>>::value>::type,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T4 = 0>
std::shared_ptr<T1> f44(std::shared_ptr<T2> &&a0) {
  return std::shared_ptr<T1>(std::move(a0));
}

template <
    typename T1, typename T2,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T3 = 0>
typename std::enable_if<std::is_assignable<std::__shared_ptr<T1> &,
                                           const std::shared_ptr<T2> &>::value,
                        std::shared_ptr<T1> &>::type
f45(std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <
    typename T1, typename T2,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T3 = 0>
typename std::enable_if<
    std::is_assignable<std::__shared_ptr<T1> &, std::shared_ptr<T2>>::value,
    std::shared_ptr<T1> &>::type
f46(std::shared_ptr<T1> &a0, std::shared_ptr<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}
