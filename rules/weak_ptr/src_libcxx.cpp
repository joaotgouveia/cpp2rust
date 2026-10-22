#include "../common/init.h"
#include "../common/traits.h"
#include <memory>
#include <utility>

template <typename T1> using t2 = typename std::weak_ptr<T1>::element_type;

template <typename T1> void f16(std::weak_ptr<T1> &a0, std::weak_ptr<T1> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2>
bool f17(const std::weak_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0.template owner_before<T2>(a1);
}

template <typename T1, typename T2>
bool f18(const std::weak_ptr<T1> &a0, const std::weak_ptr<T2> &a1) {
  return a0.template owner_before<T2>(a1);
}
