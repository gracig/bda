use bdacore::data::datastore::mem::MemDatastore;
use bdacore::data::query::Query;
use bdacore::data::{self, EntityKind};
use bdacore::{self, logic};
use bdaindex::backend::llrb::LLRBBackend;
use bdaindex::backend::lmdb::LMDBBackend;
use bdaindex::backend::Backend;
use bdaproto::bda_server::Bda;
use bdaproto::{
    self, DelResourceRequest, DelResourceResponse, DelResourcesRequest, GetKindsRequest,
    GetKindsResponse, GetNamespacesRequest, GetNamespacesResponse, GetResourceRequest,
    GetResourcesRequest, GetResourcesResponse, GetVersionsRequest, GetVersionsResponse,
    PutResourceRequest, PutResourceResponse, Resource,
};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{async_trait, Response, Status};

fn _lmdb() -> impl Backend {
    let s = shellexpand::tilde("~/.bda/data").to_string();
    let path = Path::new(&s);
    fs::create_dir_all(path).unwrap();
    LMDBBackend::new(path).unwrap()
}
fn _llrb() -> impl Backend {
    LLRBBackend::new()
}

pub struct BDADatastoreService {
    data: Arc<bdacore::data::Data>,
}
impl BDADatastoreService {
    pub fn new_mem() -> BDADatastoreService {
        BDADatastoreService {
            data: Arc::new(data::new(Arc::new(MemDatastore::new(Arc::new(_lmdb()))))),
        }
    }
}

#[async_trait]
impl Bda for BDADatastoreService {
    async fn get_versions(
        &self,
        _request: tonic::Request<GetVersionsRequest>,
    ) -> Result<tonic::Response<GetVersionsResponse>, Status> {
        self.data
            .values_as_string(&EntityKind::Resource, ".version")
            .and_then(|mut iter| {
                Ok(Response::new(GetVersionsResponse {
                    versions: iter.try_fold(Vec::new(), |mut acc, item| {
                        item.and_then(|item| {
                            acc.push(item);
                            Ok(acc)
                        })
                    })?,
                }))
            })
            .map_err(|e| Status::internal(e.to_string()))
    }

    async fn get_namespaces(
        &self,
        _request: tonic::Request<GetNamespacesRequest>,
    ) -> Result<tonic::Response<GetNamespacesResponse>, tonic::Status> {
        self.data
            .values_as_string(&EntityKind::Resource, ".namespace")
            .and_then(|mut iter| {
                Ok(Response::new(GetNamespacesResponse {
                    namespaces: iter.try_fold(Vec::new(), |mut acc, item| {
                        item.and_then(|item| {
                            acc.push(item);
                            Ok(acc)
                        })
                    })?,
                }))
            })
            .map_err(|e| Status::internal(e.to_string()))
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
        Query::from_get_resources_request(request.get_ref())
            .map_err(|e| tonic::Status::internal(e.to_string()))
            .and_then(|ref query| {
                self.data
                    .resources(&query)
                    .and_then(|rs| Ok(Response::new(GetResourcesResponse { resources: rs })))
                    .map_err(|e| tonic::Status::internal(e.to_string()))
            })
    }

    type StreamResourcesStream = ReceiverStream<Result<Resource, Status>>;
    async fn stream_resources(
        &self,
        request: tonic::Request<GetResourcesRequest>,
    ) -> Result<tonic::Response<Self::StreamResourcesStream>, tonic::Status> {
        let (tx, rx) = mpsc::channel(4);
        let items = Query::from_get_resources_request(request.get_ref())
            .map_err(|e| tonic::Status::internal(e.to_string()))
            .and_then(|ref query| {
                self.data
                    .resources(&query)
                    .and_then(|rs| Ok(rs))
                    .map_err(|e| tonic::Status::internal(e.to_string()))
            })?;
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
        request: tonic::Request<DelResourcesRequest>,
    ) -> Result<tonic::Response<DelResourceResponse>, tonic::Status> {
        Query::from_del_resources_request(request.get_ref())
            .map_err(|e| tonic::Status::internal(e.to_string()))
            .and_then(|ref query| {
                self.data
                    .search(&query)
                    .map_err(|e| tonic::Status::internal(e.to_string()))
            })
            .and_then(|mut iter| {
                iter.try_fold(0 as i32, |acc, ri| {
                    ri.and_then(|ref id| {
                        self.data.del(id).and_then(|op| {
                            if let Some(bdacore::data::Op::Delete { .. }) = op {
                                Ok(acc + 1)
                            } else {
                                Ok(acc)
                            }
                        })
                    })
                })
                .map_err(|e| tonic::Status::internal(e.to_string()))
            })
            .and_then(|updates| Ok(Response::new(DelResourceResponse { updates })))
    }

    async fn get_resource(
        &self,
        request: tonic::Request<GetResourceRequest>,
    ) -> Result<tonic::Response<Resource>, tonic::Status> {
        logic::resource_id_from_get_request(request.get_ref())
            .map_err(|e| tonic::Status::internal(e.to_string()))
            .and_then(|ref id| {
                self.data
                    .get_resource(id)
                    .map_err(|e| tonic::Status::internal(e.to_string()))
            })
            .and_then(|r| {
                r.ok_or(Err(tonic::Status::internal(format!(
                    "entity not found: {:?}",
                    request.get_ref()
                )))?)
            })
            .and_then(|r| Ok(Response::new(r)))
            .map_err(|e| tonic::Status::internal(e.to_string()))
    }

    async fn del_resource(
        &self,
        request: tonic::Request<DelResourceRequest>,
    ) -> Result<tonic::Response<DelResourceResponse>, tonic::Status> {
        logic::resource_id_from_del_request(request.get_ref())
            .map_err(|e| tonic::Status::internal(e.to_string()))
            .and_then(|id| Ok(self.data.del(&id)))
            .and_then(|rop| {
                rop.map_err(|e| tonic::Status::internal(e.to_string()))
                    .and_then(|op| {
                        if let Some(bdacore::data::Op::Delete { .. }) = op {
                            Ok(1)
                        } else {
                            Ok(0)
                        }
                    })
            })
            .and_then(|updates| Ok(Response::new(DelResourceResponse { updates })))
            .map_err(|e| tonic::Status::internal(e.to_string()))
    }

    async fn put_resource(
        &self,
        request: tonic::Request<PutResourceRequest>,
    ) -> Result<tonic::Response<PutResourceResponse>, tonic::Status> {
        request
            .get_ref()
            .resource
            .as_ref()
            .ok_or_else(|| "put request resource not defined".to_string())
            .map_err(|e| tonic::Status::internal(e.to_string()))
            .and_then(|r| {
                self.data
                    .put_resource(r)
                    .map_err(|e| tonic::Status::internal(e.to_string()))
                    .and_then(|x| match x {
                        Some(bdacore::data::Op::Create { .. }) => Ok(1),
                        Some(bdacore::data::Op::Update { .. }) => Ok(1),
                        _ => Ok(0),
                    })
                    .and_then(|updates| Ok(Response::new(PutResourceResponse { updates })))
            })
    }
}
