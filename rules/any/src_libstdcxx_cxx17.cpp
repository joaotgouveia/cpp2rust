#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <any>
#include <utility>

template <typename T1> T1 *f20(std::any *a0) { return std::any_cast<T1>(a0); }

template <typename T1> const T1 *f21(const std::any *a0) {
  return std::any_cast<T1>(a0);
}

using t2 = std::bad_any_cast;

const char *f22(const std::bad_any_cast &a0) { return a0.what(); }

std::bad_any_cast f23() { return std::bad_any_cast(); }

std::bad_any_cast f24(const std::bad_any_cast &a0) {
  return std::bad_any_cast(a0);
}

std::bad_any_cast f25(std::bad_any_cast &&a0) {
  return std::bad_any_cast(std::move(a0));
}

std::bad_any_cast &f26(std::bad_any_cast &a0, const std::bad_any_cast &a1) {
  return a0.operator=(a1);
}

std::bad_any_cast &f27(std::bad_any_cast &a0, std::bad_any_cast &&a1) {
  return a0.operator=(std::move(a1));
}
