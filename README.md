[![Rust](https://github.com/gracig/bda/actions/workflows/rust.yml/badge.svg)](https://github.com/gracig/bda/actions/workflows/rust.yml)

# BDA

Build and Deploy Applications

**STATUS** : Experimental and WIP

Components:

- BDA WEB: Web interface to interact with BDA DATASTORE and BDA ENGINE
- BDA CLI: Command line interface to interact with BDA DATASTORE and BDA ENGINE
- BDA ENGINE: Provides a scalable environment to execute bda functions
- OPTION EXTERNAL TOOL AS ENGINE: Uses other tools, like Jenkins, to fullfil the ENGINE role
- BDA DATASTORE: Management od bda resources
- BDA CORE: Provides a shared library to be used by all BDA components
- BDA PROTO: Provides a common domain language to declare software products, dependencies between components, with instructions on how to build, test and deploy
- BDA INDEX: Index and provides a query language for bda resources. Support different backends
- BDA KV STORE: Persist Resource. Support different backends


Pre-Requisites (ubuntu)
sudo apt-get install -y libssl-dev build-essential
git submodule init
git submodule update
cargo build

[![](https://mermaid.ink/img/pako:eNqVVMFuozAQ_RXLvbRSolTKnjhUgmCtotBESlBaKaxWLjbBqmOzxqitSv-9BhxIkzYbuHiY997AvBn5HcaSUOjArcJZCkI_EsA8nu9OFku08QgGE6koWKVYUQIC9vSnZaD57-nccHwXILFlglrId0N3FdZyA_lY41ybGhZFjyFazt0gXCyCzSLTTAqAXjVVAnMQSsmBm3-tN5376HFzXR83NhcES--vBaoYeDh-poK0-L3f4SY-wSv9bH1GXIPfKGfrprfr2RrU0U3nyAPy6p7N2SUnwbROmvNLcji8K58IzpTUsmw9-5HRGn74tf_UOGac1mhez5WpTTSEzvGjfOv00fSrolmhR1uqR4Ty0hr3DavICNZ0ZP7gHwc5xSpOy4OKhxtT0WWzNDFnZePTWdpPfdnf2Xc2W5-kmy1o0pGIRK7fOLWjAQnj3LlKkttBrpV8ps7VeDy28fCFEZ06v7LXvajuxmriJL5I0xnbV9hOqqewdajr7jKhvS76yqzXPb204-qraqZ5sQoO4I6qHWbEXI7v1RJEUKd0RyPomJDQBBdcRzASH4babDAizFx00Ekwz-kA4kLL1ZuIoaNVQfckn2Fz1-4s6-MT17--Ug)](https://mermaid-js.github.io/mermaid-live-editor/edit/#pako:eNqVVMFuozAQ_RXLvbRSolTKnjhUgmCtotBESlBaKaxWLjbBqmOzxqitSv-9BhxIkzYbuHiY997AvBn5HcaSUOjArcJZCkI_EsA8nu9OFku08QgGE6koWKVYUQIC9vSnZaD57-nccHwXILFlglrId0N3FdZyA_lY41ybGhZFjyFazt0gXCyCzSLTTAqAXjVVAnMQSsmBm3-tN5376HFzXR83NhcES--vBaoYeDh-poK0-L3f4SY-wSv9bH1GXIPfKGfrprfr2RrU0U3nyAPy6p7N2SUnwbROmvNLcji8K58IzpTUsmw9-5HRGn74tf_UOGac1mhez5WpTTSEzvGjfOv00fSrolmhR1uqR4Ty0hr3DavICNZ0ZP7gHwc5xSpOy4OKhxtT0WWzNDFnZePTWdpPfdnf2Xc2W5-kmy1o0pGIRK7fOLWjAQnj3LlKkttBrpV8ps7VeDy28fCFEZ06v7LXvajuxmriJL5I0xnbV9hOqqewdajr7jKhvS76yqzXPb204-qraqZ5sQoO4I6qHWbEXI7v1RJEUKd0RyPomJDQBBdcRzASH4babDAizFx00Ekwz-kA4kLL1ZuIoaNVQfckn2Fz1-4s6-MT17--Ug)



- Dockerfile
    - pull images before docker pull (faster builders)
    - add .dockerignore <- Add target (avoid mix old binaries)

- Github action rust template added
  - git submodules must be updated. in case a fetch error occurs on ci

    