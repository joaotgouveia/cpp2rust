#include <initializer_list>
#include <set>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3>
using t33 =
    typename std::multiset<T1, T2, T3>::reverse_iterator::iterator_concept;

template <typename T1, typename T2, typename T3>
using t34 =
    typename std::multiset<T1, T2,
                           T3>::const_reverse_iterator::iterator_concept;
