#!/bin/bash
set -e

echo "Running Example 1"

echo "Running img transformation..."
../host_img ../image.png grayscale

echo "Verifying img proof..."
../verifier_img receipt_grayscale.bin img_grayscale.png

echo "Running img transformation..."
../host_img img_grayscale.png flipv --recursive 1

echo "Running img transformation..."
../host_img image_1.png fliph --recursive 2

cp receipt_grayscale.bin ../example2/receipts/
cp receipt_1.bin ../example2/receipts/
cp receipt_2.bin ../example2/receipts/

cp image_2.png ../example2/