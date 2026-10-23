#include <ratio>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
typename std::ratio_greater<T1, T2>::value_type
f1(const std::ratio_greater<T1, T2> &a0) {
  return static_cast<typename std::ratio_greater<T1, T2>::value_type>(a0);
}
