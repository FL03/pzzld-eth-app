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
CMD [ "npm", "run", "start" ]

# FROM node:18.12.1-slim

# COPY --from=builder /workspace/build /app/build
# COPY --from=builder /workspace/package.json /app/package.json
# COPY --from=builder /workspace/packages/contracts /app/packages/contracts

# WORKDIR /app

# EXPOSE 3000
# CMD ["node", "build/index.js"]
