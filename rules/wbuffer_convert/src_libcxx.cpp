#include <ios>
#include <iosfwd>
#include <locale>
#include <utility>

template <typename T1, typename T2, typename T3>
using t3 = typename std::wbuffer_convert<T1, T2, T3>::char_type;

template <typename T1, typename T2, typename T3>
using t4 = typename std::wbuffer_convert<T1, T2, T3>::traits_type;

template <typename T1, typename T2, typename T3>
using t5 = typename std::wbuffer_convert<T1, T2, T3>::int_type;

template <typename T1, typename T2, typename T3>
using t6 = typename std::wbuffer_convert<T1, T2, T3>::pos_type;

template <typename T1, typename T2, typename T3>
using t7 = typename std::wbuffer_convert<T1, T2, T3>::off_type;
