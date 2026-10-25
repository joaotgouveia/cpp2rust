#include <expected>
#include <initializer_list>
#include <utility>

template <typename T1>
void f15(std::unexpected<T1> &a0, std::unexpected<T1> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2 = T1>
std::unexpected<T1> f16(T2 &&a0)
  requires((!std::is_same_v<std::remove_cvref_t<T2>, std::unexpected<T1>> &&
            !std::is_same_v<std::remove_cvref_t<T2>, std::in_place_t> &&
            std::is_constructible_v<T1, T2>)) &&
          (!(std::is_same<T2, const std::unexpected<T1> &>::value) &&
           !(std::is_same<T2, std::unexpected<T1>>::value))
{
  return std::unexpected<T1>(std::forward<T2>(a0));
}
