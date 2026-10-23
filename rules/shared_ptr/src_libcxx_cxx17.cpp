#include <atomic>
#include <cstddef>
#include <memory>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
typename std::add_lvalue_reference<typename std::remove_extent<T1>::type>::type
f90(const std::shared_ptr<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::add_lvalue_reference<typename std::remove_extent<T1>::type>::type
f91(const std::shared_ptr<T1> &a0, std::ptrdiff_t a1) {
  return a0.operator[](a1);
}
