FROM node:18.12.1 as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as langspace

RUN apt-get install -y \
    clang \
    cmake \
    git \
    llvm \
    wget

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

FROM langspace as builder-base

RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown wasm32-wasi --toolchain nightly

RUN npm install -g wasm-pack

FROM builder-base as builder

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN npm install && npm run build

FROM builder as development

ENV MODE="stag"

EXPOSE 3000
CMD [ "npm", "run", "dev" ]

FROM builder as production

ENV MODE="production"

EXPOSE 3000
CMD ["npm", "run", "start"]
