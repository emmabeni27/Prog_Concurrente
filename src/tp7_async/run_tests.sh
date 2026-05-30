#!/usr/bin/env bash
set -uo pipefail

BINARY="./target/release/tp7_async"
OUT="resultados.csv"

cargo build --release 2>/dev/null

echo "modelo,tipo,tasks,terms,run,time_ms,status" > "$OUT"

run() {
    local model=$1 tipo=$2 tasks=$3 terms=$4 run_n=$5
    local extra_args=""

    if [[ "$tipo" == "cpu" ]]; then
        extra_args="--terms $terms"
    fi

    result=$(timeout 30s "$BINARY" --model "$model" --tipo "$tipo" --tasks "$tasks" --run "$run_n" $extra_args 2>/dev/null \
             | sed 's/ \[pi.*\]//' || echo "$model,$tipo,$tasks,$terms,$run_n,,timeout")

    echo "$result" | tee -a "$OUT"
}

echo ""
echo "=== I/O: comparación threads vs async ==="

for tasks in 10 100 1000 10000; do
    for run_n in 1 2 3; do
        run threads io "$tasks" 0 "$run_n"
        run async   io "$tasks" 0 "$run_n"
    done
done

echo ""
echo "=== CPU (Pi/Leibniz): comparación threads vs async ==="

for tasks in 1 2 4 8 16; do
    for terms in 10000 1000000 10000000; do
        for run_n in 1 2 3; do
            run threads cpu "$tasks" "$terms" "$run_n"
            run async   cpu "$tasks" "$terms" "$run_n"
        done
    done
done

echo ""
echo "=== Overhead de threads: muchas tareas, pocos términos ==="

for tasks in 100 500 1000 2000; do
    terms=100000
    for run_n in 1 2 3; do
        run threads cpu "$tasks" "$terms" "$run_n"
        run async   cpu "$tasks" "$terms" "$run_n"
    done
done

echo ""
echo "=== CPU: tasks altas para ver límite de threads ==="

for tasks in 10000 100000; do
    terms=1000000
    for run_n in 1 2 3; do
        run threads cpu "$tasks" "$terms" "$run_n" || true
        run async   cpu "$tasks" "$terms" "$run_n"
    done
done

echo ""
echo "Resultados guardados en $OUT"