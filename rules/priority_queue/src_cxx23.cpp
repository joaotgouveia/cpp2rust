#include <algorithm>
#include <queue>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::priority_queue<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = typename std::priority_queue<T1, T2, T3>::value_type;

template <typename T1, typename T2, typename T3>
using t3 = typename std::priority_queue<T1, T2, T3>::reference;

template <typename T1, typename T2, typename T3>
using t4 = typename std::priority_queue<T1, T2, T3>::const_reference;

template <typename T1, typename T2, typename T3>
using t5 = typename std::priority_queue<T1, T2, T3>::size_type;

template <typename T1, typename T2, typename T3>
using t6 = typename std::priority_queue<T1, T2, T3>::container_type;

template <typename T1, typename T2, typename T3>
using t7 = typename std::priority_queue<T1, T2, T3>::value_compare;

template <typename T1, typename T2, typename T3>
std::priority_queue<T1, T2, T3> f1(const T3 &a0, const T2 &a1) {
  return std::priority_queue<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::priority_queue<T1, T2, T3> f2(const T3 &a0, T2 &&a1) {
  return std::priority_queue<T1, T2, T3>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::priority_queue<T1, T2, T3> f3(const std::priority_queue<T1, T2, T3> &a0) {
  return std::priority_queue<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::priority_queue<T1, T2, T3> &f4(std::priority_queue<T1, T2, T3> &a0,
                                    const std::priority_queue<T1, T2, T3> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::priority_queue<T1, T2, T3> f5(std::priority_queue<T1, T2, T3> &&a0) {
  return std::priority_queue<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::priority_queue<T1, T2, T3> &f6(std::priority_queue<T1, T2, T3> &a0,
                                    std::priority_queue<T1, T2, T3> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
bool f7(const std::priority_queue<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
typename std::priority_queue<T1, T2, T3>::size_type
f8(const std::priority_queue<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3>
typename std::priority_queue<T1, T2, T3>::const_reference
f9(const std::priority_queue<T1, T2, T3> &a0) {
  return a0.top();
}

template <typename T1, typename T2, typename T3>
void f10(std::priority_queue<T1, T2, T3> &a0,
         const typename std::priority_queue<T1, T2, T3>::value_type &a1) {
  return a0.push(a1);
}

template <typename T1, typename T2, typename T3>
void f11(std::priority_queue<T1, T2, T3> &a0,
         typename std::priority_queue<T1, T2, T3>::value_type &&a1) {
  return a0.push(std::move(a1));
}

template <typename T1, typename T2, typename T3>
void f12(std::priority_queue<T1, T2, T3> &a0) {
  return a0.pop();
}

template <typename T1, typename T2, typename T3>
void f13(std::priority_queue<T1, T2, T3> &a0,
         std::priority_queue<T1, T2, T3> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4 = T2,
          typename T5 = typename std::enable_if<std::integral_constant<
              bool, std::is_default_constructible<T3>::value &&
                        std::is_default_constructible<T2>::value>::value>::type>
std::priority_queue<T1, T2, T3> f14() {
  return std::priority_queue<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 =
              typename std::enable_if<std::uses_allocator<T2, T4>::value>::type>
std::priority_queue<T1, T2, T3> f15(const T4 &a0)
  requires(!(std::is_same<T4, std::priority_queue<T1, T2, T3>>::value))
{
  return std::priority_queue<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 =
              typename std::enable_if<std::uses_allocator<T2, T4>::value>::type>
std::priority_queue<T1, T2, T3> f16(const T3 &a0, const T4 &a1)
  requires(!(std::is_same<T4, T2>::value))
{
  return std::priority_queue<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 =
              typename std::enable_if<std::uses_allocator<T2, T4>::value>::type>
std::priority_queue<T1, T2, T3> f17(const T3 &a0, const T2 &a1, const T4 &a2) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 =
              typename std::enable_if<std::uses_allocator<T2, T4>::value>::type>
std::priority_queue<T1, T2, T3> f18(const T3 &a0, T2 &&a1, const T4 &a2) {
  return std::priority_queue<T1, T2, T3>(a0, std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 =
              typename std::enable_if<std::uses_allocator<T2, T4>::value>::type>
std::priority_queue<T1, T2, T3> f19(const std::priority_queue<T1, T2, T3> &a0,
                                    const T4 &a1) {
  return std::priority_queue<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 =
              typename std::enable_if<std::uses_allocator<T2, T4>::value>::type>
std::priority_queue<T1, T2, T3> f20(std::priority_queue<T1, T2, T3> &&a0,
                                    const T4 &a1) {
  return std::priority_queue<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T4>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
std::priority_queue<T1, T2, T3> f21(T4 a0, T4 a1, const T3 &a2) {
  return std::priority_queue<T1, T2, T3>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T4>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
std::priority_queue<T1, T2, T3> f22(T4 a0, T4 a1, const T3 &a2, const T2 &a3) {
  return std::priority_queue<T1, T2, T3>(std::move(a0), std::move(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T4>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
std::priority_queue<T1, T2, T3> f23(T4 a0, T4 a1, const T3 &a2, T2 &&a3) {
  return std::priority_queue<T1, T2, T3>(std::move(a0), std::move(a1), a2,
                                         std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T4>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type,
          typename T7 =
              typename std::enable_if<std::uses_allocator<T2, T5>::value>::type>
std::priority_queue<T1, T2, T3> f24(T4 a0, T4 a1, const T5 &a2) {
  return std::priority_queue<T1, T2, T3>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T4>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type,
          typename T7 =
              typename std::enable_if<std::uses_allocator<T2, T5>::value>::type>
std::priority_queue<T1, T2, T3> f25(T4 a0, T4 a1, const T3 &a2, const T5 &a3) {
  return std::priority_queue<T1, T2, T3>(std::move(a0), std::move(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T4>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type,
          typename T7 =
              typename std::enable_if<std::uses_allocator<T2, T5>::value>::type>
std::priority_queue<T1, T2, T3> f26(T4 a0, T4 a1, const T3 &a2, const T2 &a3,
                                    const T5 &a4) {
  return std::priority_queue<T1, T2, T3>(std::move(a0), std::move(a1), a2, a3,
                                         a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 =
              typename std::enable_if<std::uses_allocator<T2, T5>::value>::type>
std::priority_queue<T1, T2, T3> f27(T4 a0, T4 a1, const T3 &a2, T2 &&a3,
                                    const T5 &a4) {
  return std::priority_queue<T1, T2, T3>(std::move(a0), std::move(a1), a2,
                                         std::move(a3), a4);
}

template <typename T1, typename T2, typename T3, typename T4>
std::priority_queue<T1, T2, T3> f28(std::from_range_t a0, T4 &&a1,
                                    const T3 &a2) {
  return std::priority_queue<T1, T2, T3>(std::move(a0), std::forward<T4>(a1),
                                         a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::priority_queue<T1, T2, T3> f29(std::from_range_t a0, T4 &&a1, const T3 &a2,
                                    const T5 &a3) {
  return std::priority_queue<T1, T2, T3>(std::move(a0), std::forward<T4>(a1),
                                         a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::priority_queue<T1, T2, T3> f30(std::from_range_t a0, T4 &&a1,
                                    const T5 &a2) {
  return std::priority_queue<T1, T2, T3>(std::move(a0), std::forward<T4>(a1),
                                         a2);
}

template <typename T1, typename T2, typename T3, typename... T4>
void f31(std::priority_queue<T1, T2, T3> &a0, T4 &&...a1) {
  return a0.template emplace<T4...>(std::forward<T4>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4>
void f32(std::priority_queue<T1, T2, T3> &a0, T4 &&a1) {
  return a0.template push_range<T4>(std::forward<T4>(a1));
}
