#include <fstream>
#include <ios>
#include <iosfwd>
#include <utility>

template <typename T1, typename T2, typename T3>
std::_If_fs_path<T3, std::basic_filebuf<T1, T2> *>
f25(std::basic_filebuf<T1, T2> &a0, const T3 &a1, std::ios_base::openmode a2)
  requires(!(std::is_same<T3, std::string>::value))
{
  return a0.template open<T3>(a1, a2);
}
