#include <algorithm>
#include <tuple>
#include <utility>

template <typename... T1, typename T2>
std::tuple<T1...> f69(std::allocator_arg_t a0, const T2 &a1,
                      const std::tuple<T1...> &a2) {
  return std::tuple<T1...>(std::move(a0), a1, a2);
}
