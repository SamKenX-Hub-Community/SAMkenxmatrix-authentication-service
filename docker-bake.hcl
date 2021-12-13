// Target filled by GitHub Actions, one for the regular tag, one for the debug tag
target "docker-metadata-action" {}
target "docker-metadata-action-debug" {}

target "default" {
  dockerfile = "Dockerfile"
  context = "./"
}

target "debug" {
  inherits = ["default"]
  target = "debug"
}

target "test" {
  inherits = ["default"]
  target = "test"
}

target "release" {
  inherits = ["default"]
  platforms = [
    "linux/amd64",
    "linux/arm64",
    "linux/arm",
  ]
}

// This is what is baked by GitHub Actions
group "gha" { targets = ["gha-regular", "gha-debug", "gha-test"] }

target "gha-base" {
  inherits = ["release"]
  cache-from = ["type=gha"]
  cache-to = ["type=gha,mode=max"]
}

// This is filled by GitHub Actions
target "gha-push" {}

target "gha-regular" {
  inherits = ["gha-base", "gha-push", "docker-metadata-action"]
}

target "gha-debug" {
  inherits = ["gha-base", "gha-push", "debug", "docker-metadata-action-debug"]
}

target "gha-test" {
  inherits = ["gha-base", "test"]
}