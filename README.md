# BDA
Build and Deploy Applications

__STATUS__ : Experimental and WIP

Components:

- BDA PROTO: Provides a common domain language to declare software products, dependencies between components, with instructions on how to build, test and deploy
- BDA DATASTORE: Provides a datastore to persist bda proto with versioning support. It should have a cli and webapp admin clients
- BDA ENGINE: Provides a distributed environment to execute bda instructions
- BDA CONFIGURATORS: Configure ci/cd tools to work with bda engine
