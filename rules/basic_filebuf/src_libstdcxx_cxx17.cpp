#include "../common/init.h"
#include "../common/traits.h"
#include <fstream>
#include <ios>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_filebuf<T1, T2> *f23(std::basic_filebuf<T1, T2> &a0, const T3 &a1,
                                std::ios_base::openmode a2) {
  return a0.template open<T3>(a1, a2);
}
