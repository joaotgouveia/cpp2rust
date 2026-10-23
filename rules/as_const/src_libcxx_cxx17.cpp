#include <utility>

template <typename T1> const T1 &f1(T1 &a0) { return std::as_const<T1>(a0); }
