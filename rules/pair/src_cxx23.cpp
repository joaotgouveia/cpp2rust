#include <algorithm>
#include <utility>

template <typename T1, typename T2> using t1 = std::pair<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::pair<T1, T2>::first_type;

template <typename T1, typename T2>
using t3 = typename std::pair<T1, T2>::second_type;

template <typename T1, typename T2>
std::pair<T1, T2> f1(const std::pair<T1, T2> &a0) {
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2>
std::pair<T1, T2> f2(std::pair<T1, T2> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
void f3(std::pair<T1, T2> &a0, std::pair<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2>
void f4(const std::pair<T1, T2> &a0, const std::pair<T1, T2> &a1)
  requires(std::is_swappable_v<const T1> && std::is_swappable_v<const T2>)
{
  return a0.swap(a1);
}

template <typename T1, typename T2>
std::pair<T1, T2> f5()
  requires(std::is_default_constructible_v<T1> &&
           std::is_default_constructible_v<T2>)
{
  return std::pair<T1, T2>();
}

template <typename T1, typename T2>
std::pair<T1, T2> f6(const T1 &a0, const T2 &a1) {
  return std::pair<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::pair<T1, T2> &f7(std::pair<T1, T2> &a0, const std::pair<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::pair<T1, T2> &f8(std::pair<T1, T2> &a0, std::pair<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
const std::pair<T1, T2> &f9(const std::pair<T1, T2> &a0,
                            const std::pair<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
const std::pair<T1, T2> &f10(const std::pair<T1, T2> &a0,
                             std::pair<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename... T3, typename... T4>
std::pair<T1, T2> f11(std::piecewise_construct_t a0, std::tuple<T3...> a1,
                      std::tuple<T4...> a2) {
  return std::pair<T1, T2>(std::move(a0), std::forward<std::tuple<T3...>>(a1),
                           std::forward<std::tuple<T4...>>(a2));
}

template <typename T1, typename T2, typename T3 = T1, typename T4 = T2>
std::pair<T1, T2> f12(T3 &&a0, T4 &&a1)
  requires(!(std::is_same<T3, const std::type_identity_t<T1> &>::value &&
             std::is_same<T4, const T2 &>::value))
{
  return std::pair<T1, T2>(std::forward<T3>(a0), std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> f13(const std::pair<T3, T4> &a0)
  requires(!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> f14(std::pair<T3, T4> &&a0)
  requires(!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> f15(std::pair<T3, T4> &a0) {
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> f16(const std::pair<T3, T4> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::pair<T1, T2> f17(T3 &&a0)
  requires(!(std::is_same<T3, const std::pair<T1, T2> &>::value) &&
           !(std::is_same<T3, std::pair<T1, T2>>::value))
{
  return std::pair<T1, T2>(std::forward<T3>(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> &f18(std::pair<T1, T2> &a0, const std::pair<T3, T4> &a1)
  requires(!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> &f19(std::pair<T1, T2> &a0, std::pair<T3, T4> &&a1)
  requires(!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return a0.template operator= <T3, T4>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
const std::pair<T1, T2> &f20(const std::pair<T1, T2> &a0,
                             const std::pair<T3, T4> &a1)
  requires(!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
const std::pair<T1, T2> &f21(const std::pair<T1, T2> &a0,
                             std::pair<T3, T4> &&a1)
  requires(!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return a0.template operator= <T3, T4>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::pair<T1, T2> &f22(std::pair<T1, T2> &a0, T3 &&a1)
  requires(!(std::is_same<T3, const std::pair<T1, T2> &>::value) &&
           !(std::is_same<T3, std::pair<T1, T2>>::value))
{
  return a0.template operator= <T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
const std::pair<T1, T2> &f23(const std::pair<T1, T2> &a0, T3 &&a1)
  requires(!(std::is_same<T3, const std::pair<T1, T2> &>::value) &&
           !(std::is_same<T3, std::pair<T1, T2>>::value))
{
  return a0.template operator= <T3>(std::forward<T3>(a1));
}
