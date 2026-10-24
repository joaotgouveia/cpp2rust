#include <algorithm>
#include <generator>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::generator<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = typename std::generator<T1, T2, T3>::yielded;

template <typename T1, typename T2, typename T3>
using t3 = typename std::generator<T1, T2, T3>::promise_type;

template <typename T1, typename T2, typename T3>
std::generator<T1, T2, T3> f1(std::generator<T1, T2, T3> &&a0) {
  return std::generator<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::generator<T1, T2, T3> &f2(std::generator<T1, T2, T3> &a0,
                               std::generator<T1, T2, T3> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
auto f3(std::generator<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
std::default_sentinel_t f4(const std::generator<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
bool f5(std::generator<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<std::generator<T1, T2, T3>> &&
           (!std::ranges::sized_range<std::generator<T1, T2, T3>>)) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
bool f6(std::generator<T1, T2, T3> &a0)
  requires(std::ranges::sized_range<std::generator<T1, T2, T3>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
bool f7(const std::generator<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<const std::generator<T1, T2, T3>> &&
           (!std::ranges::sized_range<const std::generator<T1, T2, T3>>)) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
bool f8(const std::generator<T1, T2, T3> &a0)
  requires(std::ranges::sized_range<const std::generator<T1, T2, T3>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
bool f9(std::generator<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3>
bool f10(const std::generator<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3>
auto f11(std::generator<T1, T2, T3> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::generator<T1, T2, T3>>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.data();
}

template <typename T1, typename T2, typename T3>
auto f12(const std::generator<T1, T2, T3> &a0)
  requires(std::ranges::range<const std::generator<T1, T2, T3>> &&
           std::contiguous_iterator<
               std::ranges::iterator_t<const std::generator<T1, T2, T3>>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.data();
}

template <typename T1, typename T2, typename T3>
auto f13(std::generator<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<std::generator<T1, T2, T3>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<std::generator<T1, T2, T3>>,
               std::ranges::iterator_t<std::generator<T1, T2, T3>>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.size();
}

template <typename T1, typename T2, typename T3>
auto f14(const std::generator<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<const std::generator<T1, T2, T3>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<const std::generator<T1, T2, T3>>,
               std::ranges::iterator_t<const std::generator<T1, T2, T3>>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.size();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f15(std::generator<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<std::generator<T1, T2, T3>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.front();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f16(const std::generator<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<const std::generator<T1, T2, T3>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.front();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f17(std::generator<T1, T2, T3> &a0)
  requires(std::ranges::bidirectional_range<std::generator<T1, T2, T3>> &&
           std::ranges::common_range<std::generator<T1, T2, T3>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.back();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f18(const std::generator<T1, T2, T3> &a0)
  requires(std::ranges::bidirectional_range<const std::generator<T1, T2, T3>> &&
           std::ranges::common_range<const std::generator<T1, T2, T3>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.back();
}

template <typename T1, typename T2, typename T3>
auto f19(std::generator<T1, T2, T3> &a0)
  requires(std::ranges::input_range<std::generator<T1, T2, T3>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
auto f20(const std::generator<T1, T2, T3> &a0)
  requires(std::ranges::input_range<const std::generator<T1, T2, T3>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
auto f21(std::generator<T1, T2, T3> &a0)
  requires(std::ranges::input_range<std::generator<T1, T2, T3>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
auto f22(const std::generator<T1, T2, T3> &a0)
  requires(std::ranges::input_range<const std::generator<T1, T2, T3>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.cend();
}

template <typename T1, typename T2, typename T3, typename T4>
decltype(auto) f23(std::generator<T1, T2, T3> &a0,
                   std::ranges::range_difference_t<T4> a1)
  requires(std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>) &&
          (std::ranges::random_access_range<T4>)
{
  return a0.template operator[]<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
decltype(auto) f24(const std::generator<T1, T2, T3> &a0,
                   std::ranges::range_difference_t<T4> a1)
  requires(std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>) &&
          (std::ranges::random_access_range<T4>)
{
  return a0.template operator[]<T4>(a1);
}

template <typename T1, typename T2> using t4 = std::pmr::generator<T1, T2>;
