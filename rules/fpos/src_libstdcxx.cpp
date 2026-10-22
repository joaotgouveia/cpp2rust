#include "../common/init.h"
#include "../common/traits.h"
#include <string>
#include <utility>

template <typename T1> std::fpos<T1> f9() { return std::fpos<T1>(); }

template <typename T1> std::fpos<T1> f10(const std::fpos<T1> &a0) {
  return std::fpos<T1>(a0);
}

template <typename T1>
std::fpos<T1> &f11(std::fpos<T1> &a0, const std::fpos<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::streamoff f12(const std::fpos<T1> &a0, const std::fpos<T1> &a1) {
  return a0.operator-(a1);
}
