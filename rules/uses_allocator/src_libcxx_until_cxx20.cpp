#include <future>
#include <memory>
#include <queue>
#include <stack>
#include <utility>

template <
    typename T1, typename T2, typename T3,
    typename std::enable_if<(std::__has_allocator_type<T2>::value == false),
                            int>::type T4 = 0>
bool f22(const std::uses_allocator<std::queue<T1, T2>, T3> &a0) {
  return static_cast<bool>(a0);
}

template <
    typename T1, typename T2, typename T3,
    typename std::enable_if<(std::__has_allocator_type<T2>::value == false),
                            int>::type T4 = 0>
bool f23(const std::uses_allocator<std::stack<T1, T2>, T3> &a0) {
  return static_cast<bool>(a0);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename std::enable_if<(std::__has_allocator_type<T2>::value == false),
                            int>::type T5 = 0>
bool f24(const std::uses_allocator<std::priority_queue<T1, T2, T3>, T4> &a0) {
  return static_cast<bool>(a0);
}

template <
    typename T1, typename T2,
    typename std::enable_if<(std::__has_allocator_type<T1>::value == false),
                            int>::type T3 = 0>
bool f25(const std::uses_allocator<T1, T2> &a0) {
  return static_cast<bool>(a0);
}
