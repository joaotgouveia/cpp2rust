#include <future>
#include <tuple>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
std::promise<void> f33(std::allocator_arg_t a0, const T1 &a1,
                       std::promise<void> &&a2) {
  return std::promise<void>(std::move(a0), a1, std::move(a2));
}

template <typename T1, typename T2>
std::promise<T1 &> f34(std::allocator_arg_t a0, const T2 &a1,
                       std::promise<T1 &> &&a2) {
  return std::promise<T1 &>(std::move(a0), a1, std::move(a2));
}

template <typename T1, typename T2>
std::promise<T1> f35(std::allocator_arg_t a0, const T2 &a1,
                     std::promise<T1> &&a2) {
  return std::promise<T1>(std::move(a0), a1, std::move(a2));
}
