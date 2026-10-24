#include <algorithm>
#include <condition_variable>
#include <utility>

template <
    typename T1, typename T2,
    typename T3 = typename std::enable_if<std::is_constructible<
        std::__shared_ptr<T1>, const std::shared_ptr<T2> &>::value>::type,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T4 = 0>
std::shared_ptr<T1> f59(const std::shared_ptr<T2> &a0) {
  return std::shared_ptr<T1>(a0);
}

template <
    typename T1, typename T2,
    typename T3 = typename std::enable_if<std::is_constructible<
        std::__shared_ptr<T1>, std::shared_ptr<T2>>::value>::type,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T4 = 0>
std::shared_ptr<T1> f60(std::shared_ptr<T2> &&a0) {
  return std::shared_ptr<T1>(std::move(a0));
}

template <
    typename T1, typename T2,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T3 = 0>
typename std::enable_if<std::is_assignable<std::__shared_ptr<T1> &,
                                           const std::shared_ptr<T2> &>::value,
                        std::shared_ptr<T1> &>::type
f61(std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <
    typename T1, typename T2,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T3 = 0>
typename std::enable_if<
    std::is_assignable<std::__shared_ptr<T1> &, std::shared_ptr<T2>>::value,
    std::shared_ptr<T1> &>::type
f62(std::shared_ptr<T1> &a0, std::shared_ptr<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}
