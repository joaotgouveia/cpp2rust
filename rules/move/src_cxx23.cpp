#include <algorithm>
#include <utility>

template <typename T1> T1 *f1(T1 &a0) { return std::addressof<T1>(a0); }

template <typename T1> T1 &&f2(typename std::remove_reference<T1>::type &&a0) {
  return std::forward<T1>(std::move(a0));
}

template <typename T1> T1 &&f3(typename std::remove_reference<T1>::type &a0) {
  return std::forward<T1>(a0);
}

template <typename T1, typename T2> auto f4(T2 &&a0) {
  return std::forward_like<T1, T2>(std::forward<T2>(a0));
}

template <typename T1> typename std::remove_reference<T1>::type &&f5(T1 &&a0) {
  return std::move<T1>(std::forward<T1>(a0));
}

template <typename T1> auto f6(T1 &a0) { return std::move_if_noexcept<T1>(a0); }

template <typename T1> void f7(T1 &a0, T1 &a1) { return std::swap<T1>(a0, a1); }

template <typename T1, std::size_t T2> void f8(T1 (&a0)[T2], T1 (&a1)[T2]) {
  return std::swap<T1, T2>(a0, a1);
}
