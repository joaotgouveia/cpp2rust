#include <algorithm>
#include <tuple>
#include <type_traits>
#include <utility>

template <typename T1, typename T2, template <typename> typename T3,
          template <typename> typename T4>
using t2 = typename std::basic_common_reference<T1, T2, T3, T4>::type;

template <typename T1, typename T2, typename T3, typename T4,
          template <typename> typename T5, template <typename> typename T6>
using t3 =
    std::basic_common_reference<std::pair<T1, T2>, std::pair<T3, T4>, T5, T6>;

template <typename T1, typename T2, typename T3, typename T4,
          template <typename> typename T5, template <typename> typename T6>
using t4 =
    typename std::basic_common_reference<std::pair<T1, T2>, std::pair<T3, T4>,
                                         T5, T6>::type;
