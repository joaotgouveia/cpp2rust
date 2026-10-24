#include <cstddef>
#include <initializer_list>
#include <utility>
#include <valarray>

bool f265(const std::slice &a0, const std::slice &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f266(std::valarray<T1> &a0, const T2 &a1)
  requires(!(std::is_same<T2, typename std::valarray<T1>::value_type>::value))
{
  return a0.template operator*= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f267(std::valarray<T1> &a0, const T2 &a1)
  requires(!(std::is_same<T2, typename std::valarray<T1>::value_type>::value))
{
  return a0.template operator/= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f268(std::valarray<T1> &a0, const T2 &a1)
  requires(!(std::is_same<T2, typename std::valarray<T1>::value_type>::value))
{
  return a0.template operator%= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f269(std::valarray<T1> &a0, const T2 &a1)
  requires(!(std::is_same<T2, typename std::valarray<T1>::value_type>::value))
{
  return a0.template operator+= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f270(std::valarray<T1> &a0, const T2 &a1)
  requires(!(std::is_same<T2, typename std::valarray<T1>::value_type>::value))
{
  return a0.template operator-= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f271(std::valarray<T1> &a0, const T2 &a1)
  requires(!(std::is_same<T2, typename std::valarray<T1>::value_type>::value))
{
  return a0.template operator^= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f272(std::valarray<T1> &a0, const T2 &a1)
  requires(!(std::is_same<T2, typename std::valarray<T1>::value_type>::value))
{
  return a0.template operator|= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f273(std::valarray<T1> &a0, const T2 &a1)
  requires(!(std::is_same<T2, typename std::valarray<T1>::value_type>::value))
{
  return a0.template operator&= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f274(std::valarray<T1> &a0, const T2 &a1)
  requires(!(std::is_same<T2, typename std::valarray<T1>::value_type>::value))
{
  return a0.template operator<<= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f275(std::valarray<T1> &a0, const T2 &a1)
  requires(!(std::is_same<T2, typename std::valarray<T1>::value_type>::value))
{
  return a0.template operator>>= <T2>(a1);
}
