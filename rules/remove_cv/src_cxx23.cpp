#include <type_traits>
#include <utility>

template <typename T1> using t1 = std::remove_cv<T1>;

template <typename T1> using t2 = typename std::remove_cv<T1>::type;
