#include <exception>
#include <utility>

template <typename T1,
          std::__enable_if_t<
              !std::__can_dynamic_cast_v<T1, std::nested_exception>, int>
              T2 = 0>
void f8(const T1 &a0) {
  return std::rethrow_if_nested<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<
              std::__can_dynamic_cast_v<T1, std::nested_exception>, int>
              T2 = 0>
void f9(const T1 &a0) {
  return std::rethrow_if_nested<T1>(a0);
}
