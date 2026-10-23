#include <algorithm>
#include <chrono>
#include <tuple>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
using t17 = std::common_type<std::basic_const_iterator<T1>,
                             std::basic_const_iterator<T2>>;

template <typename T1, typename T2>
using t18 = typename std::common_type<std::basic_const_iterator<T1>,
                                      std::basic_const_iterator<T2>>::type;

template <typename T1, typename T2>
using t19 = std::common_type<std::basic_const_iterator<T1>, T2>;

template <typename T1, typename T2>
using t20 = typename std::common_type<std::basic_const_iterator<T1>, T2>::type;

template <typename T1, typename T2>
using t21 = typename std::common_type<T1, T2>::type;

template <typename T1, typename T2>
using t22 = std::common_type<T2, std::basic_const_iterator<T1>>;

template <typename T1, typename T2>
using t23 = typename std::common_type<T2, std::basic_const_iterator<T1>>::type;
