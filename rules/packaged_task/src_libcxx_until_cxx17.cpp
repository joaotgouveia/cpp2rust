#include "../common/init.h"
#include "../common/traits.h"
#include <future>
#include <memory>
#include <utility>

template <typename... T1, typename T2, typename T3>
std::packaged_task<void(T1...)> f24(std::allocator_arg_t a0, const T3 &a1,
                                    T2 &&a2) {
  return std::packaged_task<void(T1...)>(std::move(a0), a1,
                                         std::forward<T2>(a2));
}
