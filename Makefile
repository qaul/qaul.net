.PHONY: bundle-snapfile
bundle-snapfile:
	@docker run --rm --env SNAPCRAFT_CREDENTIALS_KEY --volume $(shell pwd):"/qaul.net" --workdir "/qaul.net" snapcore/snapcraft:edge /bin/bash -c 'sleep 1; cd utilities/installers/linux && bash snapbuild'
	@echo ""
