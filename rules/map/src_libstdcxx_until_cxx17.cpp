#include <algorithm>
#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
using t26 = typename std::map<T1, T2, T3, T4>::value_type::first_type;

template <typename T1, typename T2, typename T3, typename T4>
using t27 = typename std::map<T1, T2, T3, T4>::value_type::second_type;
