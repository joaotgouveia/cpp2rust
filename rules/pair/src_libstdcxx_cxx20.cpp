#include <algorithm>
#include <utility>

template <typename T1, typename T2>
std::pair<T1, T2> f21()
  requires(std::is_default_constructible_v<T1> &&
           std::is_default_constructible_v<T2>)
{
  return std::pair<T1, T2>();
}

template <typename T1, typename T2>
std::pair<T1, T2> f22(const T1 &a0, const T2 &a1)
  requires(
      (std::pair<T1, T2>::template _S_constructible<const T1 &, const T2 &>()))
{
  return std::pair<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::pair<T1, T2> &f23(std::pair<T1, T2> &a0, const std::pair<T1, T2> &a1)
  requires(
      (std::pair<T1, T2>::template _S_assignable<const T1 &, const T2 &>()))
{
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::pair<T1, T2> &f24(std::pair<T1, T2> &a0, std::pair<T1, T2> &&a1)
  requires((std::pair<T1, T2>::template _S_assignable<T1, T2>()))
{
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> f25(T3 &&a0, T4 &&a1)
  requires((std::pair<T1, T2>::template _S_constructible<T3, T4>()) &&
           (!std::pair<T1, T2>::template _S_dangles<T3, T4>())) &&
          (!(std::is_same<T3, const std::type_identity_t<T1> &>::value &&
             std::is_same<T4, const T2 &>::value))
{
  return std::pair<T1, T2>(std::forward<T3>(a0), std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> f26(const std::pair<T3, T4> &a0)
  requires((std::pair<T1, T2>::template _S_constructible<const T3 &,
                                                         const T4 &>()) &&
           (!std::pair<T1, T2>::template _S_dangles<T3, T4>())) &&
          (!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> f27(std::pair<T3, T4> &&a0)
  requires((std::pair<T1, T2>::template _S_constructible<T3, T4>()) &&
           (!std::pair<T1, T2>::template _S_dangles<T3, T4>())) &&
          (!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> &f28(std::pair<T1, T2> &a0, const std::pair<T3, T4> &a1)
  requires((std::pair<T1, T2>::template _S_assignable<const T3 &,
                                                      const T4 &>())) &&
          (!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> &f29(std::pair<T1, T2> &a0, std::pair<T3, T4> &&a1)
  requires((std::pair<T1, T2>::template _S_assignable<T3, T4>())) &&
          (!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return a0.template operator= <T3, T4>(std::move(a1));
}
