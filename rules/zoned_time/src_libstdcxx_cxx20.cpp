#include <chrono>
#include <string_view>
#include <type_traits>
#include <utility>

template <typename T1, typename T2> using t1 = std::chrono::zoned_time<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::chrono::zoned_time<T1, T2>::duration;

template <typename T1, typename T2> std::chrono::zoned_time<T1, T2> f1() {
  return std::chrono::zoned_time<T1, T2>();
}

template <typename T1, typename T2>
std::chrono::zoned_time<T1, T2> f2(const std::chrono::zoned_time<T1, T2> &a0) {
  return std::chrono::zoned_time<T1, T2>(a0);
}

template <typename T1, typename T2>
std::chrono::zoned_time<T1, T2> &f3(std::chrono::zoned_time<T1, T2> &a0,
                                    const std::chrono::zoned_time<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::chrono::zoned_time<T1, T2> f4(const std::chrono::sys_time<T1> &a0) {
  return std::chrono::zoned_time<T1, T2>(a0);
}

template <typename T1, typename T2> std::chrono::zoned_time<T1, T2> f5(T2 a0) {
  return std::chrono::zoned_time<T1, T2>(a0);
}

template <typename T1, typename T2>
std::chrono::zoned_time<T1, T2> f6(std::string_view a0) {
  return std::chrono::zoned_time<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::chrono::zoned_time<T1, T2> f7(T2 a0, const std::chrono::sys_time<T1> &a1) {
  return std::chrono::zoned_time<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::chrono::zoned_time<T1, T2> f8(std::string_view a0,
                                   const std::chrono::sys_time<T1> &a1) {
  return std::chrono::zoned_time<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::chrono::zoned_time<T1, T2> f9(T2 a0,
                                   const std::chrono::local_time<T1> &a1) {
  return std::chrono::zoned_time<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::chrono::zoned_time<T1, T2> f10(std::string_view a0,
                                    const std::chrono::local_time<T1> &a1) {
  return std::chrono::zoned_time<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::chrono::zoned_time<T1, T2>
f11(T2 a0, const std::chrono::local_time<T1> &a1, std::chrono::choose a2) {
  return std::chrono::zoned_time<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
std::chrono::zoned_time<T1, T2> f12(std::string_view a0,
                                    const std::chrono::local_time<T1> &a1,
                                    std::chrono::choose a2) {
  return std::chrono::zoned_time<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2>
std::chrono::zoned_time<T1, T2> &f13(std::chrono::zoned_time<T1, T2> &a0,
                                     const std::chrono::sys_time<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::chrono::zoned_time<T1, T2> &f14(std::chrono::zoned_time<T1, T2> &a0,
                                     const std::chrono::local_time<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f15(const std::chrono::zoned_time<T1, T2> &a0) {
  return static_cast<
      std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>>(a0);
}

template <typename T1, typename T2>
std::chrono::local_time<std::common_type_t<T1, std::chrono::seconds>>
f16(const std::chrono::zoned_time<T1, T2> &a0) {
  return static_cast<
      std::chrono::local_time<std::common_type_t<T1, std::chrono::seconds>>>(
      a0);
}

template <typename T1, typename T2>
T2 f17(const std::chrono::zoned_time<T1, T2> &a0) {
  return a0.get_time_zone();
}

template <typename T1, typename T2>
std::chrono::local_time<std::common_type_t<T1, std::chrono::seconds>>
f18(const std::chrono::zoned_time<T1, T2> &a0) {
  return a0.get_local_time();
}

template <typename T1, typename T2>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f19(const std::chrono::zoned_time<T1, T2> &a0) {
  return a0.get_sys_time();
}

template <typename T1, typename T2>
std::chrono::sys_info f20(const std::chrono::zoned_time<T1, T2> &a0) {
  return a0.get_info();
}

template <typename T1, typename T2, typename T3>
std::chrono::zoned_time<T1, T2> f21(const std::chrono::zoned_time<T3, T2> &a0)
  requires(std::is_convertible_v<std::chrono::sys_time<T3>,
                                 std::chrono::sys_time<T1>>)
{
  return std::chrono::zoned_time<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::zoned_time<T1, T2> f22(T2 a0,
                                    const std::chrono::zoned_time<T3, T4> &a1)
  requires(std::is_convertible_v<std::chrono::sys_time<T3>,
                                 std::chrono::sys_time<T1>>)
{
  return std::chrono::zoned_time<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::zoned_time<T1, T2>
f23(T2 a0, const std::chrono::zoned_time<T3, T4> &a1, std::chrono::choose a2)
  requires(std::is_convertible_v<std::chrono::sys_time<T3>,
                                 std::chrono::sys_time<T1>>)
{
  return std::chrono::zoned_time<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::zoned_time<T1, T2> f24(std::string_view a0,
                                    const std::chrono::zoned_time<T3, T4> &a1) {
  return std::chrono::zoned_time<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::zoned_time<T1, T2> f25(std::string_view a0,
                                    const std::chrono::zoned_time<T3, T4> &a1,
                                    std::chrono::choose a2) {
  return std::chrono::zoned_time<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2>
bool f26(const std::chrono::zoned_time<T1, T2> &a0,
         const std::chrono::zoned_time<T1, T2> &a1) {
  return operator==(a0, a1);
}
