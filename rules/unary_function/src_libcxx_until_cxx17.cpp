#include <functional>
#include <utility>

template <typename T1, typename T2> using t4 = std::unary_function<T1, T2>;

template <typename T1, typename T2>
using t5 = typename std::unary_function<T1, T2>::argument_type;

template <typename T1, typename T2>
using t6 = typename std::unary_function<T1, T2>::result_type;
