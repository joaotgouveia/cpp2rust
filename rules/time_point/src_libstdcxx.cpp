#include <chrono>
#include <utility>

template <typename T1, typename T2, typename T3,
          typename T4 = std::_Require<std::is_convertible<T3, T2>>>
std::chrono::time_point<T1, T2> f12(const std::chrono::time_point<T1, T3> &a0) {
  return std::chrono::time_point<T1, T2>(a0);
}
