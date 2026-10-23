#include <algorithm>
#include <ios>
#include <locale>
#include <string>
#include <utility>

template <typename T1, typename T2>
using t2 = typename std::time_put_byname<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::time_put_byname<T1, T2>::iter_type;
