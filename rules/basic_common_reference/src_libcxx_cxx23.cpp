#include "../common/init.h"
#include "../common/traits.h"
#include <functional>
#include <type_traits>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4,
          template <class> typename T5, template <class> typename T6>
using t7 =
    std::basic_common_reference<std::pair<T1, T2>, std::pair<T3, T4>, T5, T6>;

template <typename T1, typename T2, typename T3, typename T4,
          template <class> typename T5, template <class> typename T6>
using t8 =
    typename std::basic_common_reference<std::pair<T1, T2>, std::pair<T3, T4>,
                                         T5, T6>::type;
