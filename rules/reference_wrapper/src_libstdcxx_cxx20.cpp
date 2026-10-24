#include <barrier>
#include <utility>

template <
    typename T1, typename T2,
    typename T3 = typename std::enable_if<!std::is_same<
        std::reference_wrapper<T1>, std::__remove_cvref_t<T2>>::value>::type,
    typename T4 =
        decltype(std::reference_wrapper<T1>::_S_fun(std::declval<T2>()))>
std::reference_wrapper<T1> f7(T2 &&a0)
  requires(!(std::is_same<T2, const std::reference_wrapper<T1> &>::value))
{
  return std::reference_wrapper<T1>(std::forward<T2>(a0));
}
