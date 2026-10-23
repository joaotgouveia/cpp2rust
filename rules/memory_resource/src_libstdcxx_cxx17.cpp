#include <algorithm>
#include <memory_resource>
#include <string>
#include <utility>

using t2 = std::pmr::monotonic_buffer_resource;

std::pmr::monotonic_buffer_resource f11(std::pmr::memory_resource *a0) {
  return std::pmr::monotonic_buffer_resource(a0);
}

std::pmr::monotonic_buffer_resource f12(std::size_t a0,
                                        std::pmr::memory_resource *a1) {
  return {a0, a1};
}

std::pmr::monotonic_buffer_resource f13(void *a0, std::size_t a1,
                                        std::pmr::memory_resource *a2) {
  return {a0, a1, a2};
}

std::pmr::monotonic_buffer_resource f14() { return {}; }

std::pmr::monotonic_buffer_resource f15(std::size_t a0) {
  return std::pmr::monotonic_buffer_resource(a0);
}

std::pmr::monotonic_buffer_resource f16(void *a0, std::size_t a1) {
  return {a0, a1};
}

void f17(std::pmr::monotonic_buffer_resource &a0) { return a0.release(); }

std::pmr::memory_resource *f18(const std::pmr::monotonic_buffer_resource &a0) {
  return a0.upstream_resource();
}

void *f19(std::pmr::monotonic_buffer_resource &a0, std::size_t a1,
          std::size_t a2) {
  return a0.allocate(a1, a2);
}

void f20(std::pmr::monotonic_buffer_resource &a0, void *a1, std::size_t a2,
         std::size_t a3) {
  return a0.deallocate(a1, a2, a3);
}

template <typename T1, typename T2>
bool f21(const std::pmr::polymorphic_allocator<T1> &a0,
         const std::pmr::polymorphic_allocator<T2> &a1) {
  return a0 == a1;
}

using t3 = std::pmr::pool_options;

std::pmr::pool_options f22() { return std::pmr::pool_options(); }

std::pmr::pool_options f23(const std::pmr::pool_options &a0) {
  return std::pmr::pool_options(a0);
}

std::pmr::pool_options f24(std::pmr::pool_options &&a0) {
  return std::pmr::pool_options(std::move(a0));
}

std::pmr::pool_options &f25(std::pmr::pool_options &a0,
                            const std::pmr::pool_options &a1) {
  return a0.operator=(a1);
}

std::pmr::pool_options &f26(std::pmr::pool_options &a0,
                            std::pmr::pool_options &&a1) {
  return a0.operator=(std::move(a1));
}

using t4 = std::pmr::synchronized_pool_resource;

std::pmr::synchronized_pool_resource f27(const std::pmr::pool_options &a0,
                                         std::pmr::memory_resource *a1) {
  return {a0, a1};
}

std::pmr::synchronized_pool_resource f28() { return {}; }

std::pmr::synchronized_pool_resource f29(std::pmr::memory_resource *a0) {
  return std::pmr::synchronized_pool_resource(a0);
}

std::pmr::synchronized_pool_resource f30(const std::pmr::pool_options &a0) {
  return std::pmr::synchronized_pool_resource(a0);
}

void f31(std::pmr::synchronized_pool_resource &a0) { return a0.release(); }

std::pmr::memory_resource *f32(const std::pmr::synchronized_pool_resource &a0) {
  return a0.upstream_resource();
}

std::pmr::pool_options f33(const std::pmr::synchronized_pool_resource &a0) {
  return a0.options();
}

void *f34(std::pmr::synchronized_pool_resource &a0, std::size_t a1,
          std::size_t a2) {
  return a0.allocate(a1, a2);
}

void f35(std::pmr::synchronized_pool_resource &a0, void *a1, std::size_t a2,
         std::size_t a3) {
  return a0.deallocate(a1, a2, a3);
}

using t5 = std::pmr::unsynchronized_pool_resource;

std::pmr::unsynchronized_pool_resource f36(const std::pmr::pool_options &a0,
                                           std::pmr::memory_resource *a1) {
  return {a0, a1};
}

std::pmr::unsynchronized_pool_resource f37() { return {}; }

std::pmr::unsynchronized_pool_resource f38(std::pmr::memory_resource *a0) {
  return std::pmr::unsynchronized_pool_resource(a0);
}

std::pmr::unsynchronized_pool_resource f39(const std::pmr::pool_options &a0) {
  return std::pmr::unsynchronized_pool_resource(a0);
}

void f40(std::pmr::unsynchronized_pool_resource &a0) { return a0.release(); }

std::pmr::memory_resource *
f41(const std::pmr::unsynchronized_pool_resource &a0) {
  return a0.upstream_resource();
}

std::pmr::pool_options f42(const std::pmr::unsynchronized_pool_resource &a0) {
  return a0.options();
}

void *f43(std::pmr::unsynchronized_pool_resource &a0, std::size_t a1,
          std::size_t a2) {
  return a0.allocate(a1, a2);
}

void f44(std::pmr::unsynchronized_pool_resource &a0, void *a1, std::size_t a2,
         std::size_t a3) {
  return a0.deallocate(a1, a2, a3);
}
