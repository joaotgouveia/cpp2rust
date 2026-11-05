#include <algorithm>
#include <functional>
#include <utility>

template <typename T1, typename... T2>
bool f5(const std::function<T1(T2...)> &a0, std::nullptr_t a1) {
  return std::operator==(a0, a1);
}

template <typename T1, typename... T2>
void f6(std::function<T1(T2...)> &a0, std::function<T1(T2...)> &a1) {
  return std::swap(a0, a1);
}
