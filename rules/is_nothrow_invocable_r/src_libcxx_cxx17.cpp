#include <type_traits>
#include <utility>

template <typename T1, typename T2, typename... T3>
bool f1(const std::is_nothrow_invocable_r<T1, T2, T3...> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename... T3>
bool f2(const std::is_nothrow_invocable_r<T1, T2, T3...> &a0) {
  return a0.operator()();
}
