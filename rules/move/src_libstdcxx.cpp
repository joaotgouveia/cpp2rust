#include <algorithm>
#include <utility>

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

template <typename T1>
typename std::enable_if<std::__and_<std::__not_<std::__is_tuple_like<T1>>,
                                    std::is_move_constructible<T1>,
                                    std::is_move_assignable<T1>>::value>::type
f6(T1 &a0, T1 &a1) {
  return std::swap<T1>(a0, a1);
}

template <typename T1, std::size_t T2>
typename std::enable_if<std::__is_swappable<T1>::value>::type f7(T1 (&a0)[T2],
                                                                 T1 (&a1)[T2]) {
  return std::swap<T1, T2>(a0, a1);
}
