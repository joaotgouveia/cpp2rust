#include <memory>
#include <utility>

template <typename T1 = void, typename T2, typename... T3>
auto f1(T2 &a0, T3 &&...a1) {
  return std::inout_ptr<T1, T2, T3...>(a0, std::forward<T3>(a1)...);
}
