#include <algorithm>
#include <iterator>
#include <utility>

template <typename T1> using t1 = std::indirectly_readable_traits<T1 *>;

template <typename T1> using t2 = std::indirectly_readable_traits<const T1>;

template <typename T1> using t3 = std::indirectly_readable_traits<T1>;

template <typename T1>
  requires(std::is_array_v<T1>)
using t4 = std::indirectly_readable_traits<T1>;

template <typename T1>
using t5 = typename std::indirectly_readable_traits<T1>::value_type;
