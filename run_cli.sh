# Run CLI with wrong file
CMD=./target/debug/rudoko 

# Help message
${CMD} --help

# Wrong file
${CMD} -i nofile.txt

# Correct file
${CMD} -i resources/simple1.txt

# Correct file and limitation
${CMD} -i resources/simple1.txt -n 100