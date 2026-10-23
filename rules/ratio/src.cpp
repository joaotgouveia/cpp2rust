#include <ratio>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t1 = std::atto;

using t2 = std::centi;

using t3 = std::deca;

using t4 = std::deci;

using t5 = std::exa;

using t6 = std::femto;

using t7 = std::giga;

using t8 = std::hecto;

using t9 = std::kilo;

using t10 = std::mega;

using t11 = std::micro;

using t12 = std::milli;

using t13 = std::nano;

using t14 = std::peta;

using t15 = std::pico;

template <std::intmax_t T1, std::intmax_t T2> using t16 = std::ratio<T1, T2>;

template <std::intmax_t T1, std::intmax_t T2>
using t17 = typename std::ratio<T1, T2>::type;

template <std::intmax_t T1, std::intmax_t T2> std::intmax_t f1() {
  return std::ratio<T1, T2>::num;
}

template <std::intmax_t T1, std::intmax_t T2> std::intmax_t f2() {
  return std::ratio<T1, T2>::den;
}

template <typename T1, typename T2> using t18 = std::ratio_add<T1, T2>;

template <typename T1, typename T2> using t19 = std::ratio_divide<T1, T2>;

template <typename T1, typename T2> using t20 = std::ratio_multiply<T1, T2>;

template <typename T1, typename T2> using t21 = std::ratio_subtract<T1, T2>;

using t22 = std::tera;
