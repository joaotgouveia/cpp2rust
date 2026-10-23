#include <queue>
#include <utility>

template <typename T1, typename T2>
bool f26(const std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f27(const std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_swappable_v<T2>, int> T3 = 0>
void f28(std::queue<T1, T2> &a0, std::queue<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__is_swappable_v<T2> && std::__is_swappable_v<T3>, int>
              T4 = 0>
void f29(std::priority_queue<T1, T2, T3> &a0,
         std::priority_queue<T1, T2, T3> &a1) {
  return std::swap<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2>
bool f30(const std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f31(const std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f32(const std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f33(const std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1, typename T2> std::queue<T1, T2> f34() {
  return std::queue<T1, T2>();
}

template <typename T1, typename T2>
std::queue<T1, T2> f35(const std::queue<T1, T2> &a0) {
  return std::queue<T1, T2>(a0);
}

template <typename T1, typename T2>
std::queue<T1, T2> &f36(std::queue<T1, T2> &a0, const std::queue<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::queue<T1, T2> f37(std::queue<T1, T2> &&a0) {
  return std::queue<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::queue<T1, T2> &f38(std::queue<T1, T2> &a0, std::queue<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::uses_allocator<T2, T3>::value, int> T4 = 0>
std::queue<T1, T2> f39(const T3 &a0) {
  return std::queue<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::uses_allocator<T2, T3>::value, int> T4 = 0>
std::queue<T1, T2> f40(const std::queue<T1, T2> &a0, const T3 &a1) {
  return std::queue<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::uses_allocator<T2, T3>::value, int> T4 = 0>
std::queue<T1, T2> f41(const T2 &a0, const T3 &a1) {
  return std::queue<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::uses_allocator<T2, T3>::value, int> T4 = 0>
std::queue<T1, T2> f42(T2 &&a0, const T3 &a1) {
  return std::queue<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::uses_allocator<T2, T3>::value, int> T4 = 0>
std::queue<T1, T2> f43(std::queue<T1, T2> &&a0, const T3 &a1) {
  return std::queue<T1, T2>(std::move(a0), a1);
}
