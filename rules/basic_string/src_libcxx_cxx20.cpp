#include <initializer_list>
#include <string>
#include <string_view>
#include <utility>

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__can_be_converted_to_string_view_v<T1, T2, T4> &&
                           !std::is_same<std::__remove_cvref_t<T4>,
                                         std::basic_string<T1, T2, T3>>::value,
                       int>
        T5 = 0>
std::basic_string<T1, T2, T3>
f324(const T4 &a0, typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T3 &a3)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return std::basic_string<T1, T2, T3>(a0, std::move(a1), std::move(a2), a3);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__can_be_converted_to_string_view_v<T1, T2, T4> &&
                           !std::is_same<std::__remove_cvref_t<T4>,
                                         std::basic_string<T1, T2, T3>>::value,
                       int>
        T5 = 0>
std::basic_string<T1, T2, T3> f325(const T4 &a0, const T3 &a1)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return std::basic_string<T1, T2, T3>(a0, a1);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__can_be_converted_to_string_view_v<T1, T2, T4> &&
                           !std::is_same<std::__remove_cvref_t<T4>,
                                         std::basic_string<T1, T2, T3>>::value,
                       int>
        T5 = 0>
std::basic_string<T1, T2, T3> &f326(std::basic_string<T1, T2, T3> &a0,
                                    const T4 &a1)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template operator= <T4>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__can_be_converted_to_string_view_v<T1, T2, T4> &&
                           !std::is_same<std::__remove_cvref_t<T4>,
                                         std::basic_string<T1, T2, T3>>::value,
                       int>
        T5 = 0>
std::basic_string<T1, T2, T3> &f327(std::basic_string<T1, T2, T3> &a0,
                                    const T4 &a1)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template operator+= <T4>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__can_be_converted_to_string_view_v<T1, T2, T4> &&
                           !std::is_same<std::__remove_cvref_t<T4>,
                                         std::basic_string<T1, T2, T3>>::value,
                       int>
        T5 = 0>
std::basic_string<T1, T2, T3> &f328(std::basic_string<T1, T2, T3> &a0,
                                    const T4 &a1)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template append<T4>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__can_be_converted_to_string_view_v<T1, T2, T4> &&
                           !std::is_same<std::__remove_cvref_t<T4>,
                                         std::basic_string<T1, T2, T3>>::value,
                       int>
        T5 = 0>
std::basic_string<T1, T2, T3> &
f329(std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template append<T4>(a1, std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0>
std::basic_string<T1, T2, T3> &f330(std::basic_string<T1, T2, T3> &a0,
                                    const T4 &a1)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template assign<T4>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__can_be_converted_to_string_view_v<T1, T2, T4> &&
                           !std::is_same<std::__remove_cvref_t<T4>,
                                         std::basic_string<T1, T2, T3>>::value,
                       int>
        T5 = 0>
std::basic_string<T1, T2, T3> &
f331(std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template assign<T4>(a1, std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0>
std::basic_string<T1, T2, T3> &
f332(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1, const T4 &a2)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template insert<T4>(std::move(a1), a2);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__can_be_converted_to_string_view_v<T1, T2, T4> &&
                           !std::is_same<std::__remove_cvref_t<T4>,
                                         std::basic_string<T1, T2, T3>>::value,
                       int>
        T5 = 0>
std::basic_string<T1, T2, T3> &
f333(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1, const T4 &a2,
     typename std::basic_string<T1, T2, T3>::size_type a3,
     typename std::basic_string<T1, T2, T3>::size_type a4)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template insert<T4>(std::move(a1), a2, std::move(a3),
                                std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0>
std::basic_string<T1, T2, T3> &
f334(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template replace<T4>(std::move(a1), std::move(a2), a3);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__can_be_converted_to_string_view_v<T1, T2, T4> &&
                           !std::is_same<std::__remove_cvref_t<T4>,
                                         std::basic_string<T1, T2, T3>>::value,
                       int>
        T5 = 0>
std::basic_string<T1, T2, T3> &
f335(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3,
     typename std::basic_string<T1, T2, T3>::size_type a4,
     typename std::basic_string<T1, T2, T3>::size_type a5)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template replace<T4>(std::move(a1), std::move(a2), a3,
                                 std::move(a4), std::move(a5));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0>
std::basic_string<T1, T2, T3> &
f336(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, const T4 &a3)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template replace<T4>(std::move(a1), std::move(a2), a3);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0>
typename std::basic_string<T1, T2, T3>::size_type
f337(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template find<T4>(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0>
typename std::basic_string<T1, T2, T3>::size_type
f338(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template rfind<T4>(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0>
typename std::basic_string<T1, T2, T3>::size_type
f339(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template find_first_of<T4>(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0>
typename std::basic_string<T1, T2, T3>::size_type
f340(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template find_last_of<T4>(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0>
typename std::basic_string<T1, T2, T3>::size_type
f341(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template find_first_not_of<T4>(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0>
typename std::basic_string<T1, T2, T3>::size_type
f342(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template find_last_not_of<T4>(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0>
int f343(const std::basic_string<T1, T2, T3> &a0, const T4 &a1)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template compare<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0>
int f344(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template compare<T4>(std::move(a1), std::move(a2), a3);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__can_be_converted_to_string_view_v<T1, T2, T4> &&
                           !std::is_same<std::__remove_cvref_t<T4>,
                                         std::basic_string<T1, T2, T3>>::value,
                       int>
        T5 = 0>
int f345(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3,
         typename std::basic_string<T1, T2, T3>::size_type a4,
         typename std::basic_string<T1, T2, T3>::size_type a5)
  requires(!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value))
{
  return a0.template compare<T4>(std::move(a1), std::move(a2), a3,
                                 std::move(a4), std::move(a5));
}
