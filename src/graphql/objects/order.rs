use async_graphql::Enum;

#[derive(Debug, Clone, Copy, Enum, PartialEq, Eq, PartialOrd, Ord)]
pub enum GraphQLOrdering {
    Ascending,
    Descending,
}
