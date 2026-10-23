#include <regex>
#include <string>
#include <utility>

template <typename T1, typename T2>
using t33 =
    typename std::match_results<T1, T2>::const_iterator::iterator_concept;

template <typename T1, typename T2>
using t34 = typename std::match_results<T1, T2>::iterator::iterator_concept;
