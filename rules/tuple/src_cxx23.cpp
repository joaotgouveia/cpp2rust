#include <algorithm>
#include <array>
#include <tuple>
#include <utility>

void f31(const std::tuple<> &a0, const std::tuple<> &a1) { return a0.swap(a1); }

template <typename... T1>
const std::tuple<T1...> &f32(const std::tuple<T1...> &a0,
                             const std::tuple<T1...> &a1) {
  return a0.operator=(a1);
}

template <typename... T1>
const std::tuple<T1...> &f33(const std::tuple<T1...> &a0,
                             std::tuple<T1...> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename... T1>
void f34(const std::tuple<T1...> &a0, const std::tuple<T1...> &a1) {
  return a0.swap(a1);
}
