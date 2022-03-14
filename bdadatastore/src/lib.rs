use std::sync::Arc;

use bdacore;
use bdacore::data::EntityKind;
use bdaproto::bda_server::Bda;
use bdaproto::{
    self, DelResourceRequest, DelResourceResponse, DelResourcesRequest, GetKindsRequest,
    GetKindsResponse, GetNamespacesRequest, GetNamespacesResponse, GetResourceRequest,
    GetResourcesRequest, GetResourcesResponse, GetRevisionsRequest, GetRevisionsResponse,
    PutResourceRequest, PutResourceResponse, Resource,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{async_trait, Response, Status};

pub struct BDADatastoreService {
    data: Arc<bdacore::data::Data>,
}

#[async_trait]
impl Bda for BDADatastoreService {
    async fn get_revisions(
        &self,
        _request: tonic::Request<GetRevisionsRequest>,
    ) -> Result<tonic::Response<GetRevisionsResponse>, Status> {
        self.data
            .values_as_string(&EntityKind::Resource, ".version")
            .map_err(|e| Status::internal(e.to_string()))
            .and_then(|iter| {
                Ok(Response::new(GetRevisionsResponse {
                    revisions: iter.collect(),
                }))
            })
    }

    async fn get_namespaces(
        &self,
        _request: tonic::Request<GetNamespacesRequest>,
    ) -> Result<tonic::Response<GetNamespacesResponse>, tonic::Status> {
        self.data
            .values_as_string(&EntityKind::Resource, ".namespace")
            .map_err(|e| Status::internal(e.to_string()))
            .and_then(|iter| {
                Ok(Response::new(GetNamespacesResponse {
                    namespaces: iter.collect(),
                }))
            })
    }

    async fn get_kinds(
        &self,
        _request: tonic::Request<GetKindsRequest>,
    ) -> Result<tonic::Response<GetKindsResponse>, tonic::Status> {
        Ok(Response::new(GetKindsResponse {
            kinds: bdacore::logic::resource_kinds_iter().collect(),
        }))
    }

    async fn get_resources(
        &self,
        request: tonic::Request<GetResourcesRequest>,
    ) -> Result<tonic::Response<GetResourcesResponse>, tonic::Status> {
        bdacore::logic::query_from_get_resources_request(request.get_ref())
            .and_then(|ref query| self.data.resources(&query))
            .map_err(|e| tonic::Status::internal(e.to_string()))
            .and_then(|rs| Ok(Response::new(GetResourcesResponse { resources: rs })))
    }

    type StreamResourcesStream = ReceiverStream<Result<Resource, Status>>;
    async fn stream_resources(
        &self,
        request: tonic::Request<GetResourcesRequest>,
    ) -> Result<tonic::Response<Self::StreamResourcesStream>, tonic::Status> {
        let (tx, rx) = mpsc::channel(4);
        //TODO: stream should be direct from database. should refactor to acomplish it
        let items = bdacore::logic::query_from_get_resources_request(request.get_ref())
            .and_then(|ref query| self.data.resources(&query))
            .map_err(|e| tonic::Status::internal(e.to_string()))?;
        tokio::spawn(async move {
            for item in items {
                if let Err(e) = tx.send(Ok(item)).await {
                    eprintln!("{:?}", e);
                    return;
                }
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn del_resources(
        &self,
        _request: tonic::Request<DelResourcesRequest>,
    ) -> Result<tonic::Response<DelResourceResponse>, tonic::Status> {
        unimplemented!()
    }
    async fn get_resource(
        &self,
        _request: tonic::Request<GetResourceRequest>,
    ) -> Result<tonic::Response<Resource>, tonic::Status> {
        unimplemented!()
    }
    async fn del_resource(
        &self,
        _request: tonic::Request<DelResourceRequest>,
    ) -> Result<tonic::Response<DelResourceResponse>, tonic::Status> {
        unimplemented!()
    }
    async fn put_resource(
        &self,
        _request: tonic::Request<PutResourceRequest>,
    ) -> Result<tonic::Response<PutResourceResponse>, tonic::Status> {
        unimplemented!()
    }
}
