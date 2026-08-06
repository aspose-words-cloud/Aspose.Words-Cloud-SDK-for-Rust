properties([
    gitLabConnection('gitlab'),
    [$class: 'ParametersDefinitionProperty',
        parameterDefinitions: [
            [$class: 'StringParameterDefinition', name: 'branch', defaultValue: 'master', description: 'the branch to build'],
            [$class: 'StringParameterDefinition', name: 'apiUrl', defaultValue: 'https://api-qa.aspose.cloud', description: 'api url'],
            [$class: 'BooleanParameterDefinition', name: 'ignoreCiSkip', defaultValue: false, description: 'ignore CI Skip'],
            [$class: 'StringParameterDefinition', name: 'credentialsId', defaultValue: '6839cbe8-39fa-40c0-86ce-90706f0bae5d', description: 'credentials id'],
            [$class: 'BooleanParameterDefinition', name: 'packageTesting', defaultValue: false, description: 'Testing package from repository without local sources. Used for prodhealthcheck'],
        ]
    ]
])

def needToBuild = false
def packageTesting = false
def ciImageName = null
def targetCacheVolume = null
def dependencyCacheKey = null

def installCiTools() {
    sh 'mkdir -p .ci-bin'
    sh 'curl -LsSf https://get.nexte.st/0.9.137/linux | tar zxf - -C .ci-bin'
    sh 'test -x .ci-bin/cargo-nextest'
    sh '.ci-bin/cargo-nextest --version'
}

def runTests() {
    try {
        sh '.ci-bin/cargo-nextest nextest run --manifest-path tests/Cargo.toml --config-file .config/nextest.toml --profile ci --cargo-profile ci-cd --locked'
    } finally {
        junit '**/target/nextest/ci/junit.xml'
    }
}

node('words-linux') {
    cleanWs()
    dir('rust') {
        try {
            stage('checkout') {
                checkout([$class: 'GitSCM', branches: [[name: params.branch]], doGenerateSubmoduleConfigurations: false, extensions: [], submoduleCfg: [], userRemoteConfigs: [[credentialsId: '361885ba-9425-4230-950e-0af201d90547', url: 'https://git.auckland.dynabic.com/words-cloud/words-cloud-rust.git']]])

                sh 'git show -s HEAD > gitMessage'
                def commitMessage = readFile('gitMessage').trim()
                echo commitMessage
                needToBuild = params.ignoreCiSkip || !commitMessage.contains('[ci skip]')
                packageTesting = params.packageTesting
                sh 'git clean -fdx'

                if (needToBuild || packageTesting) {
                    withCredentials([usernamePassword(credentialsId: params.credentialsId, passwordVariable: 'ClientSecret', usernameVariable: 'ClientId')]) {
                        def credentialsJson = groovy.json.JsonOutput.toJson([
                            ClientId: env.ClientId,
                            ClientSecret: env.ClientSecret,
                            BaseUrl: params.apiUrl,
                        ])
                        sh 'mkdir -p settings'
                        writeFile file: 'settings/servercreds.json', text: credentialsJson
                    }
                }
            }

            if (packageTesting || needToBuild) {
                docker.image('rust:1.88').inside {
                    stage('prepare') {
                        if (packageTesting) {
                            sh 'cp tests/Cargo.package-testing.toml tests/Cargo.toml'
                        }

                        installCiTools()
                        sh 'cargo generate-lockfile --manifest-path tests/Cargo.toml'

                        dependencyCacheKey = sh(
                            script: 'sha256sum Dockerfile.ci Cargo.toml tests/Cargo.toml tests/Cargo.lock | sha256sum',
                            returnStdout: true
                        ).trim().tokenize()[0].take(16)
                        ciImageName = packageTesting ? 'aspose-words-cloud-rust-ci:rust-1.88-package-testing' : 'aspose-words-cloud-rust-ci:rust-1.88'
                        targetCacheVolume = packageTesting ? 'aspose-words-cloud-rust-package-testing-target' : 'aspose-words-cloud-rust-target'
                        echo "Using Rust target cache volume ${targetCacheVolume}"
                    }
                }

                def ciImage
                stage('image') {
                    ciImage = docker.build(ciImageName, '--pull --file Dockerfile.ci .')
                }

                stage('cache') {
                    withEnv([
                        "RUST_DEPENDENCY_CACHE_KEY=${dependencyCacheKey}",
                        "RUST_TARGET_CACHE_VOLUME=${targetCacheVolume}",
                        "RUST_CI_IMAGE=${ciImageName}"
                    ]) {
                        sh '''docker run --rm \
                            --env RUST_DEPENDENCY_CACHE_KEY \
                            --volume "$RUST_TARGET_CACHE_VOLUME:/cache" \
                            "$RUST_CI_IMAGE" \
                            sh -c 'cached_key="$(cat /cache/.dependency-cache-key 2>/dev/null || true)"; \
                                if test "$cached_key" != "$RUST_DEPENDENCY_CACHE_KEY"; then \
                                    find /cache -mindepth 1 -delete; \
                                    cp -a "$CARGO_TARGET_DIR/." /cache/; \
                                    printf "%s\\n" "$RUST_DEPENDENCY_CACHE_KEY" > /cache/.dependency-cache-key; \
                                fi' '''
                    }
                }

                ciImage.inside("--volume ${targetCacheVolume}:/opt/rust-ci-target") {
                    stage('build') {
                        sh 'find src tests/src -type f -exec touch {} +'
                        sh 'cargo build --manifest-path tests/Cargo.toml --profile ci-cd --all-targets --locked'
                    }

                    stage('tests') {
                        runTests()
                    }
                }
            }
        } finally {
            deleteDir()
            cleanWs()
        }
    }
}
