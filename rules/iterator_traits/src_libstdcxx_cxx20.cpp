#include <algorithm>
#include <utility>

template <typename T1>
  requires(std::same_as<std::__detail::__iter_traits<T1>,
                        std::iterator_traits<T1>>)
using t24 = std::iterator_traits<std::counted_iterator<T1>>;
