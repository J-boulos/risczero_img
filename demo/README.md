# Demo 
This directory contains a bash script that builds composition and imgTransformations .
It copies the binaries for running the host , as well as the verifier ,for both projects .

## host_img :
- expects : <image_path> <transformation1> [transformation2 ...] [--recursive <id>]

## verifier_img : 
- expects :  <receipt_file> <public_image>

## host_comp :
- expects : [--parallel ]

## verifier_comp : 
- expects :  <receipt_file> <public_image>

I also included the famous dice img that I've used since day one . ( I've resized it , to have acceptable running times )

Finally I wrote some bash scripts to demonstrate the different achievable worflows  

For a cleaner structure , I placed each one of them in a seperate subdirectory for easier output inspection.
It is recommended to run them in order since i'm using receipts from example 1 in example 2 and 3 to avoid rerunning entire worflows.

Here's what each one of them does : 

Example 1 : 

- Generates the proof for a grayscale transformation , verifies it . Then adds two more transformation in recursive mode .
- It then copies all three receipts into  example 2's receipt directory .

Example 2 : 

- Example 2 generates the composition proof , taking the receipts from it's receipts directory .
- Verifies the proof and compares the final obtained image to example 1's image 2 . They should match 

Example 3 : 

- Runs a grayscale transformation on a chunk of the dice image . Here two instances of the script are run in parallel , each one of them acting on a single chunk of the original image.
- Then the composition proof is generated , the individual chunk receipts are added as assumption and the image is reconstructed.
- The verifier verifies this proof then compares the reconstructed image to the public grayscale image which I took from example 1 's first run

