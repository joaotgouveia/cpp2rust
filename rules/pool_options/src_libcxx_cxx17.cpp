#include <memory_resource>
#include <utility>

using t1 = std::pmr::pool_options;

std::pmr::pool_options f1() { return std::pmr::pool_options(); }

std::pmr::pool_options f2(const std::pmr::pool_options &a0) {
  return std::pmr::pool_options(a0);
}

std::pmr::pool_options f3(std::pmr::pool_options &&a0) {
  return std::pmr::pool_options(std::move(a0));
}

std::pmr::pool_options &f4(std::pmr::pool_options &a0,
                           const std::pmr::pool_options &a1) {
  return a0.operator=(a1);
}

std::pmr::pool_options &f5(std::pmr::pool_options &a0,
                           std::pmr::pool_options &&a1) {
  return a0.operator=(std::move(a1));
}
