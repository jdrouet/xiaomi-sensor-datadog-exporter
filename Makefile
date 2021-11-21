build-binary-in-docker:
	docker buildx build \
		--platform ${PLATFORM} \
		--tag=xiaomi-sensor-exporter:builder \
		--target=binary \
		--output type=local,dest=$(shell pwd)/target/ \
		.
