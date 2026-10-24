#include <algorithm>
#include <condition_variable>
#include <utility>

template <typename T1, typename T2>
std::shared_ptr<T1> f64(const std::shared_ptr<T2> &a0) {
  return std::reinterpret_pointer_cast<T1, T2>(a0);
}
