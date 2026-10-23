#include <type_traits>
#include <utility>

bool f1() { return std::is_constant_evaluated(); }
