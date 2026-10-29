#include <barrier>
#include <utility>

template <typename T1, typename T2> using t1 = std::unary_function<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::unary_function<T1, T2>::argument_type;

template <typename T1, typename T2>
using t3 = typename std::unary_function<T1, T2>::result_type;
