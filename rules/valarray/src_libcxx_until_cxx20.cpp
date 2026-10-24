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
std::valarray<T1> &f255(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator*= <T2>(a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, typename std::valarray<T1>::value_type>::value)),
        int>::type T4 = 0>
std::valarray<T1> &f256(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator/= <T2>(a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, typename std::valarray<T1>::value_type>::value)),
        int>::type T4 = 0>
std::valarray<T1> &f257(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator%= <T2>(a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, typename std::valarray<T1>::value_type>::value)),
        int>::type T4 = 0>
std::valarray<T1> &f258(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator+= <T2>(a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, typename std::valarray<T1>::value_type>::value)),
        int>::type T4 = 0>
std::valarray<T1> &f259(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator-= <T2>(a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, typename std::valarray<T1>::value_type>::value)),
        int>::type T4 = 0>
std::valarray<T1> &f260(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator^= <T2>(a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, typename std::valarray<T1>::value_type>::value)),
        int>::type T4 = 0>
std::valarray<T1> &f261(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator|= <T2>(a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, typename std::valarray<T1>::value_type>::value)),
        int>::type T4 = 0>
std::valarray<T1> &f262(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator&= <T2>(a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, typename std::valarray<T1>::value_type>::value)),
        int>::type T4 = 0>
std::valarray<T1> &f263(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator<<= <T2>(a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__is_val_expr<T2>::value, int> T3 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, typename std::valarray<T1>::value_type>::value)),
        int>::type T4 = 0>
std::valarray<T1> &f264(std::valarray<T1> &a0, const T2 &a1) {
  return a0.template operator>>= <T2>(a1);
}
