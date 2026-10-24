#include <algorithm>
#include <condition_variable>
#include <utility>

template <typename T1, typename T2, std::_Lock_policy T3>
std::__shared_ptr<T1, T3> f60(const std::__shared_ptr<T2, T3> &a0) {
  return std::reinterpret_pointer_cast<T1, T2, T3>(a0);
}
