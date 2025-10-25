#!/usr/bin/env bash

set -euo pipefail

cleanup() {
	set +x
	{
		rm ./*.lst
	} >/dev/null 2>&1
}

trap cleanup EXIT

find . -type f -name main.nix >paths.lst

mapfile -t paths <paths.lst

for path in "${paths[@]}"; do
	# Start: ./path/to/scripts/bin/name/main.nix

	# Start: path/to/scripts/bin/name/main.nix
	path="${path//\.\//}"

	# After: path.to.scripts.bin.name.main.nix
	attr_path="${path//\//.}"

	case "${attr_path}" in
	scripts.bin*)
		# When there is no sub-project (e.g. 'testbed')
		attr_path_prefix=root.
		;;
	*)
		attr_path_prefix=
		;;
	esac

	# After: path.to.name.main.nix
	attr_path="${attr_path//scripts\.bin\./}"

	# After: path.to.name
	attr_path="${attr_path//\.main\.nix/}"

	echo "  ${attr_path_prefix}${attr_path} = ../${path};"
done | sort >attrs.lst

mapfile -t attrs <attrs.lst

{
	echo '{...}: {'
	for attr in "${attrs[@]}"; do
		echo "${attr}"
	done
	echo '}'
} >nix/generated.nix

echo nix/generated.nix
