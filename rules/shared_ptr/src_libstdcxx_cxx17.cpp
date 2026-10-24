#include <algorithm>
#include <condition_variable>
#include <ostream>
#include <utility>

template <typename T1, typename T2>
std::shared_ptr<T1> f69(const std::shared_ptr<T2> &a0) {
  return std::reinterpret_pointer_cast<T1, T2>(a0);
}

template <typename T1, typename T2>
bool f70(const std::shared_ptr<T1> &a0,
         const std::__weak_ptr<T2, (std::_Lock_policy)2U> &a1) {
  return a0.template owner_before<T2>(a1);
}
