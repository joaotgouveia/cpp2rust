#include <algorithm>
#include <condition_variable>
#include <ostream>
#include <utility>

template <typename T1, typename T2, typename T3,
          typename std::enable_if<std::is_constructible<
              std::__shared_ptr<T1>, std::unique_ptr<T2, T3>,
              std::__sp_array_delete>::value>::type *T4 = 0>
std::shared_ptr<T1> f70(std::unique_ptr<T2, T3> &&a0) {
  return std::shared_ptr<T1>(std::move(a0));
}

template <typename T1, typename std::enable_if<
                           ((std::_Lock_policy)2U == (std::_Lock_policy)2U) &&
                               (std::is_array<T1>::value == true) &&
                               (std::is_void<T1>::value == false),
                           int>::type T2 = 0>
typename std::shared_ptr<T1>::element_type &f71(const std::shared_ptr<T1> &a0) {
  return a0.operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::_Lock_policy)2U == (std::_Lock_policy)2U) &&
                               (std::is_array<T1>::value == true) &&
                               (std::is_void<T1>::value == false),
                           int>::type T2 = 0>
typename std::shared_ptr<T1>::element_type *f72(const std::shared_ptr<T1> &a0) {
  return a0.operator->();
}
