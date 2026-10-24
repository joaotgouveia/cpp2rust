#include <algorithm>
#include <atomic>
#include <condition_variable>
#include <cstddef>
#include <memory>
#include <ostream>
#include <type_traits>
#include <utility>

template <typename T1> using t3 = typename std::shared_ptr<T1>::weak_type;
