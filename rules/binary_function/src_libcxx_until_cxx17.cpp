#include <functional>
#include <utility>

template <typename T1, typename T2, typename T3>
using t5 = std::binary_function<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t6 = typename std::binary_function<T1, T2, T3>::first_argument_type;

template <typename T1, typename T2, typename T3>
using t7 = typename std::binary_function<T1, T2, T3>::second_argument_type;

template <typename T1, typename T2, typename T3>
using t8 = typename std::binary_function<T1, T2, T3>::result_type;
