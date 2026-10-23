#include <iterator>
#include <utility>

template <typename T1, typename T2, typename T3>
using t11 = typename std::ostream_iterator<T1, T2, T3>::difference_type;
