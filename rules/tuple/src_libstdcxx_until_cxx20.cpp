#include <algorithm>
#include <tuple>
#include <utility>

template <typename T1, typename T2> using t3 = std::tuple<T1, T2>;

template <typename T1, typename T2>
std::tuple<T1, T2> f32(const std::tuple<T1, T2> &a0) {
  return std::tuple<T1, T2>(a0);
}

template <typename T1, typename T2>
std::tuple<T1, T2> f33(std::tuple<T1, T2> &&a0) {
  return std::tuple<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
void f34(std::tuple<T1, T2> &a0, std::tuple<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::_TupleConstraints<
                  !std::tuple<T1, T2>::template __is_alloc_arg<T3>(), T1,
                  T2>::template __is_implicitly_constructible<T3, T4>(),
              bool>
              T5 = true>
std::tuple<T1, T2> f35(T3 &&a0, T4 &&a1) {
  return std::tuple<T1, T2>(std::forward<T3>(a0), std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::_TupleConstraints<
                  !std::tuple<T1, T2>::template __is_alloc_arg<T3>(), T1,
                  T2>::template __is_explicitly_constructible<T3, T4>(),
              bool>
              T5 = false>
std::tuple<T1, T2> f36(T3 &&a0, T4 &&a1) {
  return std::tuple<T1, T2>(std::forward<T3>(a0), std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::_TupleConstraints<true, T1, T2>::
                                 template __is_implicitly_constructible<
                                     const T3 &, const T4 &>(),
                             bool>
              T5 = true,
          typename std::enable_if<(!(std::is_same<T3, T1>::value &&
                                     std::is_same<T4, T2>::value)),
                                  int>::type T6 = 0>
std::tuple<T1, T2> f37(const std::tuple<T3, T4> &a0) {
  return std::tuple<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::_TupleConstraints<true, T1, T2>::
                                 template __is_explicitly_constructible<
                                     const T3 &, const T4 &>(),
                             bool>
              T5 = false,
          typename std::enable_if<(!(std::is_same<T3, T1>::value &&
                                     std::is_same<T4, T2>::value)),
                                  int>::type T6 = 0>
std::tuple<T1, T2> f38(const std::tuple<T3, T4> &a0) {
  return std::tuple<T1, T2>(a0);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::_TupleConstraints<true, T1, T2>::
                           template __is_implicitly_constructible<T3, T4>(),
                       bool>
        T5 = true,
    typename std::enable_if<(!(std::is_same<T3, T1>::value &&
                               std::is_same<T4, T2>::value)),
                            int>::type T6 = 0>
std::tuple<T1, T2> f39(std::tuple<T3, T4> &&a0) {
  return std::tuple<T1, T2>(std::move(a0));
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::_TupleConstraints<true, T1, T2>::
                           template __is_explicitly_constructible<T3, T4>(),
                       bool>
        T5 = false,
    typename std::enable_if<(!(std::is_same<T3, T1>::value &&
                               std::is_same<T4, T2>::value)),
                            int>::type T6 = 0>
std::tuple<T1, T2> f40(std::tuple<T3, T4> &&a0) {
  return std::tuple<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::_TupleConstraints<true, T1, T2>::
                                 template __is_implicitly_constructible<
                                     const T3 &, const T4 &>(),
                             bool>
              T5 = true>
std::tuple<T1, T2> f41(const std::pair<T3, T4> &a0) {
  return std::tuple<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::_TupleConstraints<true, T1, T2>::
                                 template __is_explicitly_constructible<
                                     const T3 &, const T4 &>(),
                             bool>
              T5 = false>
std::tuple<T1, T2> f42(const std::pair<T3, T4> &a0) {
  return std::tuple<T1, T2>(a0);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::_TupleConstraints<true, T1, T2>::
                           template __is_implicitly_constructible<T3, T4>(),
                       bool>
        T5 = true>
std::tuple<T1, T2> f43(std::pair<T3, T4> &&a0) {
  return std::tuple<T1, T2>(std::move(a0));
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::_TupleConstraints<true, T1, T2>::
                           template __is_explicitly_constructible<T3, T4>(),
                       bool>
        T5 = false>
std::tuple<T1, T2> f44(std::pair<T3, T4> &&a0) {
  return std::tuple<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::_TupleConstraints<
                                 std::is_object<T3>::value, T1,
                                 T2>::__is_implicitly_default_constructible(),
                             bool>
              T4 = true>
std::tuple<T1, T2> f45(std::allocator_arg_t a0, const T3 &a1) {
  return std::tuple<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::_TupleConstraints<
                                 std::is_object<T3>::value, T1,
                                 T2>::__is_explicitly_default_constructible(),
                             bool>
              T4 = false>
std::tuple<T1, T2> f46(std::allocator_arg_t a0, const T3 &a1) {
  return std::tuple<T1, T2>(std::move(a0), a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    std::__enable_if_t<std::_TupleConstraints<true, T1, T2>::
                           template __is_implicitly_constructible<T4, T5>(),
                       bool>
        T6 = true>
std::tuple<T1, T2> f47(std::allocator_arg_t a0, const T3 &a1, T4 &&a2,
                       T5 &&a3) {
  return std::tuple<T1, T2>(std::move(a0), a1, std::forward<T4>(a2),
                            std::forward<T5>(a3));
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    std::__enable_if_t<std::_TupleConstraints<true, T1, T2>::
                           template __is_explicitly_constructible<T4, T5>(),
                       bool>
        T6 = false>
std::tuple<T1, T2> f48(std::allocator_arg_t a0, const T3 &a1, T4 &&a2,
                       T5 &&a3) {
  return std::tuple<T1, T2>(std::move(a0), a1, std::forward<T4>(a2),
                            std::forward<T5>(a3));
}

template <typename T1, typename T2, typename T3>
std::tuple<T1, T2> f49(std::allocator_arg_t a0, const T3 &a1,
                       const std::tuple<T1, T2> &a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
std::tuple<T1, T2> f50(std::allocator_arg_t a0, const T3 &a1,
                       std::tuple<T1, T2> &&a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          std::__enable_if_t<std::_TupleConstraints<true, T1, T2>::
                                 template __is_implicitly_constructible<
                                     const T4 &, const T5 &>(),
                             bool>
              T6 = true>
std::tuple<T1, T2> f51(std::allocator_arg_t a0, const T3 &a1,
                       const std::tuple<T4, T5> &a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          std::__enable_if_t<std::_TupleConstraints<true, T1, T2>::
                                 template __is_explicitly_constructible<
                                     const T4 &, const T5 &>(),
                             bool>
              T6 = false>
std::tuple<T1, T2> f52(std::allocator_arg_t a0, const T3 &a1,
                       const std::tuple<T4, T5> &a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, a2);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    std::__enable_if_t<std::_TupleConstraints<true, T1, T2>::
                           template __is_implicitly_constructible<T4, T5>(),
                       bool>
        T6 = true>
std::tuple<T1, T2> f53(std::allocator_arg_t a0, const T3 &a1,
                       std::tuple<T4, T5> &&a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, std::move(a2));
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    std::__enable_if_t<std::_TupleConstraints<true, T1, T2>::
                           template __is_explicitly_constructible<T4, T5>(),
                       bool>
        T6 = false>
std::tuple<T1, T2> f54(std::allocator_arg_t a0, const T3 &a1,
                       std::tuple<T4, T5> &&a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          std::__enable_if_t<std::_TupleConstraints<true, T1, T2>::
                                 template __is_implicitly_constructible<
                                     const T4 &, const T5 &>(),
                             bool>
              T6 = true>
std::tuple<T1, T2> f55(std::allocator_arg_t a0, const T3 &a1,
                       const std::pair<T4, T5> &a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          std::__enable_if_t<std::_TupleConstraints<true, T1, T2>::
                                 template __is_explicitly_constructible<
                                     const T4 &, const T5 &>(),
                             bool>
              T6 = false>
std::tuple<T1, T2> f56(std::allocator_arg_t a0, const T3 &a1,
                       const std::pair<T4, T5> &a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, a2);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    std::__enable_if_t<std::_TupleConstraints<true, T1, T2>::
                           template __is_implicitly_constructible<T4, T5>(),
                       bool>
        T6 = true>
std::tuple<T1, T2> f57(std::allocator_arg_t a0, const T3 &a1,
                       std::pair<T4, T5> &&a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, std::move(a2));
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    std::__enable_if_t<std::_TupleConstraints<true, T1, T2>::
                           template __is_explicitly_constructible<T4, T5>(),
                       bool>
        T6 = false>
std::tuple<T1, T2> f58(std::allocator_arg_t a0, const T3 &a1,
                       std::pair<T4, T5> &&a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, std::move(a2));
}

template <
    typename... T1, typename T2 = void,
    std::__enable_if_t<
        std::_TupleConstraints<std::integral_constant<bool, true>::value,
                               T1...>::__is_implicitly_default_constructible(),
        bool>
        T3 = true>
std::tuple<T1...> f59() {
  return std::tuple<T1...>();
}

template <
    typename... T1, typename T2 = void,
    std::__enable_if_t<
        std::_TupleConstraints<std::integral_constant<bool, true>::value,
                               T1...>::__is_explicitly_default_constructible(),
        bool>
        T3 = false>
std::tuple<T1...> f60() {
  return std::tuple<T1...>();
}

template <typename... T1, typename T2,
          std::__enable_if_t<
              std::_TupleConstraints<std::is_object<T2>::value, T1...>::
                  __is_implicitly_default_constructible(),
              bool>
              T3 = true>
std::tuple<T1...> f61(std::allocator_arg_t a0, const T2 &a1) {
  return std::tuple<T1...>(std::move(a0), a1);
}

template <typename... T1, typename T2,
          std::__enable_if_t<
              std::_TupleConstraints<std::is_object<T2>::value, T1...>::
                  __is_explicitly_default_constructible(),
              bool>
              T3 = false>
std::tuple<T1...> f62(std::allocator_arg_t a0, const T2 &a1) {
  return std::tuple<T1...>(std::move(a0), a1);
}

template <typename... T1, typename T2>
std::tuple<T1...> f63(std::allocator_arg_t a0, const T2 &a1,
                      std::tuple<T1...> &&a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}
