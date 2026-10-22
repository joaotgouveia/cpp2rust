#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <atomic>
#include <barrier>
#include <chrono>
#include <compare>
#include <cstddef>
#include <memory>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
std::shared_ptr<T1> f48(std::shared_ptr<T2> &&a0) {
  return std::const_pointer_cast<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::shared_ptr<T1> f49(std::shared_ptr<T2> &&a0) {
  return std::dynamic_pointer_cast<T1, T2>(std::move(a0));
}

template <typename T1>
std::strong_ordering f50(const std::shared_ptr<T1> &a0, std::nullptr_t a1) {
  return a0 <=> a1;
}

template <typename T1, typename T2>
std::strong_ordering f51(const std::shared_ptr<T1> &a0,
                         const std::shared_ptr<T2> &a1) {
  return a0 <=> a1;
}

template <typename T1, typename T2>
std::shared_ptr<T1> f52(std::shared_ptr<T2> &&a0) {
  return std::reinterpret_pointer_cast<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::shared_ptr<T1> f53(std::shared_ptr<T2> &&a0,
                        typename std::shared_ptr<T1>::element_type *a1) {
  return std::shared_ptr<T1>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f54(std::shared_ptr<T2> &&a0) {
  return std::static_pointer_cast<T1, T2>(std::move(a0));
}
