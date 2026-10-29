#include <algorithm>
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

using t2 = std::pmr::monotonic_buffer_resource;

std::pmr::monotonic_buffer_resource f6(std::pmr::memory_resource *a0) {
  return std::pmr::monotonic_buffer_resource(a0);
}

std::pmr::monotonic_buffer_resource f7(std::size_t a0,
                                       std::pmr::memory_resource *a1) {
  return {a0, a1};
}

std::pmr::monotonic_buffer_resource f8(void *a0, std::size_t a1,
                                       std::pmr::memory_resource *a2) {
  return {a0, a1, a2};
}

std::pmr::monotonic_buffer_resource f9() { return {}; }

std::pmr::monotonic_buffer_resource f10(std::size_t a0) {
  return std::pmr::monotonic_buffer_resource(a0);
}

std::pmr::monotonic_buffer_resource f11(void *a0, std::size_t a1) {
  return {a0, a1};
}

void f12(std::pmr::monotonic_buffer_resource &a0) { return a0.release(); }

std::pmr::memory_resource *f13(const std::pmr::monotonic_buffer_resource &a0) {
  return a0.upstream_resource();
}

void *f14(std::pmr::monotonic_buffer_resource &a0, std::size_t a1,
          std::size_t a2) {
  return a0.allocate(a1, a2);
}

void f15(std::pmr::monotonic_buffer_resource &a0, void *a1, std::size_t a2,
         std::size_t a3) {
  return a0.deallocate(a1, a2, a3);
}

std::pmr::memory_resource *f16() { return std::pmr::new_delete_resource(); }

std::pmr::memory_resource *f17() { return std::pmr::null_memory_resource(); }

bool f18(const std::pmr::memory_resource &a0,
         const std::pmr::memory_resource &a1) {
  return std::pmr::operator==(a0, a1);
}

template <typename T1, typename T2>
bool f19(const std::pmr::polymorphic_allocator<T1> &a0,
         const std::pmr::polymorphic_allocator<T2> &a1) {
  return std::pmr::operator== <T1, T2>(a0, a1);
}

using t3 = std::pmr::pool_options;

std::pmr::pool_options f20() { return std::pmr::pool_options(); }

std::pmr::pool_options f21(const std::pmr::pool_options &a0) {
  return std::pmr::pool_options(a0);
}

std::pmr::pool_options f22(std::pmr::pool_options &&a0) {
  return std::pmr::pool_options(std::move(a0));
}

std::pmr::pool_options &f23(std::pmr::pool_options &a0,
                            const std::pmr::pool_options &a1) {
  return a0.operator=(a1);
}

std::pmr::pool_options &f24(std::pmr::pool_options &a0,
                            std::pmr::pool_options &&a1) {
  return a0.operator=(std::move(a1));
}

std::pmr::memory_resource *f25(std::pmr::memory_resource *a0) {
  return std::pmr::set_default_resource(a0);
}

using t4 = std::pmr::synchronized_pool_resource;

std::pmr::synchronized_pool_resource f26(const std::pmr::pool_options &a0,
                                         std::pmr::memory_resource *a1) {
  return {a0, a1};
}

std::pmr::synchronized_pool_resource f27() { return {}; }

std::pmr::synchronized_pool_resource f28(std::pmr::memory_resource *a0) {
  return std::pmr::synchronized_pool_resource(a0);
}

std::pmr::synchronized_pool_resource f29(const std::pmr::pool_options &a0) {
  return std::pmr::synchronized_pool_resource(a0);
}

void f30(std::pmr::synchronized_pool_resource &a0) { return a0.release(); }

std::pmr::memory_resource *f31(const std::pmr::synchronized_pool_resource &a0) {
  return a0.upstream_resource();
}

std::pmr::pool_options f32(const std::pmr::synchronized_pool_resource &a0) {
  return a0.options();
}

void *f33(std::pmr::synchronized_pool_resource &a0, std::size_t a1,
          std::size_t a2) {
  return a0.allocate(a1, a2);
}

void f34(std::pmr::synchronized_pool_resource &a0, void *a1, std::size_t a2,
         std::size_t a3) {
  return a0.deallocate(a1, a2, a3);
}

using t5 = std::pmr::unsynchronized_pool_resource;

std::pmr::unsynchronized_pool_resource f35(const std::pmr::pool_options &a0,
                                           std::pmr::memory_resource *a1) {
  return {a0, a1};
}

std::pmr::unsynchronized_pool_resource f36() { return {}; }

std::pmr::unsynchronized_pool_resource f37(std::pmr::memory_resource *a0) {
  return std::pmr::unsynchronized_pool_resource(a0);
}

std::pmr::unsynchronized_pool_resource f38(const std::pmr::pool_options &a0) {
  return std::pmr::unsynchronized_pool_resource(a0);
}

void f39(std::pmr::unsynchronized_pool_resource &a0) { return a0.release(); }

std::pmr::memory_resource *
f40(const std::pmr::unsynchronized_pool_resource &a0) {
  return a0.upstream_resource();
}

std::pmr::pool_options f41(const std::pmr::unsynchronized_pool_resource &a0) {
  return a0.options();
}

void *f42(std::pmr::unsynchronized_pool_resource &a0, std::size_t a1,
          std::size_t a2) {
  return a0.allocate(a1, a2);
}

void f43(std::pmr::unsynchronized_pool_resource &a0, void *a1, std::size_t a2,
         std::size_t a3) {
  return a0.deallocate(a1, a2, a3);
}
