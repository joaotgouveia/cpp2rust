#include <algorithm>
#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f94(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
         const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f95(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<T6, typename std::unordered_map<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f96(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<T6, typename std::unordered_map<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f97(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<T6, typename std::unordered_map<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template count<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f98(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<T6, typename std::unordered_map<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template contains<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f99(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<T6, typename std::unordered_map<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template equal_range<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f100(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<T6, typename std::unordered_map<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template equal_range<T6>(a1);
}
