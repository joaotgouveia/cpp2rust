#include <initializer_list>
#include <map>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3, typename T4>
using t38 =
    typename std::multimap<T1, T2, T3, T4>::reverse_iterator::iterator_concept;

template <typename T1, typename T2, typename T3, typename T4>
using t39 =
    typename std::multimap<T1, T2, T3,
                           T4>::const_reverse_iterator::iterator_concept;
