#include <algorithm>
#include <condition_variable>
#include <utility>

template <
    typename T1, std::_Lock_policy T2,
    typename std::enable_if<(T2 == T2) && (std::is_array<T1>::value == true) &&
                                (std::is_void<T1>::value == false),
                            int>::type T3 = 0>
typename std::__shared_ptr<T1, T2>::element_type &
f62(const std::__shared_ptr<T1, T2> &a0) {
  return a0.operator*();
}

template <
    typename T1, std::_Lock_policy T2,
    typename std::enable_if<(T2 == T2) && (std::is_array<T1>::value == true) &&
                                (std::is_void<T1>::value == false),
                            int>::type T3 = 0>
typename std::__shared_ptr<T1, T2>::element_type *
f63(const std::__shared_ptr<T1, T2> &a0) {
  return a0.operator->();
}
