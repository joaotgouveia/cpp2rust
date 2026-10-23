#include <cstddef>
#include <memory_resource>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t1 = std::pmr::monotonic_buffer_resource;

std::pmr::monotonic_buffer_resource f1() { return {}; }

std::pmr::monotonic_buffer_resource f2(std::size_t a0) {
  return std::pmr::monotonic_buffer_resource(a0);
}

std::pmr::monotonic_buffer_resource f3(void *a0, std::size_t a1) {
  return {a0, a1};
}

std::pmr::monotonic_buffer_resource f4(std::pmr::memory_resource *a0) {
  return std::pmr::monotonic_buffer_resource(a0);
}

std::pmr::monotonic_buffer_resource f5(std::size_t a0,
                                       std::pmr::memory_resource *a1) {
  return {a0, a1};
}

std::pmr::monotonic_buffer_resource f6(void *a0, std::size_t a1,
                                       std::pmr::memory_resource *a2) {
  return {a0, a1, a2};
}

void f7(std::pmr::monotonic_buffer_resource &a0) { return a0.release(); }

std::pmr::memory_resource *f8(const std::pmr::monotonic_buffer_resource &a0) {
  return a0.upstream_resource();
}

void *f9(std::pmr::monotonic_buffer_resource &a0, std::size_t a1,
         std::size_t a2) {
  return a0.allocate(a1, a2);
}

void f10(std::pmr::monotonic_buffer_resource &a0, void *a1, std::size_t a2,
         std::size_t a3) {
  return a0.deallocate(a1, a2, a3);
}
