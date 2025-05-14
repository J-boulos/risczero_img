#!/bin/bash
set -e

echo "Running Example 3"

cd receipts

echo "Running img transformation (one host per chunk) in parallel..."

../../host_img ../img_chunks/image_1.png grayscale --recursive 1 &
../../host_img ../img_chunks/image_2.png grayscale --recursive 2 &

wait

cd ..

echo "Running composition proof on parallel chunks processing and img reconstruction..."
../host_comp receipts --parallel

echo "Verifying proof..."
../verifier_comp composition_receipt.bin ../example1/img_grayscale.png --parallel