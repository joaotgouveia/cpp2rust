#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>
#include <valarray>

using t1 = std::gslice;

std::gslice f1() { return std::gslice(); }

std::gslice f2(std::size_t a0, const std::valarray<std::size_t> &a1,
               const std::valarray<std::size_t> &a2) {
  return std::gslice(a0, a1, a2);
}

std::gslice f3(const std::gslice &a0) { return std::gslice(a0); }

std::gslice &f4(std::gslice &a0, const std::gslice &a1) {
  return a0.operator=(a1);
}

std::size_t f5(const std::gslice &a0) { return a0.start(); }

std::valarray<std::size_t> f6(const std::gslice &a0) { return a0.size(); }

std::valarray<std::size_t> f7(const std::gslice &a0) { return a0.stride(); }
