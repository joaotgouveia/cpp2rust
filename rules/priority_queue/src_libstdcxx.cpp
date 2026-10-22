#include "../common/init.h"
#include "../common/traits.h"
#include <queue>
#include <utility>

template <typename T1, typename T2, typename T3>
std::priority_queue<T1, T2, T3> f20(const T3 &a0, const T2 &a1) {
  return std::priority_queue<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::priority_queue<T1, T2, T3> f21(const T3 &a0, T2 &&a1) {
  return std::priority_queue<T1, T2, T3>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::priority_queue<T1, T2, T3> f22(const T3 &a0, const T4 &a1) {
  return std::priority_queue<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::priority_queue<T1, T2, T3> f23(const T3 &a0, const T2 &a1, const T4 &a2) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::priority_queue<T1, T2, T3> f24(const T3 &a0, T2 &&a1, const T4 &a2) {
  return std::priority_queue<T1, T2, T3>(a0, std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::priority_queue<T1, T2, T3> f25(T4 a0, T4 a1, const T3 &a2) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::priority_queue<T1, T2, T3> f26(T4 a0, T4 a1, const T3 &a2, const T2 &a3) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::priority_queue<T1, T2, T3> f27(T4 a0, T4 a1, const T3 &a2, T2 &&a3) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2, std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::priority_queue<T1, T2, T3> f28(T4 a0, T4 a1, const T3 &a2, const T5 &a3) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::priority_queue<T1, T2, T3> f29(T4 a0, T4 a1, const T3 &a2, const T2 &a3,
                                    const T5 &a4) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::priority_queue<T1, T2, T3> f30(T4 a0, T4 a1, const T3 &a2, T2 &&a3,
                                    const T5 &a4) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2, std::move(a3), a4);
}
