#include <sstream>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
using t8 = typename std::basic_stringbuf<T1, T2, T3>::string_type;
