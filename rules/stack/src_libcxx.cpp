#include "../common/init.h"
#include "../common/traits.h"
#include <stack>
#include <utility>

template <typename T1, typename T2>
bool f24(const std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f25(const std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
void f26(std::stack<T1, T2> &a0, std::stack<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f27(const std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f28(const std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f29(const std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f30(const std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2>
std::stack<T1, T2> f31(const std::stack<T1, T2> &a0) {
  return std::stack<T1, T2>(a0);
}

template <typename T1, typename T2>
std::stack<T1, T2> &f32(std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::stack<T1, T2> f33(std::stack<T1, T2> &&a0) {
  return std::stack<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::stack<T1, T2> &f34(std::stack<T1, T2> &a0, std::stack<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::stack<T1, T2> f35(typename std::stack<T1, T2>::container_type &&a0) {
  return std::stack<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::stack<T1, T2> f36(const typename std::stack<T1, T2>::container_type &a0) {
  return std::stack<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3>
std::stack<T1, T2> f37(const T3 &a0, void *a1) {
  return std::stack<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::stack<T1, T2> f38(const typename std::stack<T1, T2>::container_type &a0,
                       const T3 &a1, void *a2) {
  return std::stack<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::stack<T1, T2> f39(const std::stack<T1, T2> &a0, const T3 &a1, void *a2) {
  return std::stack<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::stack<T1, T2> f40(typename std::stack<T1, T2>::container_type &&a0,
                       const T3 &a1, void *a2) {
  return std::stack<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
std::stack<T1, T2> f41(std::stack<T1, T2> &&a0, const T3 &a1, void *a2) {
  return std::stack<T1, T2>(std::move(a0), a1, a2);
}
