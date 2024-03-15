#include <string>

#include "gql/gql.hpp"

#include <catch2/catch_test_macros.hpp>

TEST_CASE("Name is gql", "[library]")
{
  auto const exported = exported_class {};
  REQUIRE(std::string("gql") == exported.name());
}
