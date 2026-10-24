#include <algorithm>
#include <utility>

template <typename T1> bool f1(T1 a0, T1 a1) {
  return std::is_heap<T1>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> bool f2(T1 a0, T1 a1, T2 a2) {
  return std::is_heap<T1, T2>(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1> T1 f3(T1 a0, T1 a1) {
  return std::is_heap_until<T1>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> T1 f4(T1 a0, T1 a1, T2 a2) {
  return std::is_heap_until<T1, T2>(std::move(a0), std::move(a1),
                                    std::move(a2));
}

template <typename T1> void f5(T1 a0, T1 a1) {
  return std::make_heap<T1>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> void f6(T1 a0, T1 a1, T2 a2) {
  return std::make_heap<T1, T2>(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1> void f7(T1 a0, T1 a1) {
  return std::pop_heap<T1>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> void f8(T1 a0, T1 a1, T2 a2) {
  return std::pop_heap<T1, T2>(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1> void f9(T1 a0, T1 a1) {
  return std::push_heap<T1>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> void f10(T1 a0, T1 a1, T2 a2) {
  return std::push_heap<T1, T2>(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1> void f11(T1 a0, T1 a1) {
  return std::sort_heap<T1>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> void f12(T1 a0, T1 a1, T2 a2) {
  return std::sort_heap<T1, T2>(std::move(a0), std::move(a1), std::move(a2));
}
