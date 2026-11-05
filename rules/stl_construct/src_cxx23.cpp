#include <algorithm>
#include <utility>

template <typename T1, typename... T2> T1 *f1(T1 *a0, T2 &&...a1) {
  return std::construct_at(a0, std::forward<T2>(a1)...);
}

template <typename T1> void f2(T1 a0, T1 a1) {
  return std::destroy(std::move(a0), std::move(a1));
}

template <typename T1> void f3(T1 *a0) { return std::destroy_at(a0); }

template <typename T1, typename T2> T1 f4(T1 a0, T2 a1) {
  return std::destroy_n(std::move(a0), std::move(a1));
}
