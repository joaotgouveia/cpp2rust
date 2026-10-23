#include <algorithm>
#include <initializer_list>
#include <iosfwd>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f227()
  requires(std::is_default_constructible_v<T3>)
{
  return std::basic_string<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
T3 f228(const std::basic_string<T1, T2, T3> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = std::enable_if_t<std::is_convertible_v<
              const T4 &, std::basic_string_view<T1, T2>>>>
std::basic_string<T1, T2, T3>
f229(const T4 &a0, typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T3 &a3)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return std::basic_string<T1, T2, T3>(a0, a1, a2, a3);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename T5 = std::enable_if_t<
        std::__and_<
            std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
            std::__not_<std::is_convertible<
                T4 const *, const std::basic_string<T1, T2, T3> *>>,
            std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
        void>>
std::basic_string<T1, T2, T3> f230(const T4 &a0, const T3 &a1)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return std::basic_string<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = std::_RequireInputIter<T4>>
std::basic_string<T1, T2, T3> &
f231(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, T4 a3, T4 a4)
  requires(
      !(std::is_same<T4, T1 *>::value) &&
      !(std::is_same<T4, const T1 *>::value) &&
      !(std::is_same<
          T4, typename std::basic_string<T1, T2, T3>::iterator>::value) &&
      !(std::is_same<
          T4, typename std::basic_string<T1, T2, T3>::const_iterator>::value))
{
  return a0.template replace<T4>(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
auto f232(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return std::operator<=> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
auto f233(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return std::operator<=> <T1, T2, T3>(a0, a1);
}
