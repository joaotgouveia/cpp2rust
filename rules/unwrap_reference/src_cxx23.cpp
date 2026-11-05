#include <type_traits>
#include <utility>

template <typename T1> using t1 = std::unwrap_reference<T1>;

template <typename T1> using t2 = typename std::unwrap_reference<T1>::type;
