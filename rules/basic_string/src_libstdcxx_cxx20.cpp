#include <algorithm>
#include <initializer_list>
#include <iosfwd>
#include <string>
#include <type_traits>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f242()
  requires(std::is_default_constructible_v<T3>)
{
  return std::basic_string<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
T3 f243(const std::basic_string<T1, T2, T3> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = std::enable_if_t<std::is_convertible_v<
              const T4 &, std::basic_string_view<T1, T2>>>>
std::basic_string<T1, T2, T3>
f244(const T4 &a0, typename std::basic_string<T1, T2, T3>::size_type a1,
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
std::basic_string<T1, T2, T3> f245(const T4 &a0, const T3 &a1)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return std::basic_string<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    std::basic_string<T1, T2, T3> &>
f246(std::basic_string<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template operator= <T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    std::basic_string<T1, T2, T3> &>
f247(std::basic_string<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template operator+= <T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    std::basic_string<T1, T2, T3> &>
f248(std::basic_string<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template append<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    std::basic_string<T1, T2, T3> &>
f249(std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template append<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    std::basic_string<T1, T2, T3> &>
f250(std::basic_string<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template assign<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    std::basic_string<T1, T2, T3> &>
f251(std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template assign<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    std::basic_string<T1, T2, T3> &>
f252(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1, const T4 &a2)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template insert<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    std::basic_string<T1, T2, T3> &>
f253(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1, const T4 &a2,
     typename std::basic_string<T1, T2, T3>::size_type a3,
     typename std::basic_string<T1, T2, T3>::size_type a4)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template insert<T4>(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = std::_RequireInputIter<T4>>
std::basic_string<T1, T2, T3> &
f254(std::basic_string<T1, T2, T3> &a0,
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

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    std::basic_string<T1, T2, T3> &>
f255(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template replace<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    std::basic_string<T1, T2, T3> &>
f256(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3,
     typename std::basic_string<T1, T2, T3>::size_type a4,
     typename std::basic_string<T1, T2, T3>::size_type a5)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template replace<T4>(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    std::basic_string<T1, T2, T3> &>
f257(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, const T4 &a3)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template replace<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    typename __gnu_cxx::__alloc_traits<T3>::size_type>
f258(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template find<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    typename __gnu_cxx::__alloc_traits<T3>::size_type>
f259(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template rfind<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    typename __gnu_cxx::__alloc_traits<T3>::size_type>
f260(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template find_first_of<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    typename __gnu_cxx::__alloc_traits<T3>::size_type>
f261(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template find_last_of<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    typename __gnu_cxx::__alloc_traits<T3>::size_type>
f262(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template find_first_not_of<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    typename __gnu_cxx::__alloc_traits<T3>::size_type>
f263(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template find_last_not_of<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    int>
f264(const std::basic_string<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template compare<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    int>
f265(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template compare<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    int>
f266(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3,
     typename std::basic_string<T1, T2, T3>::size_type a4,
     typename std::basic_string<T1, T2, T3>::size_type a5)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template compare<T4>(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3>
auto f267(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return std::operator<=> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
auto f268(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return std::operator<=> <T1, T2, T3>(a0, a1);
}
