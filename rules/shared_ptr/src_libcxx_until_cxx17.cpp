#include <atomic>
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1>
typename std::add_lvalue_reference<T1>::type
f86(const std::shared_ptr<T1> &a0) {
  return a0.operator*();
}

template <typename T1> bool f87(const std::shared_ptr<T1> &a0) {
  return a0.unique();
}

template <typename T1, typename T2,
          typename std::enable_if<std::is_convertible<T2 *, T1 *>::value,
                                  int>::type T3 = 0>
std::shared_ptr<T1> f88(std::auto_ptr<T2> &&a0) {
  return std::shared_ptr<T1>(std::move(a0));
}

template <typename T1, typename T2,
          typename std::enable_if<!std::is_array<T2>::value &&
                                      std::is_convertible<T2 *, T1 *>::value,
                                  int>::type T3 = 0>
std::shared_ptr<T1> &f89(std::shared_ptr<T1> &a0, std::auto_ptr<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}
