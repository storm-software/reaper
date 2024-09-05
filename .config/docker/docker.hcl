variable "RELEASE" {
  default = "dev"
}

variable "PWD" {
  default = "."
}

variable "DOCKER_REGISTRY" {
  default = ""
}

variable "COMMIT_SHA" {
  default = ""
}

variable "BRANCH_NAME" {
  default = ""
}

variable "BUILD_TYPE" {
  # Can be "", "ci" or "publish"
  default = ""
}

variable "BUILD_STABLE" {
  # Can be "" or "1"
  default = ""
}

function "get_target" {
  params = []
  result = notequal("", BUILD_TYPE) ? notequal("ci", BUILD_TYPE) ? "target-publish" : "target-ci" : "target-dev"
}

function "local_image_tag" {
  params = [name]
  result = equal("", BUILD_TYPE) ? "${DOCKER_REGISTRY}${name}:latest" : ""
}

function "stable_image_tag" {
  params = [name]
  result = equal("1", BUILD_STABLE) ? "${DOCKER_REGISTRY}${name}:latest" : ""
}

function "image_tag" {
  params = [name, tag]
  result = notequal("", tag) ? "${DOCKER_REGISTRY}${name}:${tag}" : ""
}

target "migrations-base" {
  dockerfile = "${PWD}/docker/migrations.dockerfile"
  args = {
    RELEASE = "${RELEASE}"
  }
}

target "service-base" {
  dockerfile = "${PWD}/docker/services.dockerfile"
  args = {
    RELEASE = "${RELEASE}"
  }
}

target "target-dev" {}

target "target-ci" {
  cache-from = ["type=gha,ignore-error=true"]
  cache-to = ["type=gha,mode=max,ignore-error=true"]
}

target "target-publish" {
  platforms = ["linux/amd64", "linux/arm64"]
  cache-from = ["type=gha,ignore-error=true"]
  cache-to = ["type=gha,mode=max,ignore-error=true"]
}

target "eth-engine" {
  inherits = ["service-base", get_target()]
  contexts = {
    dist = "${PWD}/dist/apps/eth-engine"
    shared = "${PWD}/docker/shared"
  }
  args = {
    SERVICE_DIR_NAME = "reaper-eth-engine"
    IMAGE_TITLE = "reaper/eth-engine"
    IMAGE_DESCRIPTION = "The usage ingestor service of the GraphQL Hive project."
    PORT = "3007"
    HEALTHCHECK_CMD = "wget --spider -q http://127.0.0.1:$${PORT}/_readiness"
  }
  tags = [
    local_image_tag("usage-ingestor"),
    stable_image_tag("usage-ingestor"),
    image_tag("usage-ingestor", COMMIT_SHA),
    image_tag("usage-ingestor", BRANCH_NAME)
  ]
}
