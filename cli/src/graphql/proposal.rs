pub struct ProposalView;
pub mod proposal_view {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "ProposalView";
    pub const QUERY : & str = "query ProposalView {\n  proposals {\n    id\n    operations {\n      id\n      index\n      target\n      value\n      data\n    }\n    predecessor\n    delay\n    timestamp\n    status\n  }\n}\n" ;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type BigInt = String;
    type Bytes = String;
    #[derive(PartialEq, Clone, Hash, Eq, Debug)]
    pub enum Status {
        Pending,
        Ready,
        Executed,
        Cancelled,
        Other(String),
    }
    impl ::serde::Serialize for Status {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                Status::Pending => "Pending",
                Status::Ready => "Ready",
                Status::Executed => "Executed",
                Status::Cancelled => "Cancelled",
                Status::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Status {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "Pending" => Ok(Status::Pending),
                "Ready" => Ok(Status::Ready),
                "Executed" => Ok(Status::Executed),
                "Cancelled" => Ok(Status::Cancelled),
                _ => Ok(Status::Other(s)),
            }
        }
    }
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct ResponseData {
        pub proposals: Vec<ProposalViewProposals>,
    }
    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct ProposalViewProposals {
        pub id: ID,
        pub operations: Vec<ProposalViewProposalsOperations>,
        pub predecessor: Bytes,
        pub delay: BigInt,
        pub timestamp: BigInt,
        pub status: Status,
    }

    impl std::fmt::Display for ProposalViewProposals {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "id: {}\noperations: {:#?}\npredecessor: {}\ntimestamp: {}\nstatus: {:?}",
                self.id, self.operations, self.predecessor, self.timestamp, self.status
            )
        }
    }
    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct ProposalViewProposalsOperations {
        pub id: ID,
        pub index: BigInt,
        pub target: Bytes,
        pub value: BigInt,
        pub data: Bytes,
    }
}
impl graphql_client::GraphQLQuery for ProposalView {
    type Variables = proposal_view::Variables;
    type ResponseData = proposal_view::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: proposal_view::QUERY,
            operation_name: proposal_view::OPERATION_NAME,
        }
    }
}
