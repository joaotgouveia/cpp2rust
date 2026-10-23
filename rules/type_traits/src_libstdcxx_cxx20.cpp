#include <type_traits>
#include <utility>

template <typename... T1> using t29 = std::common_reference_t<T1...>;

bool f2() { return std::is_constant_evaluated(); }

template <typename T1> using t30 = std::remove_cvref_t<T1>;

template <typename T1> using t31 = std::type_identity_t<T1>;

template <typename T1> using t32 = std::unwrap_ref_decay_t<T1>;

template <typename T1> using t33 = std::unwrap_reference_t<T1>;
