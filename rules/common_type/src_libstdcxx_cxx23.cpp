#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <chrono>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
using t19 = std::common_type<std::basic_const_iterator<T1>,
                             std::basic_const_iterator<T2>>;

template <typename T1, typename T2>
using t20 = typename std::common_type<std::basic_const_iterator<T1>,
                                      std::basic_const_iterator<T2>>::type;

template <typename T1, typename T2>
using t21 = typename std::common_type<
    std::basic_const_iterator<T1>,
    std::basic_const_iterator<T2>>::type::iterator_concept;

template <typename T1, typename T2>
using t22 =
    typename std::common_type<std::basic_const_iterator<T1>,
                              std::basic_const_iterator<T2>>::type::value_type;

template <typename T1, typename T2>
using t23 = typename std::common_type<
    std::basic_const_iterator<T1>,
    std::basic_const_iterator<T2>>::type::difference_type;

template <typename T1, typename T2>
using t24 = std::common_type<std::basic_const_iterator<T1>, T2>;

template <typename T1, typename T2>
using t25 = typename std::common_type<std::basic_const_iterator<T1>, T2>::type;

template <typename T1, typename T2>
using t26 = typename std::common_type<std::basic_const_iterator<T1>,
                                      T2>::type::iterator_concept;

template <typename T1, typename T2>
using t27 = typename std::common_type<std::basic_const_iterator<T1>,
                                      T2>::type::value_type;

template <typename T1, typename T2>
using t28 = typename std::common_type<std::basic_const_iterator<T1>,
                                      T2>::type::difference_type;

template <typename T1, typename T2>
using t29 = std::common_type<T2, std::basic_const_iterator<T1>>;

template <typename T1, typename T2>
using t30 = typename std::common_type<T2, std::basic_const_iterator<T1>>::type;

template <typename T1, typename T2>
using t31 = typename std::common_type<
    T2, std::basic_const_iterator<T1>>::type::iterator_concept;

template <typename T1, typename T2>
using t32 =
    typename std::common_type<T2,
                              std::basic_const_iterator<T1>>::type::value_type;

template <typename T1, typename T2>
using t33 = typename std::common_type<
    T2, std::basic_const_iterator<T1>>::type::difference_type;
