#include "../common/init.h"
#include "../common/traits.h"
#include <regex>
#include <string>
#include <utility>

template <typename T1, typename T2>
using t35 =
    typename std::match_results<T1, T2>::const_iterator::iterator_concept;

template <typename T1, typename T2>
using t36 = typename std::match_results<T1, T2>::iterator::iterator_concept;
