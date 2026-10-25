#include <memory>
#include <utility>

template <typename T1, typename T2, typename... T3>
using t1 = std::inout_ptr_t<T1, T2, T3...>;

template <typename T1, typename T2, typename... T3>
std::inout_ptr_t<T1, T2, T3...> f1(T1 &a0, T3... a1) {
  return std::inout_ptr_t<T1, T2, T3...>(a0, std::move(a1)...);
}

template <typename T1, typename T2, typename... T3>
T2 *f2(const std::inout_ptr_t<T1, T2, T3...> &a0) {
  return static_cast<T2 *>(a0);
}
