#include <type_traits>
#include <utility>

template <typename T1> using t1 = std::common_reference<T1>;

template <typename T1> using t2 = typename std::common_reference<T1>::type;

template <typename T1, typename T2> using t3 = std::common_reference<T1, T2>;
