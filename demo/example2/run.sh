#!/bin/bash
set -e

echo "Running Example 2"

echo "Running proof of receipts composition..."
../host_comp receipts

echo "Verifying proof..."
../verifier_comp composition_receipt.bin image_2.png
