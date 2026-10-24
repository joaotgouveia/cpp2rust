#include <numeric>
#include <utility>

template <typename T1>
T1 *f1(T1 *a0, T1 *a1)
  requires((std::is_object_v<T1> && (sizeof(T1) > 0)))
{
  return std::midpoint<T1>(a0, a1);
}

template <typename T1>
T1 f2(T1 a0, T1 a1)
  requires((std::is_integral_v<T1> &&
            !std::is_same_v<std::remove_cv_t<T1>, bool>))
{
  return std::midpoint<T1>(std::move(a0), std::move(a1));
}
