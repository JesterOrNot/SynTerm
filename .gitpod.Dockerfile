FROM gitpod/workspace-full

USER gitpod

RUN bash -cl "cargo install cargo-expand && rustup default nightly"
