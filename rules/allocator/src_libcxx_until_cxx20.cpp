#include <cstddef>
#include <memory>
#include <utility>

using t25 = std::allocator<void>;

using t26 = std::allocator<void>::value_type;

std::allocator<void> f23() { return std::allocator<void>(); }

std::allocator<void> f24(const std::allocator<void> &a0) {
  return std::allocator<void>(a0);
}

std::allocator<void> &f25(std::allocator<void> &a0,
                          const std::allocator<void> &a1) {
  return a0.operator=(a1);
}

template <typename T1> T1 *f26(const std::allocator<T1> &a0, T1 &a1) {
  return a0.address(a1);
}

template <typename T1>
const T1 *f27(const std::allocator<T1> &a0, const T1 &a1) {
  return a0.address(a1);
}

template <typename T1>
T1 *f28(std::allocator<T1> &a0, std::size_t a1, const void *a2) {
  return a0.allocate(a1, a2);
}

template <typename T1> std::size_t f29(const std::allocator<T1> &a0) {
  return a0.max_size();
}

template <typename T1> void f30(std::allocator<T1> &a0, T1 *a1) {
  return a0.destroy(a1);
}

template <typename T1, typename T2, typename... T3>
void f31(std::allocator<T1> &a0, T2 *a1, T3 &&...a2) {
  return a0.template construct<T2, T3...>(a1, std::forward<T3>(a2)...);
}
