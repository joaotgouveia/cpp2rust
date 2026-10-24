#include <regex>
#include <string>
#include <utility>

template <typename T1, typename T2>
using t17 =
    typename std::match_results<T1, T2>::const_iterator::iterator_concept;
