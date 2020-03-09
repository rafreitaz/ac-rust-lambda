runtest:
	cargo test

build-bin:
	docker run --rm \
		-v ${PWD}:/code \
		-v ${HOME}/.cargo/registry:/root/.cargo/registry \
		-v ${HOME}/.cargo/git:/root/.cargo/git \
		softprops/lambda-rust

package: build-bin
	sam package --output-template-file packaged.yaml --s3-bucket ac-rust-lambda

deploy: package
	sam deploy --template-file packaged.yaml --stack-name ac-rust-api --capabilities CAPABILITY_NAMED_IAM