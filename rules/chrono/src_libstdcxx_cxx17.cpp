#include <chrono>
#include <limits>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
std::enable_if_t<std::numeric_limits<T1>::is_signed,
                 std::chrono::duration<T1, T2>>
f39(std::chrono::duration<T1, T2> a0) {
  return std::chrono::abs<T1, T2>(a0);
}
