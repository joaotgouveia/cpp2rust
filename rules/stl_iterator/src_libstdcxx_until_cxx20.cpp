#include <algorithm>
#include <utility>

template <typename T1>
std::insert_iterator<T1> f21(T1 &a0, typename T1::iterator a1) {
  return std::inserter<T1>(a0, a1);
}

template <typename T1>
bool f22(const std::move_iterator<T1> &a0, const std::move_iterator<T1> &a1) {
  return a0 != a1;
}

template <typename T1>
bool f23(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T1> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f24(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return a0 != a1;
}

template <typename T1>
bool f25(const std::move_iterator<T1> &a0, const std::move_iterator<T1> &a1) {
  return a0 < a1;
}

template <typename T1>
bool f26(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T1> &a1) {
  return a0 < a1;
}

template <typename T1>
bool f27(const std::move_iterator<T1> &a0, const std::move_iterator<T1> &a1) {
  return a0 <= a1;
}

template <typename T1>
bool f28(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T1> &a1) {
  return a0 <= a1;
}

template <typename T1>
bool f29(const std::move_iterator<T1> &a0, const std::move_iterator<T1> &a1) {
  return a0 > a1;
}

template <typename T1>
bool f30(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T1> &a1) {
  return a0 > a1;
}

template <typename T1>
bool f31(const std::move_iterator<T1> &a0, const std::move_iterator<T1> &a1) {
  return a0 >= a1;
}

template <typename T1>
bool f32(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T1> &a1) {
  return a0 >= a1;
}
