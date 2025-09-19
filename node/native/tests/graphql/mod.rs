/// End-to-end tests for GraphQL endpoints
///
/// These tests verify all GraphQL query and mutation endpoints exposed by the Mina node.
/// Each test validates the endpoint's functionality and response structure.

#[cfg(test)]
mod tests_query;

#[cfg(test)]
mod tests_mutation;

#[cfg(test)]
mod tests_edge_cases;
