#include <cstdlib>
#include <utility>

std::ldiv_t f1(long a0, long a1) { return std::div(a0, a1); }

__gnu_cxx::lldiv_t f2(long long a0, long long a1) { return std::div(a0, a1); }
