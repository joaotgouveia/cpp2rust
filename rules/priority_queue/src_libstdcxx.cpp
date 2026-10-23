#include <queue>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3>
std::priority_queue<T1, T2, T3> f14(const T3 &a0, const T2 &a1) {
  return std::priority_queue<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::priority_queue<T1, T2, T3> f15(const T3 &a0, T2 &&a1) {
  return std::priority_queue<T1, T2, T3>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4 = T2,
          typename T5 = typename std::enable_if<std::integral_constant<
              bool, std::is_default_constructible<T3>::value &&
                        std::is_default_constructible<T2>::value>::value>::type>
std::priority_queue<T1, T2, T3> f16() {
  return std::priority_queue<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 =
              typename std::enable_if<std::uses_allocator<T2, T4>::value>::type>
std::priority_queue<T1, T2, T3> f17(const T4 &a0) {
  return std::priority_queue<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 =
              typename std::enable_if<std::uses_allocator<T2, T4>::value>::type>
std::priority_queue<T1, T2, T3> f18(const T3 &a0, const T4 &a1) {
  return std::priority_queue<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 =
              typename std::enable_if<std::uses_allocator<T2, T4>::value>::type>
std::priority_queue<T1, T2, T3> f19(const T3 &a0, const T2 &a1, const T4 &a2) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 =
              typename std::enable_if<std::uses_allocator<T2, T4>::value>::type>
std::priority_queue<T1, T2, T3> f20(const T3 &a0, T2 &&a1, const T4 &a2) {
  return std::priority_queue<T1, T2, T3>(a0, std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 =
              typename std::enable_if<std::uses_allocator<T2, T4>::value>::type>
std::priority_queue<T1, T2, T3> f21(const std::priority_queue<T1, T2, T3> &a0,
                                    const T4 &a1) {
  return std::priority_queue<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 =
              typename std::enable_if<std::uses_allocator<T2, T4>::value>::type>
std::priority_queue<T1, T2, T3> f22(std::priority_queue<T1, T2, T3> &&a0,
                                    const T4 &a1) {
  return std::priority_queue<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T4>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
std::priority_queue<T1, T2, T3> f23(T4 a0, T4 a1, const T3 &a2) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T4>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
std::priority_queue<T1, T2, T3> f24(T4 a0, T4 a1, const T3 &a2, const T2 &a3) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T4>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
std::priority_queue<T1, T2, T3> f25(T4 a0, T4 a1, const T3 &a2, T2 &&a3) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2, std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T4>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type,
          typename T7 =
              typename std::enable_if<std::uses_allocator<T2, T5>::value>::type>
std::priority_queue<T1, T2, T3> f26(T4 a0, T4 a1, const T5 &a2) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T4>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type,
          typename T7 =
              typename std::enable_if<std::uses_allocator<T2, T5>::value>::type>
std::priority_queue<T1, T2, T3> f27(T4 a0, T4 a1, const T3 &a2, const T5 &a3) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T4>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type,
          typename T7 =
              typename std::enable_if<std::uses_allocator<T2, T5>::value>::type>
std::priority_queue<T1, T2, T3> f28(T4 a0, T4 a1, const T3 &a2, const T2 &a3,
                                    const T5 &a4) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 =
              typename std::enable_if<std::uses_allocator<T2, T5>::value>::type>
std::priority_queue<T1, T2, T3> f29(T4 a0, T4 a1, const T3 &a2, T2 &&a3,
                                    const T5 &a4) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2, std::move(a3), a4);
}
