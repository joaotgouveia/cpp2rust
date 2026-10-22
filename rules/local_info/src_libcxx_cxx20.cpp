#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <utility>

using t1 = std::chrono::local_info;

std::chrono::local_info f1() { return std::chrono::local_info(); }

std::chrono::local_info f2(const std::chrono::local_info &a0) {
  return std::chrono::local_info(a0);
}

std::chrono::local_info f3(std::chrono::local_info &&a0) {
  return std::chrono::local_info(std::move(a0));
}

std::chrono::local_info &f4(std::chrono::local_info &a0,
                            const std::chrono::local_info &a1) {
  return a0.operator=(a1);
}

std::chrono::local_info &f5(std::chrono::local_info &a0,
                            std::chrono::local_info &&a1) {
  return a0.operator=(std::move(a1));
}

int f6() { return std::chrono::local_info::unique; }

int f7() { return std::chrono::local_info::nonexistent; }

int f8() { return std::chrono::local_info::ambiguous; }
