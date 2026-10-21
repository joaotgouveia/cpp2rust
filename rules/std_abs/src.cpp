#include "../common/init.h"
#include "../common/traits.h"
#include <cstdlib>
#include <stdlib.h>
#include <utility>

int f1(int a0) { return abs(a0); }

long double f2(long double a0) { return std::abs(a0); }

long long f3(long long a0) { return std::abs(a0); }

long f4(long a0) { return std::abs(a0); }

double f5(double a0) { return std::abs(a0); }

float f6(float a0) { return std::abs(a0); }
