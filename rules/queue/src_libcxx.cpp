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
void f28(std::queue<T1, T2> &a0, std::queue<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
void f29(std::priority_queue<T1, T2, T3> &a0,
         std::priority_queue<T1, T2, T3> &a1) {
  return std::swap<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2>
bool f30(const std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f31(const std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f32(const std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f33(const std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2>
std::queue<T1, T2> f34(const std::queue<T1, T2> &a0) {
  return std::queue<T1, T2>(a0);
}

template <typename T1, typename T2>
std::queue<T1, T2> &f35(std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::queue<T1, T2> f36(std::queue<T1, T2> &&a0) {
  return std::queue<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::queue<T1, T2> &f37(std::queue<T1, T2> &a0, std::queue<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::queue<T1, T2> f38(const typename std::queue<T1, T2>::container_type &a0) {
  return std::queue<T1, T2>(a0);
}

template <typename T1, typename T2>
std::queue<T1, T2> f39(typename std::queue<T1, T2>::container_type &&a0) {
  return std::queue<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::queue<T1, T2> f40(const typename std::queue<T1, T2>::container_type &a0,
                       const T3 &a1) {
  return std::queue<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::queue<T1, T2> f41(typename std::queue<T1, T2>::container_type &&a0,
                       const T3 &a1) {
  return std::queue<T1, T2>(std::move(a0), a1);
}
