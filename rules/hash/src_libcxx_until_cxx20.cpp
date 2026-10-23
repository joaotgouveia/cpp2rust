#include <algorithm>
#include <cstddef>
#include <functional>
#include <memory>
#include <string>
#include <string_view>
#include <system_error>
#include <typeindex>
#include <utility>
#include <vector>

template <typename T1>
using t58 = typename std::hash<std::shared_ptr<T1>>::argument_type;

template <typename T1>
using t59 = typename std::hash<std::shared_ptr<T1>>::result_type;

template <typename T1, typename T2>
using t60 = typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::argument_type;

template <typename T1, typename T2>
using t61 = typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::result_type;
