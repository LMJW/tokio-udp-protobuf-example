#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EchoMsg {
    #[prost(string, tag="1")]
    pub ipv4: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub chat: ::prost::alloc::string::String,
}
