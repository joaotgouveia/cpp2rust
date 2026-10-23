#include <stack>
#include <utility>

template <typename T1, typename T2>
bool f24(const std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f25(const std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f26(const std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f27(const std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f28(const std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f29(const std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1, typename T2> std::stack<T1, T2> f30() {
  return std::stack<T1, T2>();
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

template <typename T1, typename T2, typename T3>
std::stack<T1, T2>
f35(const T3 &a0,
    typename std::enable_if<std::uses_allocator<T2, T3>::value, void>::type
        *a1) {
  return std::stack<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::stack<T1, T2>
f36(const T2 &a0, const T3 &a1,
    typename std::enable_if<std::uses_allocator<T2, T3>::value, void>::type
        *a2) {
  return std::stack<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::stack<T1, T2>
f37(const std::stack<T1, T2> &a0, const T3 &a1,
    typename std::enable_if<std::uses_allocator<T2, T3>::value, void>::type
        *a2) {
  return std::stack<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::stack<T1, T2>
f38(T2 &&a0, const T3 &a1,
    typename std::enable_if<std::uses_allocator<T2, T3>::value, void>::type
        *a2) {
  return std::stack<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
std::stack<T1, T2>
f39(std::stack<T1, T2> &&a0, const T3 &a1,
    typename std::enable_if<std::uses_allocator<T2, T3>::value, void>::type
        *a2) {
  return std::stack<T1, T2>(std::move(a0), a1, a2);
}
