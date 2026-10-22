#include "../common/init.h"
#include "../common/traits.h"
#include <utility>

template <typename T1> bool f1(const T1 &a0, const T1 &a1) { return a0 != a1; }

template <typename T1> bool f2(const T1 &a0, const T1 &a1) { return a0 <= a1; }

template <typename T1> bool f3(const T1 &a0, const T1 &a1) { return a0 > a1; }

template <typename T1> bool f4(const T1 &a0, const T1 &a1) { return a0 >= a1; }
