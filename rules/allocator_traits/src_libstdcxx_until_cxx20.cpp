#include <algorithm>
#include <string>
#include <utility>

template <typename T1, typename T2, typename... T3>
std::__enable_if_t<
    std::__allocator_traits_base::template __can_construct<T1, T2, T3...>>
f22(T1 &a0, T2 *a1, T3 &&...a2) {
  return std::allocator_traits<T1>::construct(a0, a1, std::forward<T3>(a2)...);
}
