#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <functional>
#include <utility>

using t1 = std::bad_function_call;

const char *f1(const std::bad_function_call &a0) { return a0.what(); }

std::bad_function_call f2() { return std::bad_function_call(); }

std::bad_function_call f3(const std::bad_function_call &a0) {
  return std::bad_function_call(a0);
}

std::bad_function_call &f4(std::bad_function_call &a0,
                           const std::bad_function_call &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename... T2>
bool f5(const std::function<T1(T2...)> &a0, std::nullptr_t a1) {
  return a0 == a1;
}

template <typename T1, typename... T2>
void f6(std::function<T1(T2...)> &a0, std::function<T1(T2...)> &a1) {
  return std::swap<T1, T2...>(a0, a1);
}
