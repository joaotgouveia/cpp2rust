#include "../common/init.h"
#include "../common/traits.h"
#include <stop_token>
#include <utility>

template <typename T1> using t1 = std::stop_callback<T1>;

template <typename T1>
using t2 = typename std::stop_callback<T1>::callback_type;
