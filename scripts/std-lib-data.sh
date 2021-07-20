#!/usr/bin/env bash
./x.py fmt && ./x.py build -i --stage 1 library/std -j16 --keep-stage 1
./scripts/std-lib-regression.sh > std-lib-log.txt
sort std-lib-log.txt | uniq > std-lib-log-unique.txt

for FILE in  std-lib-log.txt std-lib-log-unique.txt
do
    if grep -q "error: internal compiler error: unexpected panic"  $FILE; then
        echo "Panic on building standard library"
        cat $FILE
        exit 1
    fi 
    
    echo $FILE
	echo "Total invocations"
    grep 'Codegen: ' $FILE | wc -l

	echo "Total unimplemented"
    grep 'Unimplemented: ' $FILE | wc -l

    echo "inline assembly"
    grep 'Unimplemented: InlineAsm' $FILE | wc -l

    echo "try intrinsic"
    grep 'Unimplemented: try' $FILE | wc -l

    echo "Skipped functions"
    grep 'Skipping current function:' $FILE | wc -l

    echo "Local calls"
    grep 'WOULD FAIL: local function call' $FILE | wc -l

    echo "Duplicate"
    grep 'WOULD FAIL: duplicate names' $FILE| wc -l

    echo "Dynamic operand"
    grep 'WOULD FAIL: dynamic operand' $FILE | wc -l

    echo "Dynamic source cast"
    grep 'WOULD FAIL: dynamic source cast' $FILE | wc -l
done