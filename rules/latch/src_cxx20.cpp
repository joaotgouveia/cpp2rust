#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <cstddef>
#include <latch>
#include <utility>

using t1 = std::latch;

std::ptrdiff_t f1() { return std::latch::max(); }

std::latch f2(std::ptrdiff_t a0) { return std::latch(a0); }

void f3(std::latch &a0, std::ptrdiff_t a1) { return a0.count_down(a1); }

bool f4(const std::latch &a0) { return a0.try_wait(); }

void f5(const std::latch &a0) { return a0.wait(); }

void f6(std::latch &a0, std::ptrdiff_t a1) { return a0.arrive_and_wait(a1); }
