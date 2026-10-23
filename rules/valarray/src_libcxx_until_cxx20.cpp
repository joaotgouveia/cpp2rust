#include <cstddef>
#include <initializer_list>
#include <utility>
#include <valarray>

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, typename std::valarray<T1>::value_type>::value)),
        int>::type T4 = 0>
std::valarray<T1> &f228(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator*= <T2>(a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, typename std::valarray<T1>::value_type>::value)),
        int>::type T4 = 0>
std::valarray<T1> &f229(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator/= <T2>(a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, typename std::valarray<T1>::value_type>::value)),
        int>::type T4 = 0>
std::valarray<T1> &f230(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator%= <T2>(a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, typename std::valarray<T1>::value_type>::value)),
        int>::type T4 = 0>
std::valarray<T1> &f231(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator+= <T2>(a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, typename std::valarray<T1>::value_type>::value)),
        int>::type T4 = 0>
std::valarray<T1> &f232(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator-= <T2>(a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, typename std::valarray<T1>::value_type>::value)),
        int>::type T4 = 0>
std::valarray<T1> &f233(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator^= <T2>(a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, typename std::valarray<T1>::value_type>::value)),
        int>::type T4 = 0>
std::valarray<T1> &f234(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator|= <T2>(a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, typename std::valarray<T1>::value_type>::value)),
        int>::type T4 = 0>
std::valarray<T1> &f235(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator&= <T2>(a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, typename std::valarray<T1>::value_type>::value)),
        int>::type T4 = 0>
std::valarray<T1> &f236(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator<<= <T2>(a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, typename std::valarray<T1>::value_type>::value)),
        int>::type T4 = 0>
std::valarray<T1> &f237(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator>>= <T2>(a1);
}
