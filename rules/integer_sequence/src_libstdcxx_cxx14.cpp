#include <algorithm>
#include <utility>

template <typename T1, T1... T2>
using t2 = typename std::integer_sequence<T1, T2...>::value_type;
