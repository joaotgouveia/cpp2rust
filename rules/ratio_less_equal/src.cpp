#include "../common/init.h"
#include "../common/traits.h"
#include <ratio>
#include <utility>

template <typename T1, typename T2> using t1 = std::ratio_less_equal<T1, T2>;

template <typename T1, typename T2>
typename std::ratio_less_equal<T1, T2>::value_type
f1(const std::ratio_less_equal<T1, T2> &a0) {
  return static_cast<typename std::ratio_less_equal<T1, T2>::value_type>(a0);
}
