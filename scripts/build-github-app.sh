#!/bin/bash
# Run setSecret for APP_ID, PRIVATE_KEY, INSTALLATION_ID, CLIENT_PAYLOAD, EVENT_TYPE
# and store the returned IDs in variables
set -euo pipefail


query() {
    jq -sRn '{query: input}' | \
    curl -s \
        -u $DAGGER_SESSION_TOKEN: \
        -H "content-type:application/json" \
        -H "X-Debug:true" \
        -d@- http://127.0.0.1:$DAGGER_SESSION_PORT/query | \
    jq -r .data
}


S_REG_PASSWORD=$(query <<EOF | jq -r .setSecret.id
{
  setSecret(
        name: "REG_PASSWORD",
        plaintext: "$REG_PASSWORD"
    ) {
        id
    }
}
EOF
)
echo "S_APP_ID: $S_REG_PASSWORD"

S_APP_ID=$(query <<EOF | jq -r .setSecret.id
{
  setSecret(
        name: "APP_ID",
        plaintext: "$APP_ID"
    ) {
        id
    }
}
EOF
)
echo "S_APP_ID: $S_APP_ID"

S_PRIVATE_KEY=$(query <<EOF | jq -r .setSecret.id
{
    setSecret(
        name: "PRIVATE_KEY",
        plaintext: "$PRIVATE_KEY"
    ) {
        id
    }
}
EOF
)

echo "S_PRIVATE_KEY: $S_PRIVATE_KEY"

S_INSTALLATION_ID=$(query <<EOF | jq -r .setSecret.id
{
    setSecret(
        name: "INSTALLATION_ID",
        plaintext: "$INSTALLATION_ID"
    ) {
        id
    }
}
EOF
)
echo "S_INSTALLATION_ID: $S_INSTALLATION_ID"

S_CLIENT_PAYLOAD=$(query <<EOF | jq -r .setSecret.id
{
    setSecret(
        name: "CLIENT_PAYLOAD",
        plaintext: "$(echo $CLIENT_PAYLOAD | sed 's/"/\\"/g' )"
    ) {
        id
    }
}
EOF
)
echo "S_CLIENT_PAYLOAD: $S_CLIENT_PAYLOAD"

S_EVENT_TYPE=$(query <<EOF | jq -r .setSecret.id
{
    setSecret(
        name: "EVENT_TYPE",
        plaintext: "$EVENT_TYPE"
    ) {
        id
    }
}
EOF
)
echo "S_EVENT_TYPE: $S_EVENT_TYPE"

S_SOURCE_DIR=$(query <<EOF | jq -r .host.directory.id
{

        host{
            directory(path: "scripts/github"){
                id
            }
        }
}
EOF
)
echo "S_SOURCE_DIR: $S_SOURCE_DIR"



TEST_OUT=$(query <<EOF | jq -r .container.withRegistryAuth.from.withDirectory.withWorkdir.withSecretVariable.withSecretVariable.withSecretVariable.withSecretVariable.withSecretVariable.withEnvVariable.withExec.stdout
{
  container {
    withRegistryAuth(address: "$REG_ADDRESS" , username: "$REG_USERNAME", secret:"$S_REG_PASSWORD"){
    from(address: "$REG_ADDRESS") {
      withDirectory(directory: "$S_SOURCE_DIR", path: "/app") {
        withWorkdir(path: "/app") {
          withSecretVariable(name: "APP_ID", secret: "$S_APP_ID") {
            withSecretVariable(name: "PRIVATE_KEY", secret: "$S_PRIVATE_KEY") {
              withSecretVariable(name: "INSTALLATION_ID", secret: "$S_INSTALLATION_ID") {
                withSecretVariable(name: "CLIENT_PAYLOAD", secret: "$S_CLIENT_PAYLOAD") {
                  withSecretVariable(name: "EVENT_TYPE", secret: "$S_EVENT_TYPE") {
                     withEnvVariable(name: "CACHE_BUSTER", value: "$(date)") {
                      withExec(args: [ "index.js"]) {
                        stdout
                      }
                    }
                  }
                }
              }
            }
          }
         }
        }
      }
    }
  }
}
EOF
)

echo "$TEST_OUT"
