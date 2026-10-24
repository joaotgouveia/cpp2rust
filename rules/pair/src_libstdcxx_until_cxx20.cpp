#include <algorithm>
#include <utility>

template <
    typename T1, typename T2, typename T3 = T1, typename T4 = T2,
    typename std::enable_if<
        std::__and_<std::__is_implicitly_default_constructible<T1>,
                    std::__is_implicitly_default_constructible<T2>>::value,
        bool>::type T5 = true>
std::pair<T1, T2> f5() {
  return std::pair<T1, T2>();
}

template <typename T1, typename T2, typename T3 = T1, typename T4 = T2,
          typename std::enable_if<
              std::__and_<
                  std::is_default_constructible<T1>,
                  std::is_default_constructible<T2>,
                  std::__not_<std::__and_<
                      std::__is_implicitly_default_constructible<T1>,
                      std::__is_implicitly_default_constructible<T2>>>>::value,
              bool>::type T5 = false>
std::pair<T1, T2> f6() {
  return std::pair<T1, T2>();
}

template <typename T1, typename T2, typename T3 = T1, typename T4 = T2,
          typename std::enable_if<
              std::_PCC<true, T1, T2>::template _ConstructiblePair<T1, T2>() &&
                  std::_PCC<true, T1,
                            T2>::template _ImplicitlyConvertiblePair<T1, T2>(),
              bool>::type T5 = true>
std::pair<T1, T2> f7(const T1 &a0, const T2 &a1) {
  return std::pair<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3 = T1, typename T4 = T2,
          typename std::enable_if<
              std::_PCC<true, T1, T2>::template _ConstructiblePair<T1, T2>() &&
                  !std::_PCC<true, T1,
                             T2>::template _ImplicitlyConvertiblePair<T1, T2>(),
              bool>::type T5 = false>
std::pair<T1, T2> f8(const T1 &a0, const T2 &a1) {
  return std::pair<T1, T2>(a0, a1);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename std::enable_if<
        std::_PCC<!std::is_same<T1, T3>::value || !std::is_same<T2, T4>::value,
                  T1, T2>::template _ConstructiblePair<T3, T4>() &&
            std::_PCC<!std::is_same<T1, T3>::value ||
                          !std::is_same<T2, T4>::value,
                      T1, T2>::template _ImplicitlyConvertiblePair<T3, T4>(),
        bool>::type T5 = true,
    typename std::enable_if<(!(std::is_same<T3, T1>::value &&
                               std::is_same<T4, T2>::value)),
                            int>::type T6 = 0>
std::pair<T1, T2> f9(const std::pair<T3, T4> &a0) {
  return std::pair<T1, T2>(a0);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename std::enable_if<
        std::_PCC<!std::is_same<T1, T3>::value || !std::is_same<T2, T4>::value,
                  T1, T2>::template _ConstructiblePair<T3, T4>() &&
            !std::_PCC<!std::is_same<T1, T3>::value ||
                           !std::is_same<T2, T4>::value,
                       T1, T2>::template _ImplicitlyConvertiblePair<T3, T4>(),
        bool>::type T5 = false,
    typename std::enable_if<(!(std::is_same<T3, T1>::value &&
                               std::is_same<T4, T2>::value)),
                            int>::type T6 = 0>
std::pair<T1, T2> f10(const std::pair<T3, T4> &a0) {
  return std::pair<T1, T2>(a0);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename std::enable_if<
        std::_PCC<true, T1, T2>::template _MoveConstructiblePair<T3, T4>() &&
            std::_PCC<true, T1,
                      T2>::template _ImplicitlyMoveConvertiblePair<T3, T4>(),
        bool>::type T5 = true,
    typename std::enable_if<(!(std::is_same<T3, const T1 &>::value &&
                               std::is_same<T4, const T2 &>::value)),
                            int>::type T6 = 0>
std::pair<T1, T2> f11(T3 &&a0, T4 &&a1) {
  return std::pair<T1, T2>(std::forward<T3>(a0), std::forward<T4>(a1));
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename std::enable_if<
        std::_PCC<true, T1, T2>::template _MoveConstructiblePair<T3, T4>() &&
            !std::_PCC<true, T1,
                       T2>::template _ImplicitlyMoveConvertiblePair<T3, T4>(),
        bool>::type T5 = false,
    typename std::enable_if<(!(std::is_same<T3, const T1 &>::value &&
                               std::is_same<T4, const T2 &>::value)),
                            int>::type T6 = 0>
std::pair<T1, T2> f12(T3 &&a0, T4 &&a1) {
  return std::pair<T1, T2>(std::forward<T3>(a0), std::forward<T4>(a1));
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename std::enable_if<
        std::_PCC<!std::is_same<T1, T3>::value || !std::is_same<T2, T4>::value,
                  T1, T2>::template _MoveConstructiblePair<T3, T4>() &&
            std::_PCC<
                !std::is_same<T1, T3>::value || !std::is_same<T2, T4>::value,
                T1, T2>::template _ImplicitlyMoveConvertiblePair<T3, T4>(),
        bool>::type T5 = true,
    typename std::enable_if<(!(std::is_same<T3, T1>::value &&
                               std::is_same<T4, T2>::value)),
                            int>::type T6 = 0>
std::pair<T1, T2> f13(std::pair<T3, T4> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename std::enable_if<
        std::_PCC<!std::is_same<T1, T3>::value || !std::is_same<T2, T4>::value,
                  T1, T2>::template _MoveConstructiblePair<T3, T4>() &&
            !std::_PCC<
                !std::is_same<T1, T3>::value || !std::is_same<T2, T4>::value,
                T1, T2>::template _ImplicitlyMoveConvertiblePair<T3, T4>(),
        bool>::type T5 = false,
    typename std::enable_if<(!(std::is_same<T3, T1>::value &&
                               std::is_same<T4, T2>::value)),
                            int>::type T6 = 0>
std::pair<T1, T2> f14(std::pair<T3, T4> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<
    std::__and_<std::is_assignable<T1 &, const T3 &>,
                std::is_assignable<T2 &, const T4 &>>::value,
    std::pair<T1, T2> &>::type
f15(std::pair<T1, T2> &a0, const std::pair<T3, T4> &a1) {
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::__and_<std::is_assignable<T1 &, T3 &&>,
                                    std::is_assignable<T2 &, T4 &&>>::value,
                        std::pair<T1, T2> &>::type
f16(std::pair<T1, T2> &a0, std::pair<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}
