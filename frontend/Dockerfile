FROM node:lts AS development

ENV CI=true
ENV PORT=5173

WORKDIR /code
COPY package.json /code/package.json
COPY package-lock.json /code/package-lock.json
RUN npm ci

COPY . /code
RUN npm run build

EXPOSE 5173

CMD [ "npm", "run", "dev" ]

FROM development as dev-envs
RUN <<EOF
apt-get update
apt-get install -y --no-install-recommends git
EOF

RUN <<EOF
useradd -s /bin/bash -m vscode
groupadd docker
usermod -aG docker vscode
EOF
# install Docker tools (cli, buildx, compose)
COPY --from=gloursdocker/docker / /
CMD [ "npm", "run", "dev" ]

FROM development AS builder

RUN npm run build

FROM nginx:1.13-alpine

COPY --from=builder /code/build /usr/share/nginx/html