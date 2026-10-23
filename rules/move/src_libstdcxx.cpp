#include <algorithm>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> T1 *f2(T1 &a0) { return std::addressof<T1>(a0); }

template <typename T1> T1 &&f3(typename std::remove_reference<T1>::type &&a0) {
  return std::forward<T1>(std::move(a0));
}

template <typename T1> T1 &&f4(typename std::remove_reference<T1>::type &a0) {
  return std::forward<T1>(a0);
}

template <typename T1>
auto f5(T1 &a0) -> decltype(std::move_if_noexcept<T1>(a0)) {
  return std::move_if_noexcept<T1>(a0);
}
