#!/bin/bash

for i in {1..100}; do
{
    start=$(date +%s%N)
    curl -s -o /dev/null "http://localhost:3000/pi/1000000"
    end=$(date +%s%N)

    elapsed_ns=$((end - start))
    elapsed_ms=$((elapsed_ns / 1000000))

    echo "Curl $i terminó en ${elapsed_ms} ms"
} &
done

wait
#esta prueba no me asegura concurrencia
