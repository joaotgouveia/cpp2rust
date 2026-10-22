#include "../common/init.h"
#include "../common/traits.h"
#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f100(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template contains<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
using t38 =
    typename std::multimap<T1, T2, T3, T4>::reverse_iterator::iterator_concept;

template <typename T1, typename T2, typename T3, typename T4>
using t39 =
    typename std::multimap<T1, T2, T3,
                           T4>::const_reverse_iterator::iterator_concept;
