#include <algorithm>
#include <cstddef>
#include <memory_resource>
#include <string>
#include <utility>

std::pmr::memory_resource *f1() { return std::pmr::get_default_resource(); }

using t1 = std::pmr::memory_resource;

std::pmr::memory_resource &f2(std::pmr::memory_resource &a0,
                              const std::pmr::memory_resource &a1) {
  return a0.operator=(a1);
}

void *f3(std::pmr::memory_resource &a0, std::size_t a1, std::size_t a2) {
  return a0.allocate(a1, a2);
}

void f4(std::pmr::memory_resource &a0, void *a1, std::size_t a2,
        std::size_t a3) {
  return a0.deallocate(a1, a2, a3);
}

bool f5(const std::pmr::memory_resource &a0,
        const std::pmr::memory_resource &a1) {
  return a0.is_equal(a1);
}

std::pmr::memory_resource *f6() { return std::pmr::new_delete_resource(); }

std::pmr::memory_resource *f7() { return std::pmr::null_memory_resource(); }

bool f8(const std::pmr::memory_resource &a0,
        const std::pmr::memory_resource &a1) {
  return std::pmr::operator==(a0, a1);
}

std::pmr::memory_resource *f9(std::pmr::memory_resource *a0) {
  return std::pmr::set_default_resource(a0);
}
