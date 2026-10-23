#include <cstddef>
#include <initializer_list>
#include <utility>
#include <valarray>

bool f238(const std::slice &a0, const std::slice &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f239(std::valarray<T1> &a0, const T2 &a1)
  requires(!(std::is_same<T2, typename std::valarray<T1>::value_type>::value))
{
  return a0.template operator*= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f240(std::valarray<T1> &a0, const T2 &a1)
  requires(!(std::is_same<T2, typename std::valarray<T1>::value_type>::value))
{
  return a0.template operator/= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f241(std::valarray<T1> &a0, const T2 &a1)
  requires(!(std::is_same<T2, typename std::valarray<T1>::value_type>::value))
{
  return a0.template operator%= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f242(std::valarray<T1> &a0, const T2 &a1)
  requires(!(std::is_same<T2, typename std::valarray<T1>::value_type>::value))
{
  return a0.template operator+= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f243(std::valarray<T1> &a0, const T2 &a1)
  requires(!(std::is_same<T2, typename std::valarray<T1>::value_type>::value))
{
  return a0.template operator-= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f244(std::valarray<T1> &a0, const T2 &a1)
  requires(!(std::is_same<T2, typename std::valarray<T1>::value_type>::value))
{
  return a0.template operator^= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f245(std::valarray<T1> &a0, const T2 &a1)
  requires(!(std::is_same<T2, typename std::valarray<T1>::value_type>::value))
{
  return a0.template operator|= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f246(std::valarray<T1> &a0, const T2 &a1)
  requires(!(std::is_same<T2, typename std::valarray<T1>::value_type>::value))
{
  return a0.template operator&= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f247(std::valarray<T1> &a0, const T2 &a1)
  requires(!(std::is_same<T2, typename std::valarray<T1>::value_type>::value))
{
  return a0.template operator<<= <T2>(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0>
std::valarray<T1> &f248(std::valarray<T1> &a0, const T2 &a1)
  requires(!(std::is_same<T2, typename std::valarray<T1>::value_type>::value))
{
  return a0.template operator>>= <T2>(a1);
}
