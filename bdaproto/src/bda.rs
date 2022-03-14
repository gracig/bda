///Resource represents a resource in the BDA architecture
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    #[prost(string, tag = "1")]
    pub revision: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub namespace: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(oneof = "resource::ResourceKind", tags = "10, 50, 51")]
    pub resource_kind: ::core::option::Option<resource::ResourceKind>,
}
/// Nested message and enum types in `Resource`.
pub mod resource {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ResourceKind {
        #[prost(message, tag = "10")]
        Variables(super::Variables),
        #[prost(message, tag = "50")]
        Function(super::Function),
        #[prost(message, tag = "51")]
        Runtime(super::Runtime),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Variables {
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<::prost_types::Struct>,
}
/// Function is a resource that declares parameters and a procedure to be executed in order to apply
/// transformations like build a source code or deploy an application
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Function {
    #[prost(message, repeated, tag = "1")]
    pub inputs: ::prost::alloc::vec::Vec<Parameter>,
    #[prost(message, repeated, tag = "2")]
    pub outputs: ::prost::alloc::vec::Vec<Parameter>,
    #[prost(string, repeated, tag = "3")]
    pub base_command: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub runtime_capabilities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Parameter {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(enumeration = "ParameterKind", tag = "3")]
    pub kind: i32,
    #[prost(message, optional, tag = "4")]
    pub default_value: ::core::option::Option<::prost_types::Value>,
}
///Runtime is a resource that describes an environment where procedures can be executed
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Runtime {
    #[prost(string, repeated, tag = "1")]
    pub capabilities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(oneof = "runtime::RuntimeKind", tags = "10")]
    pub runtime_kind: ::core::option::Option<runtime::RuntimeKind>,
}
/// Nested message and enum types in `Runtime`.
pub mod runtime {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RuntimeKind {
        #[prost(message, tag = "10")]
        Container(super::Container),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Container {
    #[prost(string, tag = "1")]
    pub dockerfile: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ParameterKind {
    Integer = 0,
    Real = 1,
    Boolean = 2,
    Text = 3,
    Json = 4,
    Url = 5,
    Path = 6,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRevisionsRequest {
    #[prost(bool, tag = "100")]
    pub ph: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNamespacesRequest {
    #[prost(bool, tag = "100")]
    pub ph: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKindsRequest {
    #[prost(bool, tag = "100")]
    pub ph: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResourcesResponse {
    #[prost(message, repeated, tag = "1")]
    pub resources: ::prost::alloc::vec::Vec<Resource>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRevisionsResponse {
    #[prost(string, repeated, tag = "1")]
    pub revisions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNamespacesResponse {
    #[prost(string, repeated, tag = "1")]
    pub namespaces: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKindsResponse {
    #[prost(string, repeated, tag = "1")]
    pub kinds: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResourcesRequest {
    ///blank is latest
    #[prost(string, tag = "1")]
    pub revision: ::prost::alloc::string::String,
    ///all for all or comma separated values. blank is all
    #[prost(string, tag = "2")]
    pub namespaces: ::prost::alloc::string::String,
    ///all for all or comma separated values. blank is all
    #[prost(string, tag = "3")]
    pub kinds: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub bql: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelResourcesRequest {
    ///blank is latest
    #[prost(string, tag = "1")]
    pub revision: ::prost::alloc::string::String,
    ///all for all or comma separated values. blank is all
    #[prost(string, tag = "2")]
    pub namespaces: ::prost::alloc::string::String,
    ///all for all or comma separated values. blank is all
    #[prost(string, tag = "3")]
    pub kinds: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub bql: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResourceRequest {
    #[prost(string, tag = "1")]
    pub revision: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub namespace: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub kind: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutResourceRequest {
    #[prost(message, optional, tag = "2")]
    pub resource: ::core::option::Option<Resource>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutResourceResponse {
    #[prost(int32, tag = "1")]
    pub updates: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelResourceRequest {
    #[prost(string, tag = "1")]
    pub revision: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub namespace: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub kind: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelResourceResponse {
    #[prost(int32, tag = "1")]
    pub updates: i32,
}
#[doc = r" Generated client implementations."]
pub mod bda_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct BdaClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BdaClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BdaClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> BdaClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            BdaClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn get_revisions(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRevisionsRequest>,
        ) -> Result<tonic::Response<super::GetRevisionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bda.BDA/GetRevisions");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_namespaces(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNamespacesRequest>,
        ) -> Result<tonic::Response<super::GetNamespacesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bda.BDA/GetNamespaces");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_kinds(
            &mut self,
            request: impl tonic::IntoRequest<super::GetKindsRequest>,
        ) -> Result<tonic::Response<super::GetKindsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bda.BDA/GetKinds");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_resources(
            &mut self,
            request: impl tonic::IntoRequest<super::GetResourcesRequest>,
        ) -> Result<tonic::Response<super::GetResourcesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bda.BDA/GetResources");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn stream_resources(
            &mut self,
            request: impl tonic::IntoRequest<super::GetResourcesRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::Resource>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bda.BDA/StreamResources");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn del_resources(
            &mut self,
            request: impl tonic::IntoRequest<super::DelResourcesRequest>,
        ) -> Result<tonic::Response<super::DelResourceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bda.BDA/DelResources");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_resource(
            &mut self,
            request: impl tonic::IntoRequest<super::GetResourceRequest>,
        ) -> Result<tonic::Response<super::Resource>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bda.BDA/GetResource");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn del_resource(
            &mut self,
            request: impl tonic::IntoRequest<super::DelResourceRequest>,
        ) -> Result<tonic::Response<super::DelResourceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bda.BDA/DelResource");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn put_resource(
            &mut self,
            request: impl tonic::IntoRequest<super::PutResourceRequest>,
        ) -> Result<tonic::Response<super::PutResourceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bda.BDA/PutResource");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod bda_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with BdaServer."]
    #[async_trait]
    pub trait Bda: Send + Sync + 'static {
        async fn get_revisions(
            &self,
            request: tonic::Request<super::GetRevisionsRequest>,
        ) -> Result<tonic::Response<super::GetRevisionsResponse>, tonic::Status>;
        async fn get_namespaces(
            &self,
            request: tonic::Request<super::GetNamespacesRequest>,
        ) -> Result<tonic::Response<super::GetNamespacesResponse>, tonic::Status>;
        async fn get_kinds(
            &self,
            request: tonic::Request<super::GetKindsRequest>,
        ) -> Result<tonic::Response<super::GetKindsResponse>, tonic::Status>;
        async fn get_resources(
            &self,
            request: tonic::Request<super::GetResourcesRequest>,
        ) -> Result<tonic::Response<super::GetResourcesResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the StreamResources method."]
        type StreamResourcesStream: futures_core::Stream<Item = Result<super::Resource, tonic::Status>>
            + Send
            + 'static;
        async fn stream_resources(
            &self,
            request: tonic::Request<super::GetResourcesRequest>,
        ) -> Result<tonic::Response<Self::StreamResourcesStream>, tonic::Status>;
        async fn del_resources(
            &self,
            request: tonic::Request<super::DelResourcesRequest>,
        ) -> Result<tonic::Response<super::DelResourceResponse>, tonic::Status>;
        async fn get_resource(
            &self,
            request: tonic::Request<super::GetResourceRequest>,
        ) -> Result<tonic::Response<super::Resource>, tonic::Status>;
        async fn del_resource(
            &self,
            request: tonic::Request<super::DelResourceRequest>,
        ) -> Result<tonic::Response<super::DelResourceResponse>, tonic::Status>;
        async fn put_resource(
            &self,
            request: tonic::Request<super::PutResourceRequest>,
        ) -> Result<tonic::Response<super::PutResourceResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct BdaServer<T: Bda> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Bda> BdaServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BdaServer<T>
    where
        T: Bda,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/bda.BDA/GetRevisions" => {
                    #[allow(non_camel_case_types)]
                    struct GetRevisionsSvc<T: Bda>(pub Arc<T>);
                    impl<T: Bda> tonic::server::UnaryService<super::GetRevisionsRequest> for GetRevisionsSvc<T> {
                        type Response = super::GetRevisionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRevisionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_revisions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRevisionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/bda.BDA/GetNamespaces" => {
                    #[allow(non_camel_case_types)]
                    struct GetNamespacesSvc<T: Bda>(pub Arc<T>);
                    impl<T: Bda> tonic::server::UnaryService<super::GetNamespacesRequest> for GetNamespacesSvc<T> {
                        type Response = super::GetNamespacesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetNamespacesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_namespaces(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNamespacesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/bda.BDA/GetKinds" => {
                    #[allow(non_camel_case_types)]
                    struct GetKindsSvc<T: Bda>(pub Arc<T>);
                    impl<T: Bda> tonic::server::UnaryService<super::GetKindsRequest> for GetKindsSvc<T> {
                        type Response = super::GetKindsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetKindsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_kinds(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetKindsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/bda.BDA/GetResources" => {
                    #[allow(non_camel_case_types)]
                    struct GetResourcesSvc<T: Bda>(pub Arc<T>);
                    impl<T: Bda> tonic::server::UnaryService<super::GetResourcesRequest> for GetResourcesSvc<T> {
                        type Response = super::GetResourcesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetResourcesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_resources(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetResourcesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/bda.BDA/StreamResources" => {
                    #[allow(non_camel_case_types)]
                    struct StreamResourcesSvc<T: Bda>(pub Arc<T>);
                    impl<T: Bda> tonic::server::ServerStreamingService<super::GetResourcesRequest>
                        for StreamResourcesSvc<T>
                    {
                        type Response = super::Resource;
                        type ResponseStream = T::StreamResourcesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetResourcesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stream_resources(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamResourcesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/bda.BDA/DelResources" => {
                    #[allow(non_camel_case_types)]
                    struct DelResourcesSvc<T: Bda>(pub Arc<T>);
                    impl<T: Bda> tonic::server::UnaryService<super::DelResourcesRequest> for DelResourcesSvc<T> {
                        type Response = super::DelResourceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DelResourcesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).del_resources(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DelResourcesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/bda.BDA/GetResource" => {
                    #[allow(non_camel_case_types)]
                    struct GetResourceSvc<T: Bda>(pub Arc<T>);
                    impl<T: Bda> tonic::server::UnaryService<super::GetResourceRequest> for GetResourceSvc<T> {
                        type Response = super::Resource;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetResourceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_resource(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetResourceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/bda.BDA/DelResource" => {
                    #[allow(non_camel_case_types)]
                    struct DelResourceSvc<T: Bda>(pub Arc<T>);
                    impl<T: Bda> tonic::server::UnaryService<super::DelResourceRequest> for DelResourceSvc<T> {
                        type Response = super::DelResourceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DelResourceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).del_resource(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DelResourceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/bda.BDA/PutResource" => {
                    #[allow(non_camel_case_types)]
                    struct PutResourceSvc<T: Bda>(pub Arc<T>);
                    impl<T: Bda> tonic::server::UnaryService<super::PutResourceRequest> for PutResourceSvc<T> {
                        type Response = super::PutResourceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PutResourceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).put_resource(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PutResourceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Bda> Clone for BdaServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Bda> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Bda> tonic::transport::NamedService for BdaServer<T> {
        const NAME: &'static str = "bda.BDA";
    }
}
