#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <bit>
#include <utility>

template <typename T1> T1 f2(T1 a0, int a1) { return std::rotl<T1>(a0, a1); }

template <typename T1> T1 f3(T1 a0, int a1) { return std::rotr<T1>(a0, a1); }
