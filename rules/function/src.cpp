#include <algorithm>
#include <cstddef>
#include <cxxabi.h>
#include <exception>
#include <functional>
#include <memory>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename... T2> using t1 = std::function<T1(T2...)>;

template <typename T1, typename... T2>
using t2 = typename std::function<T1(T2...)>::result_type;

template <typename T1, typename... T2> std::function<T1(T2...)> f1() {
  return std::function<T1(T2...)>();
}

template <typename T1, typename... T2>
std::function<T1(T2...)> f2(std::nullptr_t a0) {
  return std::function<T1(T2...)>(a0);
}

template <typename T1, typename... T2>
std::function<T1(T2...)> f3(const std::function<T1(T2...)> &a0) {
  return std::function<T1(T2...)>(a0);
}

template <typename T1, typename... T2>
std::function<T1(T2...)> f4(std::function<T1(T2...)> &&a0) {
  return std::function<T1(T2...)>(std::move(a0));
}

template <typename T1, typename... T2>
std::function<T1(T2...)> &f5(std::function<T1(T2...)> &a0,
                             const std::function<T1(T2...)> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename... T2>
std::function<T1(T2...)> &f6(std::function<T1(T2...)> &a0,
                             std::function<T1(T2...)> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename... T2>
std::function<T1(T2...)> &f7(std::function<T1(T2...)> &a0, std::nullptr_t a1) {
  return a0.operator=(a1);
}

template <typename T1, typename... T2>
void f8(std::function<T1(T2...)> &a0, std::function<T1(T2...)> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename... T2>
bool f9(const std::function<T1(T2...)> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename... T2>
T1 f10(const std::function<T1(T2...)> &a0, T2... a1) {
  return a0.operator()(a1...);
}

template <typename T1, typename... T2>
const std::type_info &f11(const std::function<T1(T2...)> &a0) {
  return a0.target_type();
}

template <typename T1, typename... T2, typename T3>
T3 *f12(std::function<T1(T2...)> &a0) {
  return a0.template target<T3>();
}

template <typename T1, typename... T2, typename T3>
const T3 *f13(const std::function<T1(T2...)> &a0) {
  return a0.template target<T3>();
}
