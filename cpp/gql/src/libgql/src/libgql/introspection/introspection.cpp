#include "./introspection.hpp"

namespace gql::introspection {
const char* INTROSPECTION_QUERY =
#include "./query.data"
};
