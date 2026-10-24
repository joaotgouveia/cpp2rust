#include <regex>
#include <string>
#include <utility>

template <typename T1, typename T2>
using t24 = typename std::match_results<T1, T2>::const_iterator;

template <typename T1, typename T2>
using t25 = typename std::match_results<T1, T2>::iterator;
