#include <tuple>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename... T1>
void f35(std::tuple<T1...> &a0, std::tuple<T1...> &a1) {
  return std::swap<T1...>(a0, a1);
}

template <typename... T1, typename T2>
std::tuple<T1...> f36(std::allocator_arg_t a0, const T2 &a1,
                      const std::tuple<T1...> &a2) {
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <typename... T1, typename T2>
std::tuple<T1...> f37(std::allocator_arg_t a0, const T2 &a1,
                      std::tuple<T1...> &&a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}
