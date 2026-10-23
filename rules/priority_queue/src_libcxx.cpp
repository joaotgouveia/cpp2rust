#include <queue>
#include <utility>

template <typename T1, typename T2, typename T3>
std::priority_queue<T1, T2, T3> f33() {
  return std::priority_queue<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::priority_queue<T1, T2, T3> f34(const T3 &a0) {
  return std::priority_queue<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::__has_input_iterator_category<T4>::value, int>
              T5 = 0>
std::priority_queue<T1, T2, T3> f35(T4 a0, T4 a1, const T3 &a2) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::__has_input_iterator_category<T4>::value, int>
              T5 = 0>
std::priority_queue<T1, T2, T3> f36(T4 a0, T4 a1, const T3 &a2, const T2 &a3) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::__has_input_iterator_category<T4>::value, int>
              T5 = 0>
std::priority_queue<T1, T2, T3> f37(T4 a0, T4 a1, const T3 &a2, T2 &&a3) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2, std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::uses_allocator<T2, T4>::value, int> T5 = 0>
std::priority_queue<T1, T2, T3> f38(const T4 &a0) {
  return std::priority_queue<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::uses_allocator<T2, T4>::value, int> T5 = 0>
std::priority_queue<T1, T2, T3> f39(const T3 &a0, const T4 &a1) {
  return std::priority_queue<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::uses_allocator<T2, T4>::value, int> T5 = 0>
std::priority_queue<T1, T2, T3> f40(const T3 &a0, const T2 &a1, const T4 &a2) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::uses_allocator<T2, T4>::value, int> T5 = 0>
std::priority_queue<T1, T2, T3> f41(const std::priority_queue<T1, T2, T3> &a0,
                                    const T4 &a1) {
  return std::priority_queue<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::uses_allocator<T2, T4>::value, int> T5 = 0>
std::priority_queue<T1, T2, T3> f42(const T3 &a0, T2 &&a1, const T4 &a2) {
  return std::priority_queue<T1, T2, T3>(a0, std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::uses_allocator<T2, T4>::value, int> T5 = 0>
std::priority_queue<T1, T2, T3> f43(std::priority_queue<T1, T2, T3> &&a0,
                                    const T4 &a1) {
  return std::priority_queue<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          std::__enable_if_t<std::__has_input_iterator_category<T4>::value &&
                                 std::uses_allocator<T2, T5>::value,
                             int>
              T6 = 0>
std::priority_queue<T1, T2, T3> f44(T4 a0, T4 a1, const T5 &a2) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          std::__enable_if_t<std::__has_input_iterator_category<T4>::value &&
                                 std::uses_allocator<T2, T5>::value,
                             int>
              T6 = 0>
std::priority_queue<T1, T2, T3> f45(T4 a0, T4 a1, const T3 &a2, const T5 &a3) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          std::__enable_if_t<std::__has_input_iterator_category<T4>::value &&
                                 std::uses_allocator<T2, T5>::value,
                             int>
              T6 = 0>
std::priority_queue<T1, T2, T3> f46(T4 a0, T4 a1, const T3 &a2, const T2 &a3,
                                    const T5 &a4) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          std::__enable_if_t<std::__has_input_iterator_category<T4>::value &&
                                 std::uses_allocator<T2, T5>::value,
                             int>
              T6 = 0>
std::priority_queue<T1, T2, T3> f47(T4 a0, T4 a1, const T3 &a2, T2 &&a3,
                                    const T5 &a4) {
  return std::priority_queue<T1, T2, T3>(a0, a1, a2, std::move(a3), a4);
}
