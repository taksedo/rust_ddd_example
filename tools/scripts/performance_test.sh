#!/bin/sh

./build_and_run_local_image.sh



docker run -v ${pwd}:/var/loadtest --rm --network host -it direvius/yandex-tank