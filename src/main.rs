#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSessionRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSessionResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitSessionRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitSessionResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestroySessionRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestroySessionResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDistributionGroupsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDistributionGroupsResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDistributionGroupRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDistributionGroupResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRoomListRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRoomListResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddDistributionGroupMemberRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddDistributionGroupMemberResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDistributionGroupMemberRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDistributionGroupMemberResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoomsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoomsResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoomRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoomResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRoomRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRoomResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRoomRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRoomResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveRoomRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveRoomResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistributionGroup {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Room {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaybeString {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaybeStringList {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringList {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaybeI32 {}
pub mod client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct MyClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MyClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> MyClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            MyClient::new(InterceptedService::new(inner, interceptor))
        }

        pub async fn get_session(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSessionRequest>,
        ) -> Result<tonic::Response<super::GetSessionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/anything/GetSession");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn init_session(
            &mut self,
            request: impl tonic::IntoRequest<super::InitSessionRequest>,
        ) -> Result<tonic::Response<super::InitSessionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/anything/InitSession");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn destroy_session(
            &mut self,
            request: impl tonic::IntoRequest<super::DestroySessionRequest>,
        ) -> Result<tonic::Response<super::DestroySessionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/anything/DestroySession");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_distribution_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDistributionGroupsRequest>,
        ) -> Result<tonic::Response<super::GetDistributionGroupsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/anything/GetDistributionGroups");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_distribution_group(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDistributionGroupRequest>,
        ) -> Result<tonic::Response<super::GetDistributionGroupResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/anything/GetDistributionGroup");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_room_list(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRoomListRequest>,
        ) -> Result<tonic::Response<super::CreateRoomListResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/anything/CreateRoomList");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_distribution_group_member(
            &mut self,
            request: impl tonic::IntoRequest<super::AddDistributionGroupMemberRequest>,
        ) -> Result<tonic::Response<super::AddDistributionGroupMemberResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/anything/AddDistributionGroupMember");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_distribution_group_member(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveDistributionGroupMemberRequest>,
        ) -> Result<tonic::Response<super::RemoveDistributionGroupMemberResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/anything/RemoveDistributionGroupMember");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_rooms(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRoomsRequest>,
        ) -> Result<tonic::Response<super::GetRoomsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/anything/GetRooms");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_room(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRoomRequest>,
        ) -> Result<tonic::Response<super::GetRoomResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/anything/GetRoom");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_room(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRoomRequest>,
        ) -> Result<tonic::Response<super::CreateRoomResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/anything/CreateRoom");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_room(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRoomRequest>,
        ) -> Result<tonic::Response<super::SetRoomResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/anything/SetRoom");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}

// Comment/Uncomment this impl to trigger "the issue"
impl<'a> From<&'a str> for MaybeString {
    fn from(val: &'a str) -> Self {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
