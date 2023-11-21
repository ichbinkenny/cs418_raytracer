INPUT_FILE=$(basename ${1} .png)
echo $INPUT_FILE
cargo run -- "tests/ray-$INPUT_FILE.txt"
echo "Generating image comparison of ${1} and ray-${1}"
compare -fuzz 2% ${1} "tests/ray-$INPUT_FILE.png" ae.png
composite ${1} "tests/ray-$INPUT_FILE.png" -alpha off -compose difference rawdiff.png
convert rawdiff.png -level 0%,8% diff.png
convert +append "tests/ray-$INPUT_FILE.png" ${1} ae.png rawdiff.png diff.png look_at_this.png