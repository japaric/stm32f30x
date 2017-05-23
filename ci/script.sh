set -ex

main() {
    local svd=STMicro/STM32F30x.svd

    if [ $TARGET = x86_64-unknown-linux-gnu ]; then
        cargo check

        # check than the patch can be applied to the original SVD
        local url=https://github.com/posborne/cmsis-svd/raw/python-0.4/data/$svd
        local td=$(mktemp -d)
        local svd=$(basename $svd)
        local patch="${svd%.*}.patch"
        curl -LO $url
        dos2unix $svd
        patch -p1 $svd < $patch

        rm -rf $td
    else
        xargo check --target $TARGET
    fi
}

main
