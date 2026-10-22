#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1> __remove_reference_t(T1) &&f9(T1 &&a0) {
  return std::move<T1>(std::forward<T1>(a0));
}

template <typename T1, typename T2> T2 f10(T1 a0, T1 a1, T2 a2) {
  return std::move<T1, T2>(a0, a1, a2);
}

template <typename T1>
typename std::conditional<!std::is_nothrow_move_constructible<T1>::value &&
                              std::is_copy_constructible<T1>::value,
                          T1 const &, T1 &&>::type
f11(T1 &a0) {
  return std::move_if_noexcept<T1>(a0);
}
