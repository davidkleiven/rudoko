FILE="cliHelp.md"

cargo build --release

CMD=target/release/rudoko

echo "# CLI Reference" > $FILE

echo "\`\`\`" >> $FILE
$CMD --help >> $FILE
echo "\`\`\`" >> $FILE