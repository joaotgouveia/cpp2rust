#include <algorithm>
#include <utility>

template <typename T1, typename T2> using t1 = std::common_iterator<T1, T2>;

template <typename T1, typename T2>
std::common_iterator<T1, T2> f1()
  requires(std::default_initializable<T1>) &&
          ((!std::same_as<T1, T2>) && std::copyable<T1>)
{
  return std::common_iterator<T1, T2>();
}

template <typename T1, typename T2>
std::common_iterator<T1, T2> f2(T1 a0)
  requires((!std::same_as<T1, T2>) && std::copyable<T1>)
{
  return std::common_iterator<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::common_iterator<T1, T2> f3(T2 a0)
  requires((!std::same_as<T1, T2>) && std::copyable<T1>)
{
  return std::common_iterator<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::common_iterator<T1, T2> f4(const std::common_iterator<T1, T2> &a0)
  requires((!std::same_as<T1, T2>) && std::copyable<T1>)
{
  return std::common_iterator<T1, T2>(a0);
}

template <typename T1, typename T2>
std::common_iterator<T1, T2> f5(const std::common_iterator<T1, T2> &a0)
  requires((!std::is_trivially_copyable_v<T1> ||
            !std::is_trivially_copyable_v<T2>)) &&
          ((!std::same_as<T1, T2>) && std::copyable<T1>)
{
  return std::common_iterator<T1, T2>(a0);
}

template <typename T1, typename T2>
std::common_iterator<T1, T2> f6(std::common_iterator<T1, T2> &&a0)
  requires((!std::same_as<T1, T2>) && std::copyable<T1>)
{
  return std::common_iterator<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::common_iterator<T1, T2> f7(std::common_iterator<T1, T2> &&a0)
  requires((!std::is_trivially_copyable_v<T1> ||
            !std::is_trivially_copyable_v<T2>)) &&
          ((!std::same_as<T1, T2>) && std::copyable<T1>)
{
  return std::common_iterator<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::common_iterator<T1, T2> &f8(std::common_iterator<T1, T2> &a0,
                                 const std::common_iterator<T1, T2> &a1)
  requires((!std::same_as<T1, T2>) && std::copyable<T1>)
{
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::common_iterator<T1, T2> &f9(std::common_iterator<T1, T2> &a0,
                                 const std::common_iterator<T1, T2> &a1)
  requires((!std::is_trivially_copy_assignable_v<T1> ||
            !std::is_trivially_copy_assignable_v<T2>)) &&
          ((!std::same_as<T1, T2>) && std::copyable<T1>)
{
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::common_iterator<T1, T2> &f10(std::common_iterator<T1, T2> &a0,
                                  std::common_iterator<T1, T2> &&a1)
  requires((!std::same_as<T1, T2>) && std::copyable<T1>)
{
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::common_iterator<T1, T2> &f11(std::common_iterator<T1, T2> &a0,
                                  std::common_iterator<T1, T2> &&a1)
  requires((!std::is_trivially_move_assignable_v<T1> ||
            !std::is_trivially_move_assignable_v<T2>)) &&
          ((!std::same_as<T1, T2>) && std::copyable<T1>)
{
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
decltype(auto) f12(std::common_iterator<T1, T2> &a0)
  requires((!std::same_as<T1, T2>) && std::copyable<T1>)
{
  return a0.operator*();
}

template <typename T1, typename T2>
decltype(auto) f13(const std::common_iterator<T1, T2> &a0)
  requires((!std::same_as<T1, T2>) && std::copyable<T1>)
{
  return a0.operator*();
}

template <typename T1, typename T2>
auto f14(const std::common_iterator<T1, T2> &a0)
  requires((!std::same_as<T1, T2>) && std::copyable<T1>)
{
  return a0.operator->();
}

template <typename T1, typename T2>
std::common_iterator<T1, T2> &f15(std::common_iterator<T1, T2> &a0)
  requires((!std::same_as<T1, T2>) && std::copyable<T1>)
{
  return a0.operator++();
}

template <typename T1, typename T2>
decltype(auto) f16(std::common_iterator<T1, T2> &a0, int a1)
  requires((!std::same_as<T1, T2>) && std::copyable<T1>)
{
  return a0.operator++(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::common_iterator<T1, T2> f17(const std::common_iterator<T3, T4> &a0)
  requires(std::convertible_to<const T3 &, T1> &&
           std::convertible_to<const T4 &, T2>) &&
          ((!std::same_as<T1, T2>) && std::copyable<T1>) &&
          (!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return std::common_iterator<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::common_iterator<T1, T2> &f18(std::common_iterator<T1, T2> &a0,
                                  const std::common_iterator<T3, T4> &a1)
  requires(std::convertible_to<const T3 &, T1> &&
           std::convertible_to<const T4 &, T2> &&
           std::assignable_from<T1 &, const T3 &> &&
           std::assignable_from<T2 &, const T4 &>) &&
          ((!std::same_as<T1, T2>) && std::copyable<T1>) &&
          (!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f19(const std::common_iterator<T1, T2> &a0,
         const std::common_iterator<T3, T4> &a1)
  requires(std::indirectly_swappable<T3, T1>)
{
  return iter_swap(a0, a1);
}

template <typename T1, typename T2>
std::iter_rvalue_reference_t<T1> f20(const std::common_iterator<T1, T2> &a0) {
  return iter_move(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::iter_difference_t<T3> f21(const std::common_iterator<T1, T2> &a0,
                               const std::common_iterator<T3, T4> &a1)
  requires(std::sized_sentinel_for<T2, T3>) &&
          (std::sized_sentinel_for<T3, T1>) && (std::sized_sentinel_for<T4, T1>)
{
  return operator-(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f22(const std::common_iterator<T1, T2> &a0,
         const std::common_iterator<T3, T4> &a1)
  requires(std::sentinel_for<T2, T3> &&
           std::equality_comparable_with<T1, T3>) &&
          (std::sentinel_for<T4, T1>)
{
  return operator==(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f23(const std::common_iterator<T1, T2> &a0,
         const std::common_iterator<T3, T4> &a1)
  requires(std::sentinel_for<T2, T3>) && (std::sentinel_for<T4, T1>)
{
  return operator==(a0, a1);
}

template <typename T1, typename T2>
using t2 = typename std::common_iterator<T1, T2>::__arrow_proxy;

template <typename T1, typename T2>
const std::iter_value_t<T1> *
f24(const typename std::common_iterator<T1, T2>::__arrow_proxy &a0) {
  return a0.operator->();
}

template <typename T1, typename T2>
using t3 = typename std::common_iterator<T1, T2>::__postfix_proxy;

template <typename T1, typename T2>
const std::iter_value_t<T1> &
f25(const typename std::common_iterator<T1, T2>::__postfix_proxy &a0) {
  return a0.operator*();
}
