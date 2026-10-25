#include <tuple>
#include <utility>

using t1 = std::__ignore_type;

std::__ignore_type f1() { return std::__ignore_type(); }

std::__ignore_type f2(const std::__ignore_type &a0) {
  return std::__ignore_type(a0);
}

std::__ignore_type f3(std::__ignore_type &&a0) {
  return std::__ignore_type(std::move(a0));
}

std::__ignore_type &f4(std::__ignore_type &a0, const std::__ignore_type &a1) {
  return a0.operator=(a1);
}

std::__ignore_type &f5(std::__ignore_type &a0, std::__ignore_type &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
const std::__ignore_type &f6(const std::__ignore_type &a0, const T1 &a1) {
  return a0.template operator= <T1>(a1);
}
