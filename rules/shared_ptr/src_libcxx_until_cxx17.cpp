#include <atomic>
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1> T1 *f119(const std::shared_ptr<T1> &a0) {
  return a0.get();
}

template <typename T1>
typename std::add_lvalue_reference<T1>::type
f120(const std::shared_ptr<T1> &a0) {
  return a0.operator*();
}

template <typename T1> T1 *f121(const std::shared_ptr<T1> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2>
std::shared_ptr<T1> f122(const std::shared_ptr<T2> &a0, T1 *a1) {
  return std::shared_ptr<T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::is_convertible<T2 *, T1 *>::value, int> T3 = 0>
std::shared_ptr<T1> f123(std::auto_ptr<T2> &&a0) {
  return std::shared_ptr<T1>(std::move(a0));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__compatible_with_v<T2, T1> &&
                  std::is_convertible<typename std::unique_ptr<T2, T3>::pointer,
                                      T1 *>::value,
              int>
              T4 = 0>
std::shared_ptr<T1> f124(std::unique_ptr<T2, T3> &&a0) {
  return std::shared_ptr<T1>(std::move(a0));
}

template <typename T1, typename T2,
          std::__enable_if_t<!std::is_array<T2>::value &&
                                 std::is_convertible<T2 *, T1 *>::value,
                             int>
              T3 = 0>
std::shared_ptr<T1> &f125(std::shared_ptr<T1> &a0, std::auto_ptr<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__compatible_with_v<T2, T1> &&
                  std::is_convertible<typename std::unique_ptr<T2, T3>::pointer,
                                      T1 *>::value,
              int>
              T4 = 0>
std::shared_ptr<T1> &f126(std::shared_ptr<T1> &a0,
                          std::unique_ptr<T2, T3> &&a1) {
  return a0.template operator= <T2, T3>(std::move(a1));
}
