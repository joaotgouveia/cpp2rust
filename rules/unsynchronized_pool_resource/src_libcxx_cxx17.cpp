#include "../common/init.h"
#include "../common/traits.h"
#include <cstddef>
#include <memory_resource>
#include <utility>

using t1 = std::pmr::unsynchronized_pool_resource;

std::pmr::unsynchronized_pool_resource f1(const std::pmr::pool_options &a0,
                                          std::pmr::memory_resource *a1) {
  return {a0, a1};
}

std::pmr::unsynchronized_pool_resource f2() { return {}; }

std::pmr::unsynchronized_pool_resource f3(std::pmr::memory_resource *a0) {
  return std::pmr::unsynchronized_pool_resource(a0);
}

std::pmr::unsynchronized_pool_resource f4(const std::pmr::pool_options &a0) {
  return std::pmr::unsynchronized_pool_resource(a0);
}

void f5(std::pmr::unsynchronized_pool_resource &a0) { return a0.release(); }

std::pmr::memory_resource *
f6(const std::pmr::unsynchronized_pool_resource &a0) {
  return a0.upstream_resource();
}

std::pmr::pool_options f7(const std::pmr::unsynchronized_pool_resource &a0) {
  return a0.options();
}

void *f8(std::pmr::unsynchronized_pool_resource &a0, std::size_t a1,
         std::size_t a2) {
  return a0.allocate(a1, a2);
}

void f9(std::pmr::unsynchronized_pool_resource &a0, void *a1, std::size_t a2,
        std::size_t a3) {
  return a0.deallocate(a1, a2, a3);
}
