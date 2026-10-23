#include <iterator>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
bool f21(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return a0 != a1;
}

template <typename T1>
typename std::move_iterator<T1>::pointer f22(const std::move_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::move_iterator<T1> f23(typename std::move_iterator<T1>::difference_type a0,
                           const std::move_iterator<T1> &a1) {
  return a0 + a1;
}
