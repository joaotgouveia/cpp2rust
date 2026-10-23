#include <memory>
#include <utility>

template <typename T1> void f1(T1 *a0) { return std::destroy_at<T1>(a0); }
