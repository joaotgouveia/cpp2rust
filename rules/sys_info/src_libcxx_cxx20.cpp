#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <utility>

using t1 = std::chrono::sys_info;

std::chrono::sys_info f1() { return std::chrono::sys_info(); }

std::chrono::sys_info f2(const std::chrono::sys_info &a0) {
  return std::chrono::sys_info(a0);
}

std::chrono::sys_info f3(std::chrono::sys_info &&a0) {
  return std::chrono::sys_info(std::move(a0));
}

std::chrono::sys_info &f4(std::chrono::sys_info &a0,
                          const std::chrono::sys_info &a1) {
  return a0.operator=(a1);
}

std::chrono::sys_info &f5(std::chrono::sys_info &a0,
                          std::chrono::sys_info &&a1) {
  return a0.operator=(std::move(a1));
}
