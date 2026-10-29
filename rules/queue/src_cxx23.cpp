#include <algorithm>
#include <queue>
#include <utility>

template <typename T1, typename T2> using t1 = std::queue<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::queue<T1, T2>::value_type;

template <typename T1, typename T2>
using t3 = typename std::queue<T1, T2>::reference;

template <typename T1, typename T2>
using t4 = typename std::queue<T1, T2>::const_reference;

template <typename T1, typename T2>
using t5 = typename std::queue<T1, T2>::size_type;

template <typename T1, typename T2>
using t6 = typename std::queue<T1, T2>::container_type;

template <typename T1, typename T2> std::queue<T1, T2> f1(const T2 &a0) {
  return std::queue<T1, T2>(a0);
}

template <typename T1, typename T2> std::queue<T1, T2> f2(T2 &&a0) {
  return std::queue<T1, T2>(std::move(a0));
}

template <typename T1, typename T2> bool f3(const std::queue<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
typename std::queue<T1, T2>::size_type f4(const std::queue<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
typename std::queue<T1, T2>::reference f5(std::queue<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::queue<T1, T2>::const_reference f6(const std::queue<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::queue<T1, T2>::reference f7(std::queue<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::queue<T1, T2>::const_reference f8(const std::queue<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
void f9(std::queue<T1, T2> &a0,
        const typename std::queue<T1, T2>::value_type &a1) {
  return a0.push(a1);
}

template <typename T1, typename T2>
void f10(std::queue<T1, T2> &a0, typename std::queue<T1, T2>::value_type &&a1) {
  return a0.push(std::move(a1));
}

template <typename T1, typename T2> void f11(std::queue<T1, T2> &a0) {
  return a0.pop();
}

template <typename T1, typename T2>
void f12(std::queue<T1, T2> &a0, std::queue<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3 = T2,
          typename T4 = typename std::enable_if<
              std::is_default_constructible<T2>::value>::type>
std::queue<T1, T2> f13() {
  return std::queue<T1, T2>();
}

template <typename T1, typename T2, typename T3,
          typename T4 =
              typename std::enable_if<std::uses_allocator<T2, T3>::value>::type>
std::queue<T1, T2> f14(const T3 &a0)
  requires(!(std::is_same<T3, T2>::value))
{
  return std::queue<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3,
          typename T4 =
              typename std::enable_if<std::uses_allocator<T2, T3>::value>::type>
std::queue<T1, T2> f15(const T2 &a0, const T3 &a1) {
  return std::queue<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          typename T4 =
              typename std::enable_if<std::uses_allocator<T2, T3>::value>::type>
std::queue<T1, T2> f16(T2 &&a0, const T3 &a1) {
  return std::queue<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3,
          typename T4 =
              typename std::enable_if<std::uses_allocator<T2, T3>::value>::type>
std::queue<T1, T2> f17(const std::queue<T1, T2> &a0, const T3 &a1) {
  return std::queue<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          typename T4 =
              typename std::enable_if<std::uses_allocator<T2, T3>::value>::type>
std::queue<T1, T2> f18(std::queue<T1, T2> &&a0, const T3 &a1) {
  return std::queue<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::queue<T1, T2> f19(T3 a0, T3 a1) {
  return std::queue<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T6 =
              typename std::enable_if<std::uses_allocator<T2, T4>::value>::type>
std::queue<T1, T2> f20(T3 a0, T3 a1, const T4 &a2) {
  return std::queue<T1, T2>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
std::queue<T1, T2> f21(std::from_range_t a0, T3 &&a1) {
  return std::queue<T1, T2>(std::move(a0), std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::queue<T1, T2> f22(std::from_range_t a0, T3 &&a1, const T4 &a2) {
  return std::queue<T1, T2>(std::move(a0), std::forward<T3>(a1), a2);
}

template <typename T1, typename T2, typename... T3>
decltype(auto) f23(std::queue<T1, T2> &a0, T3 &&...a1) {
  return a0.template emplace<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename T3>
void f24(std::queue<T1, T2> &a0, T3 &&a1) {
  return a0.template push_range<T3>(std::forward<T3>(a1));
}
