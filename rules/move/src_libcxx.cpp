#include <algorithm>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2> T2 f8(T1 a0, T1 a1, T2 a2) {
  return std::move<T1, T2>(a0, a1, a2);
}

template <typename T1>
typename std::conditional<!std::is_nothrow_move_constructible<T1>::value &&
                              std::is_copy_constructible<T1>::value,
                          T1 const &, T1 &&>::type
f9(T1 &a0) {
  return std::move_if_noexcept<T1>(a0);
}
