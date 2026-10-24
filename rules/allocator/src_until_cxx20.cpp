#include <algorithm>
#include <cstddef>
#include <memory>
#include <string>
#include <utility>

using t1 = std::allocator<void>::pointer;

using t2 = std::allocator<void>::const_pointer;

std::allocator<void> f1(std::allocator<void> &&a0) {
  return std::allocator<void>(std::move(a0));
}

std::allocator<void> &f2(std::allocator<void> &a0, std::allocator<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> using t3 = typename std::allocator<T1>::pointer;

template <typename T1> using t4 = typename std::allocator<T1>::const_pointer;

template <typename T1> using t5 = typename std::allocator<T1>::reference;

template <typename T1> using t6 = typename std::allocator<T1>::const_reference;

template <typename T1> T1 *f3(const std::allocator<T1> &a0, T1 &a1) {
  return a0.address(a1);
}

template <typename T1>
T1 *f4(std::allocator<T1> &a0, std::size_t a1, const void *a2) {
  return a0.allocate(a1, a2);
}

template <typename T1> std::size_t f5(const std::allocator<T1> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename... T3>
void f6(std::allocator<T1> &a0, T2 *a1, T3 &&...a2) {
  return a0.template construct<T2, T3...>(a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2>
bool f7(const std::allocator<T1> &a0, const std::allocator<T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}
