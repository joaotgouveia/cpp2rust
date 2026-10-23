#include <utility>

using t1 = std::piecewise_construct_t;

std::piecewise_construct_t f1() { return std::piecewise_construct_t(); }

std::piecewise_construct_t f2(const std::piecewise_construct_t &a0) {
  return std::piecewise_construct_t(a0);
}

std::piecewise_construct_t f3(std::piecewise_construct_t &&a0) {
  return std::piecewise_construct_t(std::move(a0));
}

std::piecewise_construct_t &f4(std::piecewise_construct_t &a0,
                               const std::piecewise_construct_t &a1) {
  return a0.operator=(a1);
}

std::piecewise_construct_t &f5(std::piecewise_construct_t &a0,
                               std::piecewise_construct_t &&a1) {
  return a0.operator=(std::move(a1));
}
