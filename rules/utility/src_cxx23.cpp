#include <algorithm>
#include <utility>

template <typename T1> std::add_const_t<T1> &f7(T1 &a0) {
  return std::as_const(a0);
}

template <typename T1, typename T2 = T1> T1 f14(T1 &a0, T2 &&a1) {
  return std::exchange(a0, std::forward<T2>(a1));
}

template <typename T1> std::underlying_type_t<T1> f37(T1 a0) {
  return std::to_underlying(std::move(a0));
}
