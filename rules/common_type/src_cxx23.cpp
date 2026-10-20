#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <chrono>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
using t15 = std::common_type<std::basic_const_iterator<T1>,
                             std::basic_const_iterator<T2>>;

template <typename T1, typename T2>
using t16 = typename std::common_type<std::basic_const_iterator<T1>,
                                      std::basic_const_iterator<T2>>::type;

template <typename T1, typename T2>
using t17 = typename std::common_type<
    std::basic_const_iterator<T1>,
    std::basic_const_iterator<T2>>::type::iterator_concept;

template <typename T1, typename T2>
using t18 =
    typename std::common_type<std::basic_const_iterator<T1>,
                              std::basic_const_iterator<T2>>::type::value_type;

template <typename T1, typename T2>
using t19 = typename std::common_type<
    std::basic_const_iterator<T1>,
    std::basic_const_iterator<T2>>::type::difference_type;

template <typename T1, typename T2>
using t20 = std::common_type<std::basic_const_iterator<T1>, T2>;

template <typename T1, typename T2>
using t21 = typename std::common_type<std::basic_const_iterator<T1>, T2>::type;

template <typename T1, typename T2>
using t22 = typename std::common_type<std::basic_const_iterator<T1>,
                                      T2>::type::iterator_concept;

template <typename T1, typename T2>
using t23 = typename std::common_type<std::basic_const_iterator<T1>,
                                      T2>::type::value_type;

template <typename T1, typename T2>
using t24 = typename std::common_type<std::basic_const_iterator<T1>,
                                      T2>::type::difference_type;

template <typename T1, typename T2>
using t25 = std::common_type<T2, std::basic_const_iterator<T1>>;

template <typename T1, typename T2>
using t26 = typename std::common_type<T2, std::basic_const_iterator<T1>>::type;

template <typename T1, typename T2>
using t27 = typename std::common_type<
    T2, std::basic_const_iterator<T1>>::type::iterator_concept;

template <typename T1, typename T2>
using t28 =
    typename std::common_type<T2,
                              std::basic_const_iterator<T1>>::type::value_type;

template <typename T1, typename T2>
using t29 = typename std::common_type<
    T2, std::basic_const_iterator<T1>>::type::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t30 = std::common_type<std::pair<T1, T2>, std::pair<T3, T4>>;

template <typename T1, typename T2, typename T3, typename T4>
using t31 =
    typename std::common_type<std::pair<T1, T2>, std::pair<T3, T4>>::type;

template <typename T1, typename T2, typename T3, typename T4>
using t32 = typename std::common_type<std::pair<T1, T2>,
                                      std::pair<T3, T4>>::type::first_type;

template <typename T1, typename T2, typename T3, typename T4>
using t33 = typename std::common_type<std::pair<T1, T2>,
                                      std::pair<T3, T4>>::type::second_type;
