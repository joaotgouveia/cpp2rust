#include "../common/init.h"
#include "../common/traits.h"
#include <ios>
#include <iosfwd>
#include <utility>

template <typename T1>
bool f13(const std::fpos<T1> &a0, const std::fpos<T1> &a1) {
  return a0 != a1;
}

template <typename T1>
bool f14(const std::fpos<T1> &a0, const std::fpos<T1> &a1) {
  return a0 == a1;
}

template <typename T1>
std::streamoff f15(const std::fpos<T1> &a0, const std::fpos<T1> &a1) {
  return a0 - a1;
}
