#include <utility>
#include <valarray>

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
void f38(const std::gslice_array<T1> &a0, const T2 &a1)
  requires(
      !(std::is_same<T2, std::gslice_array<T1>>::value) &&
      !(std::is_same<T2, typename std::gslice_array<T1>::value_type>::value))
{
  return a0.template operator= <T2>(a1);
}
