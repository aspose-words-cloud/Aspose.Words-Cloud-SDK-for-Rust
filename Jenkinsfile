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

def installCiTools() {
    sh 'rustup component add rustfmt clippy'
    sh 'mkdir -p .ci-bin'
    sh 'curl -LsSf https://get.nexte.st/0.9.137/linux | tar zxf - -C .ci-bin'
}

def runTests() {
    try {
        sh 'cargo nextest run --manifest-path tests/Cargo.toml --config-file .config/nextest.toml --profile ci'
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

            if (packageTesting) {
                docker.image('rust:1.88').inside {
                    stage('prepare package testing') {
                        sh 'rm -rf src'
                        sh 'cp tests/Cargo.package-testing.toml tests/Cargo.toml'
                        installCiTools()
                    }

                    withEnv(["PATH+NEXTEST=${pwd()}/.ci-bin"]) {
                        stage('format') {
                            sh 'cargo fmt --manifest-path tests/Cargo.toml -- --check'
                        }

                        stage('lint') {
                            sh 'cargo clippy --manifest-path tests/Cargo.toml --all-targets -- -D warnings'
                        }

                        stage('tests') {
                            runTests()
                        }
                    }
                }
            } else if (needToBuild) {
                docker.image('rust:1.88').inside {
                    stage('prepare') {
                        installCiTools()
                    }

                    withEnv(["PATH+NEXTEST=${pwd()}/.ci-bin"]) {
                        stage('format') {
                            sh 'cargo fmt -- --check'
                            sh 'cargo fmt --manifest-path tests/Cargo.toml -- --check'
                        }

                        stage('lint') {
                            sh 'cargo clippy --all-targets -- -D warnings'
                            sh 'cargo clippy --manifest-path tests/Cargo.toml --all-targets -- -D warnings'
                        }

                        stage('package') {
                            sh 'cargo package'
                        }

                        stage('tests') {
                            runTests()
                        }
                    }
                }
            }
        } finally {
            deleteDir()
            cleanWs()
        }
    }
}
