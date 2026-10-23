#include <tuple>
#include <utility>

template <typename... T1>
void f35(std::tuple<T1...> &a0, std::tuple<T1...> &a1) {
  return std::swap<T1...>(a0, a1);
}
