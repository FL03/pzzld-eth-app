FROM node:18.12.1 as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as langspace

RUN npm install -g ganache-cli hardhat truffle

FROM langspace as builder

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN npm install && npm run build

FROM builder as development

ENV MODE="production"

EXPOSE 3000
CMD [ "npm", "run", "dev" ]

FROM node:18.12.1-slim

COPY --from=builder /workspace/build /app
COPY --from=builder /workspace/packages/contracts/build /app/contracts

CMD [ "node", "app/index.js"]