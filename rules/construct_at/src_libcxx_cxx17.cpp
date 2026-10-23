#include <memory>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> void f1(T1 *a0) { return std::destroy_at<T1>(a0); }
