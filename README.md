# BDA

Build and Deploy Applications

**STATUS** : Experimental and WIP

Components:

- BDA PROTO: Provides a common domain language to declare software products, dependencies between components, with instructions on how to build, test and deploy
- BDA DATASTORE: Provides a datastore to persist bda proto with versioning support. It should have a cli and webapp admin clients
- BDA ENGINE: Provides a distributed environment to execute bda instructions
- BDA CONFIGURATORS: Configure ci/cd tools to work with bda engine

```mermaid
graph TD
    BDACORE[Bda Core Module]
    BDAENGINE[BDA Engine]
    DATASTORE[BDA Datastore]
    EXTERNALTOOL[Option External Tool As Engine]
    INDEX[(INDEX)]
    MEMKV[(MEM KV)]
    DISTRKV[(DISTRIBUTED KV)]
    BDAQL
    BDAWEB[BDA WEB/REST]
    BDACLI
    BDACLI-->|start/stop|BDAWEB
    BDACLI-->|start/stop/consume grpc|DATASTORE
    BDACLI-->|start/stop/consume grpc|BDAENGINE
    BDAWEB-->|consume grpc|DATASTORE
    BDAWEB-->|consume grpc|BDAENGINE
    BDAENGINE-->|consume grpc|DATASTORE
    DATASTORE-->|search with bdaql|BDAQL
    DATASTORE-->|shared code|BDACORE
    BDAENGINE-->|shared code|BDACORE
    BDACLI-->|shared code|BDACORE
    BDAWEB-->|shared code|BDACORE
    BDAQL-->INDEX
    INDEX-->|put/get|MEMKV
    DATASTORE-->|put/get/del|BDAPROTO
    BDAPROTO-->INDEX
    BDAPROTO-->DISTRKV
    EXTERNALTOOL-->|option consume rest|BDAWEB
    EXTERNALTOOL-->|option consume cli|BDACLI
    EXTERNALTOOL-->|option consume grpc|DATASTORE
```

[![](https://mermaid.ink/img/pako:eNqNVN9PqzAU_leaPmmi4X0PJmxtDJENZb1qMnyo9DhIoMVS4r0R__dbOkCmc5OXnh_f-Tjn8JV3nCoBeIa3mlcZYiSRyD5z4i-imG7mgqOF0oCWSjQFPI1ZuroOVjZPfETlNpdDivjMXzNXalOEG14bW99n6SOj8coPWRSFm6gyuZKI_jWgJS8QU6pAfr3PF6wIfdycueO8jy3p8uZ-c2YPdHM_BEmwZnEXdkYw_8MomaRtN3fhaD7QuevPnl5M1-wTtAiDqX15edXWhmvj2TGqdlf6M8BLlaybEtBWV2k77uK3BeNep412BSdoD6G-c-3cU3Sj6xoFrtMMveUmQ8-CvxbtZI_7yIxrEKgTU9uL58CLj6GG3RyB9HMeg9yFFuHUMtFPV1Q1xtuCaZ14DgzQ5z0BbsjbOGLRyOq8feJJtJfed4l3vGqn8mHlGmqzJ6MT-LTI26ksT8C_ftFE4gtcgi55Luwlf-9IEmwyKCHBM2sKeOFNYRKcyA8LbSrBDVCR20uLZy-8qOEC88ao9T-Z4pnRDQwgknP7zyh71Md_WKhS1A)](https://mermaid-js.github.io/mermaid-live-editor/edit/#pako:eNqNVN9PqzAU_leaPmmi4X0PJmxtDJENZb1qMnyo9DhIoMVS4r0R__dbOkCmc5OXnh_f-Tjn8JV3nCoBeIa3mlcZYiSRyD5z4i-imG7mgqOF0oCWSjQFPI1ZuroOVjZPfETlNpdDivjMXzNXalOEG14bW99n6SOj8coPWRSFm6gyuZKI_jWgJS8QU6pAfr3PF6wIfdycueO8jy3p8uZ-c2YPdHM_BEmwZnEXdkYw_8MomaRtN3fhaD7QuevPnl5M1-wTtAiDqX15edXWhmvj2TGqdlf6M8BLlaybEtBWV2k77uK3BeNep412BSdoD6G-c-3cU3Sj6xoFrtMMveUmQ8-CvxbtZI_7yIxrEKgTU9uL58CLj6GG3RyB9HMeg9yFFuHUMtFPV1Q1xtuCaZ14DgzQ5z0BbsjbOGLRyOq8feJJtJfed4l3vGqn8mHlGmqzJ6MT-LTI26ksT8C_ftFE4gtcgi55Luwlf-9IEmwyKCHBM2sKeOFNYRKcyA8LbSrBDVCR20uLZy-8qOEC88ao9T-Z4pnRDQwgknP7zyh71Md_WKhS1A)
