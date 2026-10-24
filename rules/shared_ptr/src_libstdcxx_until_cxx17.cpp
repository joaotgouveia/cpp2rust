#include <algorithm>
#include <condition_variable>
#include <utility>

template <typename T1, typename T2, typename T3,
          typename std::enable_if<std::is_constructible<
              std::__shared_ptr<T1>, std::unique_ptr<T2, T3>,
              std::__sp_array_delete>::value>::type *T4 = 0>
std::shared_ptr<T1> f63(std::unique_ptr<T2, T3> &&a0) {
  return std::shared_ptr<T1>(std::move(a0));
}
