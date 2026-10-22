#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <any>
#include <initializer_list>
#include <type_traits>
#include <utility>

using t1 = std::any;

std::any f1() { return std::any(); }

std::any f2(const std::any &a0) { return std::any(a0); }

std::any f3(std::any &&a0) { return std::any(std::move(a0)); }

std::any &f4(std::any &a0, const std::any &a1) { return a0.operator=(a1); }

std::any &f5(std::any &a0, std::any &&a1) {
  return a0.operator=(std::move(a1));
}

void f6(std::any &a0) { return a0.reset(); }

void f7(std::any &a0, std::any &a1) { return a0.swap(a1); }

bool f8(const std::any &a0) { return a0.has_value(); }

const std::type_info &f9(const std::any &a0) { return a0.type(); }

template <typename T1> T1 f10(std::any &a0) { return std::any_cast<T1>(a0); }

template <typename T1> T1 f11(std::any &&a0) {
  return std::any_cast<T1>(std::move(a0));
}

template <typename T1> T1 f12(const std::any &a0) {
  return std::any_cast<T1>(a0);
}

void f13(std::any &a0, std::any &a1) { return std::swap(a0, a1); }
