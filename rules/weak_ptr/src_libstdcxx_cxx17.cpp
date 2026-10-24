#include <condition_variable>
#include <utility>

template <typename T1, typename T2>
bool f17(const std::weak_ptr<T1> &a0,
         const std::__weak_ptr<T2, (std::_Lock_policy)2U> &a1) {
  return a0.template owner_before<T2>(a1);
}
