#include <cstddef>
#include <memory>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t26 = std::allocator<void>::pointer;

using t27 = std::allocator<void>::const_pointer;

using t28 = std::allocator<void>::value_type;

template <typename T1> using t29 = typename std::allocator<T1>::pointer;

template <typename T1> using t30 = typename std::allocator<T1>::const_pointer;

template <typename T1> using t31 = typename std::allocator<T1>::reference;

template <typename T1> using t32 = typename std::allocator<T1>::const_reference;

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

template <typename T1>
typename std::allocator<T1>::size_type f29(const std::allocator<T1> &a0) {
  return a0.max_size();
}

template <typename T1> void f30(std::allocator<T1> &a0, T1 *a1) {
  return a0.destroy(a1);
}

template <typename T1, typename T2, typename... T3>
void f31(std::allocator<T1> &a0, T2 *a1, T3 &&...a2) {
  return a0.template construct<T2, T3...>(a1, std::forward<T3>(a2)...);
}
