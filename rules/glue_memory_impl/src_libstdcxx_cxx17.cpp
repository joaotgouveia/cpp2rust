#include "../common/init.h"
#include "../common/traits.h"
#include <execution>
#include <utility>

template <typename T1, typename T2>
typename std::enable_if<
    __pstl::execution::is_execution_policy<typename std::remove_cv<
        typename std::remove_reference<T1>::type>::type>::value,
    void>::type
f1(T1 &&a0, T2 a1, T2 a2) {
  return std::destroy<T1, T2>(std::forward<T1>(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<
    __pstl::execution::is_execution_policy<typename std::remove_cv<
        typename std::remove_reference<T1>::type>::type>::value,
    T2>::type
f2(T1 &&a0, T2 a1, T3 a2) {
  return std::destroy_n<T1, T2, T3>(std::forward<T1>(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<
    __pstl::execution::is_execution_policy<typename std::remove_cv<
        typename std::remove_reference<T1>::type>::type>::value,
    T3>::type
f3(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::uninitialized_copy<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<
    __pstl::execution::is_execution_policy<typename std::remove_cv<
        typename std::remove_reference<T1>::type>::type>::value,
    T4>::type
f4(T1 &&a0, T2 a1, T3 a2, T4 a3) {
  return std::uninitialized_copy_n<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2,
                                                   a3);
}

template <typename T1, typename T2>
typename std::enable_if<
    __pstl::execution::is_execution_policy<typename std::remove_cv<
        typename std::remove_reference<T1>::type>::type>::value,
    void>::type
f5(T1 &&a0, T2 a1, T2 a2) {
  return std::uninitialized_default_construct<T1, T2>(std::forward<T1>(a0), a1,
                                                      a2);
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<
    __pstl::execution::is_execution_policy<typename std::remove_cv<
        typename std::remove_reference<T1>::type>::type>::value,
    T2>::type
f6(T1 &&a0, T2 a1, T3 a2) {
  return std::uninitialized_default_construct_n<T1, T2, T3>(
      std::forward<T1>(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<
    __pstl::execution::is_execution_policy<typename std::remove_cv<
        typename std::remove_reference<T1>::type>::type>::value,
    void>::type
f7(T1 &&a0, T2 a1, T2 a2, const T3 &a3) {
  return std::uninitialized_fill<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<
    __pstl::execution::is_execution_policy<typename std::remove_cv<
        typename std::remove_reference<T1>::type>::type>::value,
    T2>::type
f8(T1 &&a0, T2 a1, T3 a2, const T4 &a3) {
  return std::uninitialized_fill_n<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2,
                                                   a3);
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<
    __pstl::execution::is_execution_policy<typename std::remove_cv<
        typename std::remove_reference<T1>::type>::type>::value,
    T3>::type
f9(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::uninitialized_move<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<
    __pstl::execution::is_execution_policy<typename std::remove_cv<
        typename std::remove_reference<T1>::type>::type>::value,
    T4>::type
f10(T1 &&a0, T2 a1, T3 a2, T4 a3) {
  return std::uninitialized_move_n<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2,
                                                   a3);
}

template <typename T1, typename T2>
typename std::enable_if<
    __pstl::execution::is_execution_policy<typename std::remove_cv<
        typename std::remove_reference<T1>::type>::type>::value,
    void>::type
f11(T1 &&a0, T2 a1, T2 a2) {
  return std::uninitialized_value_construct<T1, T2>(std::forward<T1>(a0), a1,
                                                    a2);
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<
    __pstl::execution::is_execution_policy<typename std::remove_cv<
        typename std::remove_reference<T1>::type>::type>::value,
    T2>::type
f12(T1 &&a0, T2 a1, T3 a2) {
  return std::uninitialized_value_construct_n<T1, T2, T3>(std::forward<T1>(a0),
                                                          a1, a2);
}
