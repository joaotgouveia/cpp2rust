#include <cstddef>
#include <cxxabi.h>
#include <functional>
#include <memory>
#include <utility>

template <typename T1, typename... T2>
bool f18(const std::function<T1(T2...)> &a0, std::nullptr_t a1) {
  return std::operator== <T1, T2...>(a0, a1);
}

template <typename T1, typename... T2>
void f19(std::function<T1(T2...)> &a0, std::function<T1(T2...)> &a1) {
  return std::swap<T1, T2...>(a0, a1);
}

using t3 = std::bad_function_call;

std::bad_function_call f20() { return std::bad_function_call(); }

std::bad_function_call f21(const std::bad_function_call &a0) {
  return std::bad_function_call(a0);
}

std::bad_function_call &f22(std::bad_function_call &a0,
                            const std::bad_function_call &a1) {
  return a0.operator=(a1);
}

const char *f23(const std::bad_function_call &a0) { return a0.what(); }
