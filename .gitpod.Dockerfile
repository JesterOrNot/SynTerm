FROM gitpod/workspace-full

USER gitpod

RUN cargo install cargo-expand
