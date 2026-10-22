#include "../common/init.h"
#include "../common/traits.h"
#include <queue>
#include <utility>

template <typename T1, typename T2>
bool f26(const std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f27(const std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
bool f28(const std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f29(const std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f30(const std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f31(const std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2> std::queue<T1, T2> f32() {
  return std::queue<T1, T2>();
}

template <typename T1, typename T2>
std::queue<T1, T2> f33(const std::queue<T1, T2> &a0) {
  return std::queue<T1, T2>(a0);
}

template <typename T1, typename T2>
std::queue<T1, T2> &f34(std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::queue<T1, T2> f35(std::queue<T1, T2> &&a0) {
  return std::queue<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::queue<T1, T2> &f36(std::queue<T1, T2> &a0, std::queue<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::queue<T1, T2> f37(const typename std::queue<T1, T2>::container_type &a0) {
  return std::queue<T1, T2>(a0);
}

template <typename T1, typename T2>
std::queue<T1, T2> f38(typename std::queue<T1, T2>::container_type &&a0) {
  return std::queue<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<std::uses_allocator<T2, T3>::value, int>::type
              T4 = 0>
std::queue<T1, T2> f39(const T3 &a0) {
  return std::queue<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<std::uses_allocator<T2, T3>::value, int>::type
              T4 = 0>
std::queue<T1, T2> f40(const std::queue<T1, T2> &a0, const T3 &a1) {
  return std::queue<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<std::uses_allocator<T2, T3>::value, int>::type
              T4 = 0>
std::queue<T1, T2> f41(const typename std::queue<T1, T2>::container_type &a0,
                       const T3 &a1) {
  return std::queue<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<std::uses_allocator<T2, T3>::value, int>::type
              T4 = 0>
std::queue<T1, T2> f42(typename std::queue<T1, T2>::container_type &&a0,
                       const T3 &a1) {
  return std::queue<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<std::uses_allocator<T2, T3>::value, int>::type
              T4 = 0>
std::queue<T1, T2> f43(std::queue<T1, T2> &&a0, const T3 &a1) {
  return std::queue<T1, T2>(std::move(a0), a1);
}
